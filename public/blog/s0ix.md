## Background (S3 v. S0ix)

One of the main things still missing in FreeBSD for it to be usable on a modern laptop such as the AMD Framework laptops and the newer Intel ones is the ability to go to sleep.
In the past, this was done using something called ACPI S3.

S3 is one of the global sleep states that ACPI defines (other examples include S0 when in regular operation and S5 when the computer is fully turned off).
When you tell your machine to go into the S3 sleep state, the `acpi_EnterSleepState` function is called, which eventually tells your ACPI firmware to put your machine to sleep.

However, modern laptops have started ditching S3 and using something called S0ix instead.
With S0ix, the system stays in the S0 state, and the firmware only enters a low-power state (e.g. S0i3) when the CPUs are idle and some device power constraints are met, which the OS is responsible for ensuring.

A fair warning: this article delves into the sombre depths and tedium of ACPI, so it's not the most exciting read.

Also, I'm still currently figuring this out, so some of the information here might be incomplete or slightly inaccurate.
These are really mostly my personal, somewhat disjointed notes.

### Does my laptop use S3 or S0ix?

On FreeBSD, you can query the sleep states your machine supports by reading the `hw.acpi.supported_sleep_state` sysctl (`hw.acpi.suspend_state` gives you the sleep state used for suspend).
If you don't see `S3` in the list, your machine probably only supports S0ix.

To be sure that your machine indeed does support S0ix, you need to check the FADT flags, specifically `AcpiGbl_FADT.Flags & ACPI_FADT_LOW_POWER_S0`.

### What has already been done?

