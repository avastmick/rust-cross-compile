# General Development Notes and Gotchas 

Follow the following steps to enable development.

<!-- vim-markdown-toc GitLab -->

- [Local Cross-Compilation](#local-cross-compilation)
  - [General Notes](#general-notes)
  - [Steps](#steps)
- [Emulation](#emulation)
  - [Usage](#usage)
  - [Coding](#coding)
- [Current Issues](#current-issues)
  - [QEMU Segmentation Fault on Compile in Container](#qemu-segmentation-fault-on-compile-in-container)
    - [Why - Idea One](#why-idea-one)
    - [Why - Idea Two](#why-idea-two)
    - [Why - Idea Three](#why-idea-three)
  - [Ubuntu apt corruption - local cross compilation](#ubuntu-apt-corruption-local-cross-compilation)
    - [Clean up](#clean-up)

<!-- vim-markdown-toc -->

## Local Cross-Compilation

Developing directly on the target hardware (in this instance, a RaspberryPi) is less than ideal, does not scale, nor allow for easy CI. Therefore the general recommendation is to cross-compile to the target architecture instead.

Most of the steps were taken from the following:

- [guide for cross-compiling Maidsafe for RaspberryPi](https://safenetforum.org/t/cross-compiling-maidsafe-rust-code-for-arm/4175).
- [Jorge Aparicio's Rust Cross instructions](https://github.com/japaric/rust-cross)
- [Takeshix's instructions](https://gist.github.com/takeshixx/686a4b5e057deff7892913bf69bcb85a)

### General Notes

ARM versions:

- ARM version 7: for reference most later ARM devices and the RaspberryPi 2 and 3 - Broadcom BCM2836 / BCM2837
- ARM version 6: this is the target for a POC as we are targeting the RaspberryPi Zero - Broadcom BCM2835

Therefore, we need to ensure the correct compiler and toolchain. Note that many of the cross compilation instructions are targeting ARM v7, not the ARM v6 we are wanting here.

- Target: ARM v6
  - Toolchain: right so the toolchain recommended in the instructions, `gnueabihf`, the **hf** means it's going to link against **hard-float** objects that are not supported by the ARM1176JZF-S core found in the BCM2835 SoC of the RaspberryPi Zero. 
  - Flags: the following flags are required to "turn off" the hard-float object generation.
    - 

**Alternatives**

The general issues with the toolchain for ARMv6 targets is messy and problematic. However, for a RaspberryPi there seems to be a specific solution due to the extended compilation for Raspbian. The following article details how to create a viable SDK:

- [Raspbian cross compilation SDK](https://medium.com/@zw3rk/making-a-raspbian-cross-compilation-sdk-830fe56d75ba)

AT this point this approach is untested.

- Use Linux. If you're using Windows or Mac, you're on your own!
- Use a Debian-based distro, if you're wedded to Arch (as I was), it's a lot more painful.

### Steps

Add the toolchain for Rust:

- `rustup target add arm-unknown-linux-gnueabihf`

Add the right C compiler:

For the project, you'll need `clang` (for `sodiumoxide`) and `gcc-arm-linux-gnueabihf` to cross compile.

Using `libsodium`:

- From the [libsodium docs](https://libsodium.gitbook.io/doc/usage): the library is called "sodium", use `-lsodium` as a flag in order to link it.

Debian (Ubuntu etc):

  - `sudo apt-get install clang gcc-arm-linux-gnueabihf`

  *See issue below*

Arch Linux:

  - `sudo pacman -S clang`
  - `mkdir rpi && cd rpi && git clone --depth 1 https://github.com/raspberrypi/tools.git`
  - `echo 'export PATH=$PATH:~/rpi/tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian/bin' >> ~/.bash_profile`
  - `source ~/.bash_profile`
  - Check it works: `arm-linux-gnueabihf-gcc -v`

  The installation will be added under `pacman` The `AUR` repositories have too many dependency issues.

Run cargo to use that tool chain:

- `PKG_CONFIG_ALLOW_CROSS=1 cargo test --target=arm-unknown-linux-gnueabihf`

## Emulation

While Rust is easy to use for cross compilation, there is a Docker environment that makes emulation much simpler and closer to the target for the POC (Raspberry Pi). NOTE: See issue, below!

### Usage

Dependencies:

- `sudo apt-get install qemu-user-static`

Simply:

- `./build-devenv.sh`
- `./run-devenv.sh`

### Coding

The rust compiler is fast on inside the container, so to do complete `cargo` builds, the recommendation is to do this.

However, for editors (Vim, VS-Code etc.) you'll need a local version and the compilation is best done using a cross target.

## Current Issues

### QEMU Segmentation Fault on Compile in Container
 
At this point in time (09-26-2018) emulation via qemu in a docker container fails. When the build is done via emulation (in the docker container), we have the following fatal error:

`qemu: uncaught target signal 11 (Segmentation fault) - core dumped
Segmentation fault (core dumped)`

Not sure why. 

#### Why - Idea One

One idea is that the container runs out of memory. So I added the following flags to the `docker run`

- `--memory="2g" --memory-swap="4g" --memory-swappiness="100"`

However, this doesn't work as you get the following:

- `WARNING: Your kernel does not support swap limit capabilities or the cgroup is not mounted. Memory limited without swap.`

The solution to this is supposed to be:

- `sudoedit /etc/default/grub`
- Add line:
- `GRUB_CMDLINE_LINUX="cgroup_enable=memory swapaccount=1"`
- Then:
- `sudo update_grub`
- `sudo reboot`

However, this does not seem to work in Ubuntu 18.04 LTS. 

#### Why - Idea Two

This is more serious. The suggestion is that the compiler is actually writing incorrect instructions to registries that may not exist. I'm not sure about this, nor enough at that (low) level to be able to understand it.

Reference can be found [HERE](http://embed.rs/articles/2016/arm-inline-assembly-rust/)

#### Why - Idea Three

When cross compiling the version of `libsodium` used for the host. In emulation, the right one is used, but maybe the bindings are wrong.

### Ubuntu apt corruption - local cross compilation

I have hit an issue with local cross compilation where `apt` no longer functions and bombs out with the following error:

```
dpkg: error processing archive /var/cache/apt/archives/libc6-armhf-cross_2.27-3ubuntu1cross1.1_all.deb (--unpack):
 unable to open '/usr/arm-linux-gnueabihf/lib/ld-2.27.so.dpkg-new': No such file or directory
No apport report written because MaxReports is reached already
                                                              Errors were encountered while processing:
 /var/cache/apt/archives/libc6-dev-armel-cross_2.27-3ubuntu1cross1.1_all.deb
 /var/cache/apt/archives/libc6-armel-cross_2.27-3ubuntu1cross1.1_all.deb
 /var/cache/apt/archives/libc6-dev-armhf-cross_2.27-3ubuntu1cross1.1_all.deb
 /var/cache/apt/archives/libc6-armhf-cross_2.27-3ubuntu1cross1.1_all.deb
E: Sub-process /usr/bin/dpkg returned an error code (1)
```

#### Clean up

- `sudo apt-get remove gcc-arm-linux-gnueabihf libc6-armel-cross libc6-armhf-cross libc6-dev-armel-cross libc6-dev-armhf-cross`
- `sudo apt autoremove`
