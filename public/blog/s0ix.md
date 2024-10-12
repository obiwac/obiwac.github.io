# Modern standby on FreeBSD

## Background (S3 v. S0ix)

One of the main things still missing in FreeBSD for it to be usable on a modern laptop such as the AMD Framework laptops and the newer Intel ones is the ability to go to sleep.
In the past, this was done using something called ACPI S3.

S3 is one of the sleep states that ACPI defines (other examples include S0 when in regular operation and S5 when the computer is fully turned off).
When you told your machine to go into the S3 sleep state, your CPU would go to sleep, and a small

## What about hibernation (S4)?

S4 is really just S3 with the extra step of writing the contents of memory to disk before fully powering off.
This essentially gives you the power savings of S5 while still having a relatively small wake latency.