Ben Widawsky from Intel started work on this in 2018 with two patches, [D17675](https://reviews.freebsd.org/D17675) for suspend-to-idle support and [D17676](https://reviews.freebsd.org/D17676) for emulating S3 with S0ix.
This work was never finished, however.

## LPIT v. `_LPI`, residency counters, and AMD PMC 

The LPIT (Low Power Idle Table, defined in [this Intel spec](https://uefi.org/sites/default/files/resources/Intel_ACPI_Low_Power_S0_Idle.pdf)) describes the low-power idle states that the CPU supports.
These table entries also contain residency counters, which just tell you how long a CPU has spent in a particular low-power state (useful for debugging).

[It would seem](https://www.kernel.org/doc/html/v6.4/arm64/acpi_object_usage.html) as though LPIT has gone out of favour since ACPI 6.0.
It says as much for ARM.
It does seem like newer Intel devices still have the LPIT table but no `_LPI` objects (e.g. the [Dell XPS 15 9570](https://raw.githubusercontent.com/linuxhw/ACPI/master/Notebook/Dell/XPS/XPS%2015%209570/7298D35A1731)), whereas AMD laptops only have `_LPI` objects, which means both will have to be supported.

The ACPI spec now makes no mention of LPIT, [but it does look like LPI is its replacement](https://uefi.org/specs/ACPI/6.5/08_Processor_Configuration_and_Control.html#lpi-low-power-idle-states).
Unfortunately, they made residency counters for each of these states optional, and it so happens that they are missing on my AMD Framework laptop, which is a little annoying for debugging.

There may be a glimmer of hope though - Linux has an `amd-pmc` driver in `drivers/platform/x86/amd/pmc/pmc.c` ([pmc.c](https://elixir.bootlin.com/linux/v6.13-rc3/source/drivers/platform/x86/amd/pmc/pmc.c), initially added by this [patch](https://patchwork.kernel.org/project/platform-driver-x86/patch/20201105140531.2955555-1-Shyam-sundar.S-k@amd.com/)) which is able to read S0i3 entry and exit times, from which residency can be calculated.
I haven't yet found much documentation for this but I'd like to tackle this next.

## SPMC (System Power Management Controller) or PEP (Power Engine Plugin)

The SPMC or PEP - as far as I'm aware, these are used interchangeably - is the primary device used for interacting with the firmware for S0ix.
It uses ACPI ID `PNP0D80` ("Windows-compatible System Power Management Controller").
For this, I have written a new `acpi_spmc` driver for FreeBSD in [D48387](https://reviews.freebsd.org/D48387).

It is useful for two main things:

- Giving us the device power constraints (so-called "D-states") required for entering a given low-power idle state. We'll learn more about this later.
- For us to send notifications to the firmware denoting specific checkpoints, such as when the displays have been turned off or when we're ready to enter a low-power idle state.

This is done through DSMs (Device Specific Methods).

### DSMs (Device Specific Methods)

In ACPI-speak, a DSM (`_DSM` object) is a sort of special multiplexed method for executing, well, device-specific methods.
When you evaluate a `_DSM` object, you pass it a UUID as its first argument, a revision as its second, a function index as its third, and, finally, an optional package (really just a vector in ACPI-speak) of arguments as its fourth.
On FreeBSD, `acpi_EvaluateDSMTyped` is used to do this for you.

It seems like the original [Intel spec](https://uefi.org/sites/default/files/resources/Intel_ACPI_Low_Power_S0_Idle.pdf) linked above is not actually used in practice (UUID `c4eb40a0-6cd2-11e2-bcfd-0800200c9a66`), at least not on modern Intel or AMD platforms.
Instead, there's [Microsoft's](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/modern-standby-firmware-notifications) DSM UUID `11e00d56-ce64-47ce-837b-1f898f9aa461`, and thankfully is quite similar to the original DSM's, except with a couple extra "Modern Standby" functions and missing some others:

|Index|Description|Notes|
|--|--|--|
|0|Enumerate functions.||
|1|Get device constraints.|Only in the Intel spec.|
|2|Get crash dump device.|Only in the Intel spec.|
|3|Display off notification.||
|4|Display on notification.||
|5|Entry notification.||
|6|Exit notification.||
|7|"Modern Standby" entry notification.||
|8|"Modern Standby" exit notification.||

AMD seems to have their own DSM UUID `e3f32452-febc-43ce-9039-932122d37721` along with Microsoft's one, for which I haven't really been able to find any documentation.
This is what they look like:

|Index|Description|Notes|
|--|--|--|
|0|Enumerate functions.||
|1|Get device constraints.||
|2|Entry notification.||
|3|Exit notification.||
|4|Display off notification.||
|5|Display on notification.||

On AMD platforms, we must use the AMD UUID for getting device constraints, which makes sense as Microsoft's DSMs don't have this.
For some reason, though, the device constraints package returned by the AMD UUID follows a different format for which I couldn't find a spec anywhere 🙃

It seems like we need to use both the Microsoft and AMD UUIDs for the notifications (including the "Modern Standby" ones), though. We'll talk more about this later.

I don't know what exactly the situation is like on modern Intel platforms.

## Going to sleep 💤

Okay, so what does the process for going to sleep *actually* look like?
Broadly, we follow the following steps:

- Put all devices to sleep.
- Make sure there are no device power constraint violations.
- Send display off and sleep entry notifications.
- Mask any interrupts or GPE's that could wake the system up prematurely.
- Idle the CPU (suspend-to-idle).

### Putting devices to sleep

The first step is to put all the devices attached to the system to sleep themselves.
These devices are things like USB peripherals, the GPU, any NVMe drives, &c.
At minimum, to enter an LPI state, we must satisfy the device constraints gotten from the SPMC.
In practice though, if we're going to sleep, we might as well try to save as much power as possible and attempt to put all devices to sleep.

An ACPI device has <s>four</s> five-ish power states, known as D-states: D0 (fully on), D1, D2, D3hot (off but still powered), and D3cold (off and with power completely removed).
The distinction between D3hot and D3cold seems to be a relatively new one, and it's unclear which one "D3" refers to in the ACPI spec. See the [PR](https://github.com/acpica/acpica/pull/993) I opened on the ACPICA GitHub repo discussing this, and the (WIP) [D48384](https://reviews.freebsd.org/D48384) revision for adding D3cold support to FreeBSD.

Switching between these states is done through the `acpi_pwr_switch_consumer` function on FreeBSD (a "power consumer" is just a device).

To set a device's D-state, one must first get the power resources required for that D-state through the `_PRx` (where `x` is the target D-state) objects ([ACPI 7.3.8 - 7.3.11](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/07_Power_and_Performance_Mgmt/device-power-management-objects.html?highlight=_psc#pr0-power-resources-for-d0)) and ensure they are all turned on.
Conversely, the power resources for all higher-power states (i.e. lower-numbered `x`) must be turned off.
Finally, the `_PSx` object is evaluated to actually set the device to the desired D-state.

A device only supports D3cold if it lists explicit power resources for D3 through a `_PR3` object, in which case, keeping those power resources on transitions the device to D3hot and turning them off transitions it to D3cold.

There was an issue with turning these power resources off in FreeBSD, which I fixed in [D48385](https://reviews.freebsd.org/D48385).

### Checking for device power constraint violations 🚓

Before we intend to go to sleep, it is useful to check that we're not violating any of the device power constraints gotten from the SPMC.

For this, we need a way to get a device's current D-state.
I added an `acpi_pwr_get_consumer` function for doing this in [D48386](https://reviews.freebsd.org/D48386).

ACPI defines multiple ways of getting the D-state of a device.
The first and simplest is through the `_PSC` (power state current, [ACPI 7.3.6](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/07_Power_and_Performance_Mgmt/device-power-management-objects.html?highlight=_psc#psc-power-state-current)) control method, which simply spits out the device's D-state when evaluated.
`_PSC` isn't implemented for all devices, however:

> This control method is not required if the device state can be inferred by the Power Resource settings. This would be the case when the device does not require a _PS0, _PS1, _PS2, or _PS3 control method.

The "Power Resource settings" the spec mentions are our friends the `_PRx` objects.
From these, we can infer the D-state of a device is as follows:

- Go through each D-state from the lowest-numbered (D0) to the highest (D3).
- Evaluate the `_PRx` object for that D-state.
- For each power resource object returned, evaluate the `_STA` object ([ACPI 7.2.4](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/07_Power_and_Performance_Mgmt/declaring-a-power-resource-object.html?highlight=_sta#sta-power-resource-status)), which lets us know if it is on (0) or off (1).
- If all the power resources are on, we know that the device is in that D-state.
- If not, we move on to the next D-state. If we just checked the `_PR3` object and all the power resources are off, then we know the device supports and is in D3cold.

Then, it's just a simple matter of making sure the device's D-state is greater or equal to the one in the corresponding device power constraint package.

### Sending display off and sleep entry notifications 🖥

**TODO** This is easy. Talk about the hooks in [D48387](https://reviews.freebsd.org/D48387).

### Interrupts and GPEs 📣

**TODO** This is not as easy. Talk about the SCI, GPEs, the EC, and maybe a brief section on `_PSW` and `_DSW` (but these aren't the most important).

### Idling the CPU

CPU power states are known as C-states.
To put it to sleep, we need to set it to its maximum C-state and, if we did the previous setup right, the firmware will hopefully take care of the rest.

The [MWAIT](https://www.felixcloutier.com/x86/mwait) instruction can do this for us.
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

**TODO** Just talk about the exit notifications, there's nothing else really of interest to mention here that we haven't already covered.

## What about hibernation (S4)?

**TODO**

## What's next?

**TODO** Modern standby.
