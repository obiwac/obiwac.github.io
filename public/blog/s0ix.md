## Background (S3 v. S0ix)

One of the main things still missing in FreeBSD for it to be usable on a modern laptop such as the AMD Framework laptops and the newer Intel ones is the ability to go to sleep.
In the past, this was done using something called ACPI S3.

S3 is one of the sleep states that ACPI defines (other examples include S0 when in regular operation and S5 when the computer is fully turned off).
When you tell your machine to go into the S3 sleep state, the `acpi_EnterSleepState` function is called, which eventually tells your ACPI firmware to put your machine to sleep.
With S3, your firmware is thus responsible for turning off the CPU and other devices when explicitly told to do so.

However, modern laptops have started ditching S3 and using something called S0ix instead.
With S0ix, the operating system is the one responsible for figuring out what devices need to be put to sleep before the CPU can be turned off, and the firmware will automatically turn off the CPU only once all the requirements are met.

A fair warning: this article delves into the sombre depths and tedium of ACPI, so it's not the most exciting read.

Also I'm still currently figuring this out, so some of the information here might be incomplete or flat wrong.
These are really mostly personal notes.

### Does my laptop use S3 or S0ix?

On FreeBSD, you can query the sleep states your machine supports by reading the `hw.acpi.supported_sleep_state` sysctl (`hw.acpi.suspend_state` gives you the sleep state used for suspend).
If you don't see `S3` in the list, your machine probably only supports S0ix.

To be sure that your machine indeed does support S0ix, you need to check the FADT flags, specifically `AcpiGbl_FADT.Flags & ACPI_FADT_LOW_POWER_S0`.

## What has already been done?

