## Background (S3 v. S0ix)

One of the main things still missing in FreeBSD for it to be usable on modern laptops is the ability to go to sleep.
In the past, this was done using something called ACPI S3, but vendors have slowly been phasing this out in favour of something else called S0ix.
FreeBSD does not support S0ix as of yet, leaving it without sleep support on these devices.

S3 is one of the global sleep states that ACPI defines (other examples include S0 when in regular operation and S5 when the computer is fully turned off).
When you tell your machine to go into the S3 sleep state, the `acpi_EnterSleepState` function is called, which eventually tells your ACPI firmware to put your machine to sleep.

With S0ix, the system instead stays in the S0 global state, and the firmware only enters a low-power state when the CPUs are idle and some device power constraints are met, which the OS is responsible for ensuring.
The **x** in S0i**x** denotes the specific low-power idle state the system, the deepest of which and our eventual goal is S0i**3**.

A fair warning: this article delves into the sombre depths and tedium of ACPI, so it's probably not the most exciting read.
But here's a picture of [Beastie](https://en.wikipedia.org/wiki/BSD_Daemon) snoozing to keep you company:

![Beastie sleeping](/public/blog/zzz.png)

I gave a presentation on this topic at [FOSDEM 2025](https://fosdem.org/2025/), which you can view [here](https://youtu.be/mBxj_EkAzV0).

### Does my laptop use S3 or S0ix? And what is s2idle?

On FreeBSD, you can query the sleep states your machine supports by reading the `hw.acpi.supported_sleep_state` sysctl (`hw.acpi.suspend_state` gives you the sleep state used for suspend).
If you don't see `S3` in the list, your machine probably only supports S0ix.

To be sure that your machine indeed does support S0ix, you need to check the FADT flags, specifically `AcpiGbl_FADT.Flags & ACPI_FADT_LOW_POWER_S0`.

Note that as of [D48734](https://reviews.freebsd.org/D48734), all ACPI machines will advertise `s2idle` as supported, which, although related to S0ix, does not imply that a given machine supports S0ix.

`s2idle` or "suspend-to-idle" is a "fake" sleep state which basically just means that you do all the usual setup to sleep your machine, except that you're just idling the CPU rather than actually entering a sleep state.
Theoretically, this works on any machine, but it doesn't save all that much power on its own.

Instead, if everything has been set up correctly, the firmware will enter one of the S0ix states and will hopefully end up in S0i3 at some point when the OS is in `s2idle`.

### What has already been done?

Ben Widawsky from Intel started work on this in 2018 with two patches, [D17675](https://reviews.freebsd.org/D17675) for suspend-to-idle support and [D17676](https://reviews.freebsd.org/D17676) for emulating S3 with S0ix.
This work was never finished, however.

## Debugging: LPIT v. `_LPI`, residency counters, and the AMD SMU 🐛

The LPIT (Low Power Idle Table, defined in [this Intel spec](https://uefi.org/sites/default/files/resources/Intel_ACPI_Low_Power_S0_Idle.pdf)) describes the low-power idle states that the CPU supports.
These table entries also contain residency counters, which just tell you how long a CPU has spent in a particular low-power state, which is obviously useful for debugging.

[It would seem](https://www.kernel.org/doc/html/v6.4/arm64/acpi_object_usage.html) as though LPIT has gone out of favour since ACPI 6.0.
It says as much for ARM.
It does seem like newer Intel devices still have the LPIT table but no `_LPI` objects (e.g. the [Dell XPS 15 9570](https://raw.githubusercontent.com/linuxhw/ACPI/master/Notebook/Dell/XPS/XPS%2015%209570/7298D35A1731)), whereas AMD laptops only have `_LPI` objects, which means both will have to be supported.

The ACPI spec now makes no mention of LPIT, [but it does look like LPI is its replacement](https://uefi.org/specs/ACPI/6.5/08_Processor_Configuration_and_Control.html#lpi-low-power-idle-states).
Unfortunately, they made residency counters for each of these states optional, and it so happens that they are missing on my AMD Framework laptop.

Luckily, AMD chips have an SMU (System Management Unit, which you'll also see referred to as "MP1") core on-die which we can ask for residency information.
This is a small [LatticeMico32](https://en.wikipedia.org/wiki/LatticeMico32) microprocessor that runs power management firmware (PMFW) which also serves to actually decide whether or not we enter S0i3 and power goes to the CPU.

Initial support for this is added with an `amdsmu` driver in [D48683](https://reviews.freebsd.org/D48683), and residency counters are exposed as `sysctl`s in [D48714](https://reviews.freebsd.org/D48714).
We'll revisit the SMU later when we talk about the sleep process.
Rudolf Marek has an interesting CCC talk about "[Matroshka processors](https://media.ccc.de/v/31c3_-_6103_-_en_-_saal_2_-_201412272145_-_amd_x86_smu_firmware_analysis_-_rudolf_marek)" as he calls them.

![Dieshot of Matroshka processor on an AMD CPU.](/public/blog/s0ix-dieshot.webp)
*Credit to [@Locuza\_](https://twitter.com/Locuza_/status/1325534004855058432) on Twitter.*

One last thing I'd like to touch on regarding debugging on AMD is the [amd_s2idle.py](https://git.kernel.org/pub/scm/linux/kernel/git/superm1/amd-debug-tools.git/tree/amd_s2idle.py) script on Linux, which is very helpful in debugging the myriad reasons why a laptop may not be entering the deep sleep S0i3 state.
I'd like to write something similar for FreeBSD at some point once S0i3 is actually working.

## SPMC (System Power Management Controller) or PEP (Power Engine Plugin)

The SPMC or PEP - as far as I'm aware, these can be used interchangeably - is the primary device used for interacting with the firmware for S0ix.
It uses ACPI ID `PNP0D80` ("Windows-compatible System Power Management Controller").
For this, I have written a new `acpi_spmc` driver for FreeBSD in [D48387](https://reviews.freebsd.org/D48387).

It is useful for two main things:

- Giving us the device power constraints (so-called "D-states") required for entering a given low-power idle state. We'll learn more about this later.
- For us to send notifications to the firmware denoting specific checkpoints, such as when the displays have been turned off or when we're ready to enter a low-power idle state.

This is done through DSMs (Device Specific Methods).

### DSMs (Device Specific Methods)

In ACPI-speak, a DSM (`_DSM` object) is a sort of special multiplexed method for executing, well, device-specific methods.
When you evaluate a `_DSM` object, you pass it a vendor-specific UUID as its first argument, a revision as its second, a function index as its third, and, finally, an optional package (== a vector in ACPI-speak) of arguments as its fourth.
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

AMD seems to have their own DSM UUID `e3f32452-febc-43ce-9039-932122d37721` along with Microsoft's one, for which I haven't really been able to find any documentation outside of the Linux implementation.
This is what they look like:

|Index|Description|Notes|
|--|--|--|
|0|Enumerate functions.||
|1|Get device constraints.||
|2|Entry notification.|On Framework laptops, this slowly fades the power button led in and out.|
|3|Exit notification.||
|4|Display off notification.||
|5|Display on notification.||

A simplified pseudo-code example of calling e.g. the "get device constraints" function on AMD looks like this:

```c
Arg0 = "e3f32452-febc-43ce-9039-932122d37721" // AMD's SPMC DSM UUID.
Arg1 = 0 // Revision zero.
Arg2 = 1 // "Get device constraints" function ID.
Arg3 = Package() // No arguments needed.
call_dsm(spmc_device, Arg0, Arg1, Arg2, Arg3)
```

On AMD platforms, we must use the AMD UUID for getting device constraints, which makes sense as Microsoft's DSMs don't have this.
For some reason, though, the device constraints package returned by the AMD UUID follows a different format for which I couldn't find a spec anywhere 🙃

It seems like we need to use both the Microsoft and AMD UUIDs for the notifications (including the "Modern Standby" ones), though.
We'll talk more about this later.

I don't know what exactly the situation is like on modern Intel platforms.

## Going to sleep 💤

Okay, so what does the process for going to sleep *actually* look like?
Broadly, we follow the following steps:

- Put all devices to sleep.
- Make sure there are no device power constraint violations.
- Mask any interrupts or GPE's that could wake the system up prematurely.
- Send display off and sleep entry notifications.
- Idle the CPU (suspend-to-idle a.k.a. s2idle).

### Putting devices to sleep

The first step is to put all the devices attached to the system to sleep themselves.
These devices are things like USB peripherals, the GPU, any NVMe drives, &c.
At minimum, to enter an LPI state, we must satisfy the device constraints gotten from the SPMC.
In practice though, if we're going to sleep, we might as well try to save as much power as possible and attempt to put all devices to sleep.

An ACPI device has <s>four</s> five-ish power states, known as D-states: D0 (fully on), D1, D2, D3hot (off but still powered), and D3cold (off and with power completely removed).
The distinction between D3hot and D3cold seems to be a relatively new one, and it's unclear which one "D3" refers to in the ACPI spec.
See this [PR](https://github.com/acpica/acpica/pull/993) I opened on the ACPICA GitHub repo discussing this, and the (WIP) [D48384](https://reviews.freebsd.org/D48384) revision for adding D3cold support to FreeBSD.

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

There isn't much to talk about here.
We just need to call the display off and sleep entry DSM functions on the SPMC.
This is done in [D48735](https://reviews.freebsd.org/D48735).

### Interrupts and GPEs 📣

This is a little tricky.
In the next step we'll stop the CPUs in such a way that they can only be woken up by an interrupt.
Lots of things could interrupt the CPU, so we'd like to mask out all interrupts which are not related to actually waking the system before going to sleep.

ACPI interrupts are done through system control interrupts or SCIs.
The interrupt number for SCIs is gotten from `AcpiGbl_FADT.SciInterrupt`, and is usually interrupt number 9.
So we first mask out all the interrupts except for the SCI:

```c
register_t rflags = intr_disable(); // Save previous IF, run x86 cli.
intr_suspend(); // Stop interrupts from all PICs.
intr_enable_src(AcpiGbl_FADT.SciInterrupt); // Enable SCIs (interrupt 9).

// Sleep...

intr_resume(false); // Resume interrupts on all PICs.
intr_restore(rflags); // Restore IF.
```

When an SCI is triggered, the OS is supposed to read a special register to figure out what GPE number (general purpose event) caused this interrupt.
[This blog post](https://queue.acm.org/blogposting.cfm?id=18977) explains this in further detail.

Not all of these GPEs should cause an interrupt though.
For example, my Framework's embedded controller sends me a GPE once a second to update me on the battery status.
Obviously, we don't want this to wake the system up from sleep.

So ACPI has a mechanism for masking out GPEs coming from specific devices, namely through the `_DSW` (or `_PSW` for older devices, see [ACPI 7.3.1](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/07_Power_and_Performance_Mgmt/device-power-management-objects.html#dsw-device-sleep-wake)) method.

The issue is that lots of laptops will put important wake devices under the same GPE number as noisy devices such as the battery mentioned previously.
Here is some simplified ASL code showing that the lid and battery status change GPEs are under the same GPE number:

```c
Device (EC0) { // The embedded controller.
	Name (_GPE, 0x0B) // GPE number.
	Device (LID0) { /* ... */ }
	Method (_Q01, 0, NotSerialized) { // GPE for lid device.
		P80H = 0x01
		Notify (LID0, 0x80) // Status change.
	}
	Method (_Q3C, 0, NotSerialized) { // VERY noisy GPE for battery (1 GPE/s).
		P80H = 0x3C
		Notify (BAT1, 0x80) // Status change.
	}
}
Device (BAT1) { /* ... */ }
```

This means that, if we mask out the battery GPE, we also mask out the lid GPE, which is no good.
This is mitigated somewhat by entering suspend on my machine, but the battery will still emit a GPE, just a bit slower, at once a minute.
Hopefully once we get to S0i3 the firmware will know to shut up with the useless GPEs, and 1 minute is more than enough time to enter S0i3.

We might still not be 100% safe from spurious wakeups, so the solution that Linux uses and the solution that I'll implement in FreeBSD soon is to have an "s2idle loop".
When the CPU is woken up from idle, the OS will check what the last wakeup even was, and if it doesn't agree that it should have been woken up, it will immediately idle the CPUs again.

Checking the last wakeup event means that we can expose it as a `sysctl` for free, which is great for a user wanting to debug for what reason their laptop woke up in the middle of the night.

### Idling the CPU (MWAIT)

The final step the OS has to take is to idle the CPUs.

To go to sleep, we need to set them to their maximum C-state (CPU power state) and, if we did the previous setup right, the firmware will hopefully take care of the rest.

The [MWAIT](https://www.felixcloutier.com/x86/mwait) instruction can do this for us.
It's an x86 instruction that's usually used in conjunction with MONITOR to enter an "implementation-dependent optimized state" and wait until a specific memory range is written to.

If `CPUID.05H:ECX[bit 0]` is set, it can also be used for power management.
Specifically, `eax` can be set to contain hints to MWAIT and `ecx` (extension) can be set to contain the C-state that the processor should enter.

For our purposes, we can set the lowest bit of `eax` to 1 to allow for interrupts to break out of MWAIT (i.e. wake the CPU up).
Thanks to this, we can forgo the need to set up a memory range to monitor.

Bits 7 to 4 of `ecx` are used to specify the target C-state to enter (we can ignore the lowest 4 bits which are for "sub C-states"):

```x86asm
mov eax, 0x30 ; C-state C4 (MWAIT_C4).
mov ecx, 1    ; Break on interrupt, like hlt (MWAIT_INTRBREAK).
mwait
```

FreeBSD's `cpu_idle()` function will use MWAIT when it's available.

## Vendor-specific complications: AMD

On AMD, there are a few extra thing we need to do for the PMFW running on the SMU to actually enter S0i3.
As mentioned earlier, these conditions can be checked with the [amd_s2idle.py](https://git.kernel.org/pub/scm/linux/kernel/git/superm1/amd-debug-tools.git/tree/amd_s2idle.py) script on Linux:

- We need to send the SMU a special message hinting to it that we're entering and exiting sleep ([D48721](https://reviews.freebsd.org/D48721)).
- We need to write a USB4 HCM (host connection manager) to tell the USB4 controller to go to sleep, as it starts off in a high-power state. The current state of USB4 on FreeBSD is compiled on this [FreeBSD wiki page](https://wiki.freebsd.org/MohammadNoureldin/FreeBSDUSB4TBT3Support). I have started work on this and will probably write up a separate blog post on this.
- All GPIO interrupts must be serviced. Preliminary support for AMD GPIO on FreeBSD is provided by the `amdgpio` driver which was added by [D16865](https://reviews.freebsd.org/D16865). The remaining functionality to be added can be found on Linux in [pinctrl-amd.c](https://elixir.bootlin.com/linux/v6.13.2/source/drivers/pinctrl/pinctrl-amd.c).
- The `amdgpu` driver (DRM) must be loaded and told to go to sleep. I have not looked into what exactly it is doing when going to sleep, but I image it is just telling the graphics cores to go to sleep. Either way FreeBSD already supports this and I presume the driver is already doing what's needed.

If any of these conditions are unmet, PMFW will refuse to transition to S0i3, and you will get negligible power savings.

I have built up a [minimal kernel config](/public/blog/s0i3-linux-config) starting from `make tinyconfig` with just enough enabled to actually enter S0i3 for debugging.
A special thanks to Mario Limonciello ([superm1](https://github.com/superm1)) from AMD for helping me figure this all out.

## What about hibernation (S4)?

Hibernation actually has little to do with S0ix.
Instead of suspending-to-RAM (i.e. keeping it active while the rest of the system is powered off), hibernation swaps all pages in RAM to disk and then completely powers off the system.
When you want to exit out of hibernation, the bootloader reads back the image from disk to memory to restore the system to its previous state.

S4 saves more power than S0ix (actually, in S4 the system consumes no power at all), but the downside is that it of course takes way longer to enter and exit.

FreeBSD actually has support for S4BIOS, which was a transitional way of doing hibernation where the BIOS does most of the heavy-lifting intended to ease the adoption of S4.
This doesn't exist on modern laptops, but you can check if yours has it through `hw.acpi.s4bios`.

There is also hybrid suspend, in which the system enters an S3 or S0ix state but still writes the hibernation image to disk anyway.
This way, you get the advantages of fast wake times but you don't risk corrupting your filesystem if the battery reaches a critical level and your system suddenly loses power.

## What's next? 🔮

Here's a grab-bag of things that still need to be done or would be nice to have:

- Actual S0i3 support, of course 😁
- Testing on Intel. At the moment, I don't have access to a modern Intel laptop, though I'm eyeing one of the Ultra Series 1 laptops. In the meantime, do test the patches on your Intel laptop if you have one and send me logs! [obiwac@freebsd.org](mailto:obiwac@freebsd.org)
- [Powertop](https://en.wikipedia.org/wiki/PowerTOP) equivalent. This would be a good [GSoC](https://summerofcode.withgoogle.com) project, so I've added it to the [SummerOfCodeIdeas](https://wiki.freebsd.org/SummerOfCodeIdeas#Power_profiling_tool) wiki page.
- [RTC alarm](https://en.wikipedia.org/wiki/Real-time_clock_alarm) wake. This wakes the system from sleep after a given amount of time. This is used by `amd_s2idle.py` to sleep for a consistent amount of time during each test run. It can also be used to hibernate the system if it has been suspended to RAM for over e.g. 5 minutes.
- Wake on low battery level. Through the `_BLT` ACPI control method on battery devices we can set the battery warning, low, and wake levels. Perhaps this also means we can ask firmware to emit a wake interrupt to wake the system when the battery reaches a given threshold. This would be useful to wake the device to notify the user that the battery is low, or gracefully unmount all disks and shut down the system if the battery is critically low.
- Idleness determination. This is a concept in [IOKit on macOS](https://developer.apple.com/library/archive/documentation/DeviceDrivers/Conceptual/IOKitFundamentals/PowerMgmt/PowerMgmt.html). If a device is determined to be idle, power is removed from it. If all of a bus device's children are idle, power is removed from the bus device too.
- Enable S0ix on the FreeBSD NVIDIA driver. This is being worked on by NVIDIA.
