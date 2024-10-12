## Introduction

Biometric authentication is a faster, more secure, and unarguably cooler method of authentication than plain old passwords.
This article goes through the process of setting up a fingerprint scanner on FreeBSD and using it for biometric authentication.

I have only tested this on the AMD Framework 13 (as that's the only laptop I own which has a fingerprint scanner), so YMMV on other laptops, but in theory any scanner supported under Linux* (and that's most) should Just Work™ on FreeBSD following this guide.

*Except for ELAN scanners.
More on that later.

## How everything fits together

Fingerprint scanner support is provided by the [`libfprint`](https://fprint.freedesktop.org/) library.
It provides an API for fingerprint scanning devices, as well as the userspace drivers for a range of different fingerprint scanners.

Most scanners are connected via USB, even if they're internal, so all that's really needed from the kernel is for it to provide a USB interface to the device though a `libusb` implementation, and the `libfprint` userspace drivers sit on top of that.

This is where we hit our first snag on FreeBSD, but I'll get back to that.

A separate `fprintd` daemon is used to manage the fingerprint scanner (including a D-Bus interface and management commands), and also provides the `pam_fprintd` authentication module for PAM.

In the past, you had the now-obsolete `pam_fprint` module which didn't need `fprintd` or D-Bus.
OpenBSD still has something similar with [`sysutils/login_fingerprint`](https://openports.pl/path/sysutils/login_fingerprint), but unfortunately, at the moment, you're stuck with `fprintd` and D-Bus on FreeBSD.

Now that you're up to speed with the general architecture, let's get to actually getting things to work!

## Updated port

Currently, the `security/libfprint` port in FreeBSD is very outdated, and doesn't support the 2<sup>nd</sup> version of the API or any new fingerprint scanners (including the Framework ones).
I'm working on a new port (new versions of `libfprint` use Meson instead of GNU configure and include what was previously in the also now-obsolete `security/fprint_demo`), but until that's done you'll have to build it yourself.

The main issue you'll encounter doing so is the build script not being able to find the `udev` library, needed for fingerprint scanners connected via SPI (i.e. those from ELAN).
I believe it is named something different on FreeBSD than it is on Linux, so a patch probably needs to be made against `meson.build`.
For now, you can just disable it with `-Dudev_rules=disabled -Dudev_hwdb=disabled`.

In all, this is what you need to do to build and install `libfprint`:

```sh
git clone "https://gitlab.freedesktop.org/libfprint/libfprint.git"
cd libfprint
meson setup build -Dudev_rules=disabled -Dudev_hwdb=disabled
ninja -Cbuild install
```

To test things out, there are some example programs you can run in the `build/examples` directory buuuuut...

## `libusb_get_parent`

...the next thing you'll run into is that `libfprint` won't detect any scanners.

This is because it uses the `g_usb_device_get_parent` function from `devel/libgusb`, which always returns `NULL` on FreeBSD, because it relies on `libusb_get_parent` from the system's `libusb` implementation, which FreeBSD doesn't implement yet (see [PR224454](https://bugs.freebsd.org/bugzilla/show_bug.cgi?id=224454)).

I've implemented this in [D46992](https://reviews.freebsd.org/D46992), which is currently awaiting review.
You'll have to apply this patch to your kernel and rebuild `libgusb` for `libfprint` to be able to detect your fingerprint scanner correctly.

## Updating firmware (for Framework laptops)

On certain revisions of Framework laptops, specifically those who's fingerprint scanner is on firmware version 01000320, `libfprint` will still refuse to work, and will tell you to update your firmware.

This is a bit annoying and tricky to do.

On Linux, this is done using `fwupd`, but this has not yet been fully ported to FreeBSD (though work seems to be well underway, see [D29332](https://reviews.freebsd.org/D29332)).

In the meantime, you can just use a Linux live ISO. Do make sure `fwupd` is up to date though as support for the AMD Framework's fingerprint scanner was only added in newer versions (see [fwupd#3637](https://github.com/fwupd/fwupd/discussions/3637)).
Then, you can just follow the instructions at [Updating Fingerprint Reader Firmware on Linux for all Framework Laptops | Framework](https://knowledgebase.frame.work/en_us/updating-fingerprint-reader-firmware-on-linux-for-13th-gen-and-amd-ryzen-7040-series-laptops-HJrvxv_za).

If it says something about a transfer timing out, that's alright, it's still transferring in the background.
Just wait a couple minutes and then reboot, and it should all work fine.

## Installing `fprintd`

As mentioned earlier, you need to build and install `fprintd`.
It's a similar story to `libfprint`: the `security/fprintd` port is outdated on FreeBSD, so you need to build it yourself.
It's also a Meson project, so the process is pretty simple, you just need to pass `-Dlibsystemd=basu` to the setup command as FreeBSD doesn't have systemd:

```sh
git clone "https://gitlab.freedesktop.org/libfprint/fprintd.git"
cd fprintd
meson setup build -Dlibsystemd=basu
ninja -Cbuild install
```

Then, we can create a service for `fprintd` in `/usr/local/etc/rc.d/fprintd`:

```sh
#!/bin/sh

# PROVIDE: fprintd
# REQUIRE: DAEMON dbus
# BEFORE: LOGIN
#
# Add the following lines to /etc/rc.conf to enable fprintd:
#
# fprintd_enable="YES"
#

. /etc/rc.subr

name=fprintd
rcvar=${name}_enable

: ${seatd_enable="NO"}

command="/usr/sbin/daemon"
procname="/usr/local/libexec/${name}"
pidfile="/var/run/${name}.pid"
command_args="-s notice -T ${name} -p ${pidfile} ${procname} -t"

load_rc_config ${name}
run_rc_command "$1"
```

And start it with:

```sh
service fprintd start
```

This is all assuming you've already set up D-Bus and the `dbus` service is running.

You can naturally set it to start automatically in your `/etc/rc.conf` by adding the `fprintd_enable="YES"` line, and that's all.

Phew 😮‍💨

## Enrolling and managing fingerprints

`fprintd` provides the `fprintd-enroll` command (to be run as root!) to enroll fingerprints:

```sh
fprintd-enroll <username> [-f finger]
```

## PAM configuration and login

Finally, you need to add the following to the PAM configuration file for the `system` service (`/etc/pam.d/system`):

```pam
auth		sufficient	pam_unix.so	no_warn try_first_pass nullok
auth		sufficient	/usr/local/lib/security/pam_fprintd.so
```

If you'd like a MFA setup, you can also set both authentication factors to be `required` or `requisite`, rather than sufficient.
See the [`pam.conf(5)`](https://man.freebsd.org/cgi/man.cgi?query=pam.conf) manpage for details.

This should now allow you to log in using your fingerprint!

## `doas` configuration

You can use the same policy for the `doas` PAM service as for the `system` service by creating the `/usr/local/etc/pam.d/doas` file:

```pam
auth include system
```

That'll work, but it'll continuously prompt you for your password, even if like 1 ms elapsed since you last authenticated.
Vanilla `doas` only supports the `persist` option in its config on OpenBSD, not on FreeBSD.

You can install the `security/opendoas` port which does let you use the `persist` option on FreeBSD.
Do be advised that this _might_ not be as secure as `persist` is on OpenBSD, however.
The only issue is that the timeout is hardcoded to 5 minutes, which is a tad long.
You can just `sed -i '' 's/5 \* 60/10/g'` files with occurrences of `5 * 60` to set the timeout to 10 seconds instead.

## Polkit configuration

You're probably going to want user applications to have permission to verify fingerprints without needing to be root.
You can do this by creating a new Polkit rule in something like `/usr/local/etc/polkit-1/rules.d/10-fprintd.rules`:

```js
polkit.addRule(function(action, subject) {
    if (action.id === "net.reactivated.fprint.device.verify") {
        return polkit.Result.YES
    }
})
```

To apply these changes, just restart the `dbus` service:

```sh
service dbus restart
```

## `swaylock`

`swaylock` is a popular screen locker for `wlroots` Wayland compositors.
It doesn't support `fprintd` by default, however.
Fortunately, there is a [`swaylock-fprintd`](https://github.com/SL-RU/swaylock-fprintd) fork of `swaylock` which does.

This port is slightly broken on FreeBSD.
In `fingerprint/meson.build`, you need to change the `/usr/share/dbus-1` prefixes to `/usr/local/share/dbus-1`.
Then, in the `/usr/local/etc/pam.d/swaylock` PAM service file it installs, you need to include the `unix-selfauth` service instead of `login`.

Finally, you can run `swaylock` with `--fingerprint`, and it should work fine (there's no need to add `fprintd` in its PAM service file):

```sh
swaylock --fingerprint
```

## Conclusion

HTH! 👋

I will keep this article up to date as I work on the new `libfprint` and `fprintd` ports.