Ben Widawsky from Intel started work on this in 2018 with two patches, [D17675](https://reviews.freebsd.org/D17675) and [D17676](https://reviews.freebsd.org/D17676) (Emulated S3 with s0ix).
The first patch just implements suspending to idle, which means that the CPU is simply idled when suspending instead of being fully put to sleep.
This work was never finished, however.

## The LPIT v. the `_LPI` object

The LPIT (Low Power Idle Table, defined in [this Intel spec](https://uefi.org/sites/default/files/resources/Intel_ACPI_Low_Power_S0_Idle.pdf)) is a table that describes the low-power idle states that the CPU supports.
These table entries also contain residency counters, which just tell you how long a CPU has spent in a particular low-power state (useful for debugging).

[It would seem](https://www.kernel.org/doc/html/v6.4/arm64/acpi_object_usage.html) as though LPIT has gone out of favour since ACPI 6.0.
It says as much for ARM.
I haven't found any further information on this for AMD, but it does seem like newer Intel devices still have the LPIT table (e.g. the [Dell XPS 15 9570](https://raw.githubusercontent.com/linuxhw/ACPI/master/Notebook/Dell/XPS/XPS%2015%209570/7298D35A1731)).

The ACPI spec now makes no mention of LPIT, [but it does look like LPI is its replacement](https://uefi.org/specs/ACPI/6.5/08_Processor_Configuration_and_Control.html#lpi-low-power-idle-states).
Unfortunately, they made residency counters for each of these states optional, and it so happens that they are missing on my AMD Framework laptop, which is a little annoying for debugging.

I don't yet know if there are machines which have the LPIT but no `_LPI` object, but if there are then both will have to be implemented.
For now I'm just looking at reading the `_LPI` object.

## DSM's

In ACPI-speak, a `_DSM` (Device Specific Method) is just a command you can send to an ACPI device (with `acpi_EvaluateDSMTyped`).
Sets of these DSM's are identified by a UUID (which is queried with `acpi_DSMQuery`).

It seems like, in practice, the original Intel spec linked above is not actually used (with UUID `c4eb40a0-6cd2-11e2-bcfd-0800200c9a66`), at least on modern Intel or AMD platforms.
Instead, there's DSM UUID `11e00d56-ce64-47ce-837b-1f898f9aa461`, which is documented [here](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/modern-standby-firmware-notifications), and thankfully seems quite similar to the original DSM's, except with a couple extra "Modern Standby" functions:

|Index|Description|
|--|--|
|0|Enumerate functions.|
|1|Get device constraints (only in the Intel spec).|
|2|Get crash dump device (only in the Intel spec).|
|3|Display off notification.|
|4|Display on notification.|
|5|Entry notification.|
|6|Exit notification.|
|7|"Modern Standby" entry notification.|
|8|"Modern Standby" exit notification.|

I will detail what each of these DSM's are for in a bit.

AMD seems to have their own DSM set with UUID `e3f32452-febc-43ce-9039-932122d37721`, to which the only reference I've been able to find is [this post](https://lists.freedesktop.org/archives/amd-gfx/2020-January/044906.html) on the Freedesktop mailing list.
This is what they look like:

|Index|Description|
|--|--|
|0|Enumerate functions.|
|1|Get device constraints.|
|2|Entry notification.|
|3|Exit notification.|
|4|Display off notification.|
|5|Display on notification.|

**TODO** I'm unsure of if I can just ignore this and use the Microsoft DSM's, as my machine reports both UUID's.

## Architecture

I've created a new `acpi_lps0` driver, which looks for the `PNP0D80` ACPI ID ("Windows-compatible System Power Management Controller"), which is the device on which the above DSM's are defined.

**TODO** Should I rename this to `acpi_spmc` like Ben's patches? I only named it `acpi_lps0` because I didn't know what SPMC stood for at the time.

## Going to sleep

Here are the detailed steps for going to sleep with S0ix.

### Getting device constraints

The first step when trying to enter a low-power state is to get all the device constraints required for entering that state in the first place.
The "Get device constraints" DSM is used for this, and the output is parsed (for some reason, the AMD DSM's result package follows a different format for which I couldn't find a spec anywhere 🙃).

The output contains a list of all the ACPI devices on the system along with the minimum D-states required for the CPU to go to sleep.
Higher D-state correspond to deeper sleep states, D0 means "fully on", and D3 means "fully off".

An `acpi_pwr_switch_consumer` function already exists to set the D-state of a device, but FreeBSD doesn't currently have an equivalent `acpi_pwr_get_consumer` function, which is necessary to know which are the constraining devices exactly.

ACPI defines multiple ways of getting the D-state of a device.
The first and simplest is through the `_PSC` (power state current, [ACPI 7.3.6](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/07_Power_and_Performance_Mgmt/device-power-management-objects.html?highlight=_psc#psc-power-state-current)) control method, which simply spits out the device's D-state when evaluated.
`_PSC` isn't implemented for all devices, however:

> This control method is not required if the device state can be inferred by the Power Resource settings. This would be the case when the device does not require a _PS0, _PS1, _PS2, or _PS3 control method.

The "Power Resource settings" the spec mentions are the `_PR0`, `_PR1`, `_PR2`, and `_PR3` objects ([ACPI 7.3.8 - 7.3.11](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/07_Power_and_Performance_Mgmt/device-power-management-objects.html?highlight=_psc#pr0-power-resources-for-d0)), which evaluate to the power requirements (i.e. a list of power resource objects) for the device to enter the respective D-state.

From this, the process of inferring the D-state of the device is as follows:

- Go through each D-state from the lowest (D0) to the highest (D3).
- Evaluate the `_PRx` object for that D-state.
- For each power resource object returned, evaluate the `_STA` object ([ACPI 7.2.4](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/07_Power_and_Performance_Mgmt/declaring-a-power-resource-object.html?highlight=_sta#sta-power-resource-status)), which lets us know if it is on (0) or off (1).
- If all the power resources are on, we know that the device is in that D-state.
- If not, we move on to the next D-state.

I've implemented this through the `acpi_pwr_infer_state` function, which is called as a fallback to `_PSC` in `acpi_pwr_get_consumer`.

### Putting those devices to sleep

Each device which is constraining the CPU from going to sleep (i.e. their D-state is lower than the minimum D-state required) needs to be put to sleep.

I think this is done by calling the `acpi_pwr_switch_consumer` function.

**TODO** Actually implement this.
I'm going to need to work on a bit more infrastructure and understand the sleep process in FreeBSD before I can do this.

Now, the system should be ready to go to sleep.

### Sending display off and entry notifications

**TODO** This is easy.

### Putting the CPU to sleep

CPU power states are known as C-states.
To put it to sleep, we need to set it to its maximum C-state and, if we did the previous setup right, the hardware will hopefully take care of the rest.

[MWAIT](https://www.felixcloutier.com/x86/mwait) can do this for us.
It's an x86 instruction that's usually used in conjunction with MONITOR to enter an "implementation-dependent optimized state" and wait until a specific memory range is written to.

If `CPUID.05H:ECX[bit 0]` is set, it can also be used for power management.
Specifically, `eax` can be set to contain hints to MWAIT and `ecx` (extension) can be set to contain the C-state that the processor should enter.

For our purposes, we can set the lowest bit of `eax` to 1 to allow for interrupts to break out of MWAIT (i.e. wake the CPU up).
Thanks to this, we can forgo the need to set up a memory range to monitor.

Bits 7 to 4 of `ecx` are used to specify the target C-state to enter (we can ignore the lowest 4 bits which are for "sub C-states").

All in all, this is what that looks like on FreeBSD:

```c
#include <machine/cpufunc.h>

cpu_mwait(MWAIT_INTRBREAK, MWAIT_C3); // TODO What's the maximum C-state?
```

## Waking up from sleep

**TODO**

## What about hibernation (S4)?

S4 is really just S3 with the extra step of writing the contents of memory to disk before fully powering off.
This essentially gives you the power savings of S5 while still having a relatively small wake latency.

## What's next?

**TODO** Modern standby.
