## Introduction

Traditionally, when dualbooting a system, you have to partition your disk into a partition for each operating system.
This can be inconvenient as you have to decide how much space to allocate to each OS installation upfront lest you spend time resizing them later, and for certain filesystems this is essentially impossible to do in-place (e.g. ZFS).

It would be nice if we could instead have a single ZFS pool which has a dataset for each OS.

In this guide, I'll be going over how to run both [Arch Linux](https://archlinux.org/) and [FreeBSD](https://www.freebsd.org/) on the same ZFS pool, assuming you already have a working FreeBSD installation and have a basic understanding of Linux & FreeBSD.
The same idea can be extended to any number of other Linux distributions or FreeBSD versions (though if you plan on only dualbooting multiple FreeBSD installations, you may want to look into [boot environment](https://wiki.freebsd.org/BootEnvironments) instead).

<iframe src="https://www.shadertoy.com/embed/wlc3zr?gui=false&paused=false&muted=true" height="180" frameborder="0"></iframe>

## Bootstrapping Arch ⛩️

Download the latest Arch Linux image from <https://archlinux.org/download> (download mirrors are at the bottom of the page) and extract it somewhere:

```sh
doas tar -xf archlinux-bootstrap-x86_64.tar.zst --numeric-owner
```

This will extract into a `root.x86_64` directory.

You can then create the dataset you want your Arch root to be on, and copy `root.x86_64` into it:

```sh
sudo zfs create -o mountpoint=/mnt tank/arch-root
sudo mv root.x86_64/* /mnt
```

## Setting up Arch 🤓

While I've had success doing this entirely from FreeBSD in the past, it is definitely easier to do this from a Linux system.
I recommend using a live USB of the distro of your choosing (in my case, [Linux Mint](https://linuxmint.com/)).

First step is going to be to install ZFS to our live system and to import/mount our pool (named `tank` in this guide):

```sh
sudo apt install zfsutils-linux
sudo zpool import
sudo zpool import -f tank -R /mnt
```

Our previously bootstrapped Arch root is now mounted at `/mnt`.
Then we can chroot into it as such:

```sh
sudo mount --rbind {,/mnt/tank/arch-root}/dev
sudo mount --rbind {,/mnt/tank/arch-root}/proc
sudo mount --rbind {,/mnt/tank/arch-root}/sys
sudo cp {,/mnt/tank/arch-root}/etc/resolv.conf # For DNS.
sudo chroot /mnt/tank/arch-root
```

Do note that, if you want to unmount anything, you must first run `mount --make-rslave` on the mountpoint so that e.g. unmounting `/dev/pts` in our chroot doesn't unmount our live system's `/dev/pts`:

```sh
sudo mount --make-rslave /mnt/tank/arch-root/dev
sudo umount -R -l /mnt/tank/arch-root/dev
```

We're now gonna wanna bootstrap pacman next:

```sh
pacman-key --init
pacman-key --populate
```

You can then uncomment the server you want to use in `/etc/pacman.d/mirrorlist` (which I think must be done from outside the chroot - idk why but it seems like there is no text editor in the Arch Linux imgae) and update the package database:

```sh
pacman -Sy
```

### I *really* want to do this from FreeBSD

Okay fine.

I can't help you all that much but I can at least give you a starting point by showing how to set up a basic chroot environment.
With the `linux64` kernel module loaded:

```sh
doas mount -t linprocfs linprocfs root.x86_64/proc
doas mount -t linsysfs linsysfs root.x86_64/sys
doas mount -t devfs devfs root.x86_64/dev
doas cp {,root.x86_64}/etc/resolv.conf
doas chroot root.x86_64
```

You might have to pass `--disable-sandbox` to pacman if you get this error:

```log
error: restricting filesystem access because landlock is not supported by the kernel!
```

This is because FreeBSD's Linuxulator doesn't support Linux's landlock LSM as of yet.

## Adding kernel and ZFS kernel module 🌽

Install the kernel (you can use the `mkinitcpio` provider for `initramfs`):

```sh
pacman -S linux
```

Add the `archzfs` repo to `/etc/pacman.conf` (choose the mirror closest to you):

```sh
[archzfs]
# Origin Server - Finland
Server = http://archzfs.com/$repo/$arch
# Mirror - Germany
Server = http://mirror.sum7.eu/archlinux/archzfs/$repo/$arch
# Mirror - Germany
Server = http://mirror.sunred.org/archzfs/$repo/$arch
# Mirror - Germany
Server = https://mirror.biocrafting.net/archlinux/archzfs/$repo/$arch
# Mirror - India
Server = https://mirror.in.themindsmaze.com/archzfs/$repo/$arch
# Mirror - US
Server = https://zxcvfdsa.com/archzfs/$repo/$arch
```

Then, update package databases:

```sh
pacman -Sy
```

You may need to do [this](https://www.reddit.com/r/archlinux/comments/hom2v8/archzfs_signature_from_archzfs_bot/) to get the signature validated:

```sh
pacman-key -d DDF7DB817396A49B2A2723F7403BD972F75D9D76
pacman-key -r DDF7DB817396A49B2A2723F7403BD972F75D9D76 --keyserver keyserver.ubuntu.com
pacman-key --lsign-key DDF7DB817396A49B2A2723F7403BD972F75D9D76 # the important one!
pacman -Sy
```

Install ZFS:

```sh
pacman -S zfs-linux
```

You might have to build `zfs-linux` from the [AUR](https://aur.archlinux.org/) if it doesn't target the kernel version you just installed.
Alternatively you could try DKMS (`zfs-dkms`), but I didn't have too much luck doing that so YMMV.

Check that the ZFS kernel module is indeed in the initramfs, because the `zfs` hook doesn't always seem to be run for some reason:

```sh
lsinitcpio /boot/initr* | grep modules/.*/zfs
```

If it's not in the hook, add `zfs` to the `HOOKS` list in `/etc/mkinitcpio.conf` and rerun the `mkinitcpio` command (and also recheck with `lsinitcpio` of course):

```sh
mkinitcpio -p linux
```

Phew.
Well that wasn't really a walk in the park 🙂
I'll be honest the ZoL experience on Arch isn't too great compared to Ubuntu or FreeBSD.

## Booting 👢

We've installed and set up our root filesystem and all that, but we still need a way to actually boot into it.
I'm going to cover two ways to do this; one using a traditional bootloader, i.e. [GRUB](https://www.gnu.org/software/grub/), and one using the fancy schmancy [EFI boot stub](https://docs.kernel.org/admin-guide/efi-stub.html).

## GRUB 🐛

Conceptually, this is quite simple.
Just install GRUB in the chroot:

```sh
pacman -S grub
```

Outside of the chroot, we need to mount our target ESP (EFI System Partition) to `/boot/efi`.
In this example, that's going to be `/dev/nvme0n1p1`:

```sh
mkdir /boot/efi
mount /dev/nvme0n1p1 /boot/efi
```

Once that's done, you should just be able to install GRUB as you normally would inside the chroot:

```sh
mkdir /boot/grub
grub-mkconfig > /boot/grub/grub.cfg
grub-install
```

## EFI boot stub 🪵

I'm going to assume you already know what the [EFI boot stub](https://docs.kernel.org/admin-guide/efi-stub.html) is and have already built a kernel with `CONFIG_EFI_STUB=y` as `bzImage`.
Do of course make sure it is the same version as the ZFS kernel module you installed.

**TODO** Generate initramfs?

```sh
fs0:
cd efi/arch
vmlinuz initrd=\efi\arch\initramfs-linux.img root=ZFS=tank/arch-root
```

**TODO** Both `efibootmgr` commands, for FreeBSD and Linux.

**rdinit** for custom linux with `CONFIG_BLOCK`.
`dpkg-architecture` from `dpkg-dev` for building ZFS, otherwise it builds for 32-bit.
