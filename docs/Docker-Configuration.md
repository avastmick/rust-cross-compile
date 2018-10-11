# Docker Configuration

Below are the currently configured `Dockerfile`s for each cross-compile target, their status and notes to why things are done the way they are. This document is key so that decisions are **not** relitigated or accidentally rewound when trying to fix problems.

The notes in each `Dockerfile` are added as comments. These are not present in the actual `Dockerfile`.

<!-- vim-markdown-toc GitLab -->

- [General Status](#general-status)
- [avastmick/rust-cross-compile:arm-unknown-linux-gnueabi](#avastmickrust-cross-compilearm-unknown-linux-gnueabi)
  - [Current status:](#current-status)
  - [To Do](#to-do)
  - [Dockerfile](#dockerfile)
- [avastmick/rust-cross-compile:arm-unknown-linux-gnueabihf](#avastmickrust-cross-compilearm-unknown-linux-gnueabihf)
  - [Current status:](#current-status-1)
  - [To Do](#to-do-1)
    - [FAILING](#failing)
  - [Dockerfile](#dockerfile-1)

<!-- vim-markdown-toc -->
 
## General Status

The goal is make the following Docker images available on the [hub.docker.com](https://hub.docker.com) registry.

**STATUS**: FAILING. Reason: The [hub.docker.com](https://hub.docker.com) registry cannot build the images. The build fails when cross-compiling `libsodium` during the `make check` step, when all the tests fail. I cannot reproduce this locally, but is possibly due to something to do with `qemu` - at a guess. 


## avastmick/rust-cross-compile:arm-unknown-linux-gnueabi

This `dockerfile` enables the `rust-cross` to target ARM v6.

### Current status:

**TESTED** and **FUNCTIONING** for the following versions:
  - Rust: 1.29.1
  - `libsodium` 1.0.16

### To Do

- Add in versions from single properties file (toml)

### Dockerfile

```
FROM debian:stretch

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    crossbuild-essential-armel \  # Adds make and other build tools
    clang \                       # Required by the sodiumoxide crate
    gcc-arm-linux-gnueabi \       # The target cross compiler
    libc6-dev-armel-cross \       # Adds the target header libraries
    qemu-user-static              # Adds the correct qemu virtual files (Taken from the source Dockerfile by japaric)

# Adds in environment variables to be used by rust-cross
# (Taken from the source Dockerfile by japaric)
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER=arm-linux-gnueabi-gcc \
    QEMU_LD_PREFIX=/usr/arm-linux-gnueabi \
    RUST_TEST_THREADS=1

# Useful libraries

# Libsodium - we install a specified version and cross-compile it for the right arch target 
# Note the following: 
#   - because of the way the arm-linux-gnueabi-gcc compiler searches we cannot use the 
#     the standard libsodium installation location, so we override using the --prefix flag
#     to check the actual compiler params, run the container and `arm-linux-gnueabi-gcc -v` to list
#   - the host seems to require forcing.
ENV LIBSODIUM_VERS 1.0.16
RUN \
  mkdir -p /tmp/libsodium \
  && cd /tmp/libsodium \
  && curl -fsSL https://download.libsodium.org/libsodium/releases/libsodium-${LIBSODIUM_VERS}.tar.gz \
  -o libsodium-${LIBSODIUM_VERS}.tar.gz \
  && tar xfvz libsodium-${LIBSODIUM_VERS}.tar.gz \
  && cd libsodium-${LIBSODIUM_VERS} \
  && mkdir -p target/arm-unknown-linux-gnueabi \
  && cd target/arm-unknown-linux-gnueabi \
  && export CC=arm-linux-gnueabi-gcc \    # Need to test if we need this line, 
                                          #  given we are configuring with the following flags.
  && ../../configure --host=arm-linux-gnueabi --prefix=/usr/arm-linux-gnueabi \
  && make && make check \
  && make install && /sbin/ldconfig \
  && mv src/libsodium /usr/local/ \
  && rm -Rf /tmp/libsodium-${LIBSODIUM_VERS}/

# Set up the cross compilation environment variables so the running is cleaner.
# If we do not have these we need to pass them as variables to `cross` and add them to the "passthrough" set in 
#   Cross.toml
ENV PKG_CONFIG_ALLOW_CROSS=1 \
    SODIUM_LIB_DIR=/usr/arm-linux-gnueabi/lib\
    SODIUM_INC_DIR=/usr/arm-linux-gnueabi/include
```


## avastmick/rust-cross-compile:arm-unknown-linux-gnueabihf

This `dockerfile` enables the `rust-cross` to target ARM v7 Raspberrypi target (so, not a Raspberrypi v1 or Zero).

### Current status:

**FAILING** for the following versions:
  - Rust: 1.29.1
  - `libsodium` 1.0.16

### To Do

- FIX the following:
  -  
- Add in versions from single properties file (toml)

#### FAILING

Does not pick up any of the `PKG_CONFIG_ALLOW_CROSS`, `SODIUM_LIB_DIR`, or `SODIUM_INC_DIR` flags. These have to be added to the `passthrough` set in `Cross.toml`. This is even though the `Dockerfile` is set up in the same way as the `avastmick/rust-cross-compile:arm-unknown-linux-gnueabi` target.

I pass in the flags using: `PKG_CONFIG_ALLOW_CROSS=1 SODIUM_LIB_DIR=/usr/arm-linux-gnueabihf/lib SODIUM_INC_DIR=/usr/arm-linux-gnueabihf/include cross test --target arm-unknown-linux-gnueabihf`

To accept this are actually pass the flags I have to add the following to the `passthrough` in `Cross.toml`:

```
  passthrough = [
    "PKG_CONFIG_ALLOW_CROSS",
    "SODIUM_LIB_DIR",
    "SODIUM_INC_DIR",
  ]
```

Result is:

```
Compiling libsodium-sys v0.1.0                                                                                     
error: failed to run custom build command for `libsodium-sys v0.1.0`
process didn't exit successfully: `/home/avastmick/dev/rust-cross-compile/target/debug/build/libsodium-sys-4879cabe1c504b63/build-script-build` (exit code: 101)
--- stdout
cargo:rerun-if-env-changed=SODIUM_LIB_DIR
cargo:rerun-if-env-changed=SODIUM_INC_DIR
cargo:rerun-if-env-changed=SODIUM_STATIC
cargo:rustc-link-search=native=/usr/arm-linux-gnueabihf/lib
cargo:rustc-link-lib=dylib=sodium

--- stderr
/usr/include/x86_64-linux-gnu/gnu/stubs.h:7:11: fatal error: 'gnu/stubs-32.h' file not found
/usr/include/x86_64-linux-gnu/gnu/stubs.h:7:11: fatal error: 'gnu/stubs-32.h' file not found, err: true
thread 'main' panicked at 'Unable to generate bindings: ()', libcore/result.rs:945:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

### Dockerfile

```
FROM debian:stretch
# STATUS: INCOMPLETE and UNTESTED
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    crossbuild-essential-armhf \  # Adds make and other build tools
    clang \                       # Required by the sodiumoxide crate
    gcc-arm-linux-gnueabihf \       # The target cross compiler
    libc6-dev-armhf-cross \       # Adds the target header libraries
    qemu-user-static              # Adds the correct qemu virtual files (Taken from the source Dockerfile by japaric)

# Adds in environment variables to be used by rust-cross
# (Taken from the source Dockerfile by japaric)
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER=arm-linux-gnueabihf-gcc \
    QEMU_LD_PREFIX=/usr/arm-linux-gnueabihf \
    RUST_TEST_THREADS=1

# Crypto libraries
# Libsodium - we install a specified version and cross-compile it for the right arch target 
# Note the following: 
#   - because of the way the arm-linux-gnueabihf-gcc compiler searches we cannot use the 
#     the standard libsodium installation location, so we override using the --prefix flag
#     to check the actual compiler params, run the container and `arm-linux-gnueabihf-gcc -v` to list
#   - the host seems to require forcing.
#    - TODO how to pass in version number from properties / toml file
ENV LIBSODIUM_VERS 1.0.16
RUN \
  mkdir -p /tmp/libsodium \
  && cd /tmp/libsodium \
  && curl -fsSL https://download.libsodium.org/libsodium/releases/libsodium-${LIBSODIUM_VERS}.tar.gz \
  -o libsodium-${LIBSODIUM_VERS}.tar.gz \
  && tar xfvz libsodium-${LIBSODIUM_VERS}.tar.gz \
  && cd libsodium-${LIBSODIUM_VERS} \
  && mkdir -p target/arm-unknown-linux-gnueabihf \
  && cd target/arm-unknown-linux-gnueabihf \
  && export CC=arm-linux-gnueabihf-gcc \  # Need to test if we need this line, 
                                          #  given we are configuring with the following flags.
  && ../../configure --host=arm-linux-gnueabihf --prefix=/usr/arm-linux-gnueabihf --target arm-linux-gnueabihf\
  && make && make check \
  && make install && /sbin/ldconfig \
  && mv src/libsodium /usr/local/ \
  && rm -Rf /tmp/libsodium-${LIBSODIUM_VERS}/

# Set up the cross compilation environment variables so the running is cleaner.
ENV PKG_CONFIG_ALLOW_CROSS=1 \
    SODIUM_LIB_DIR=/usr/arm-linux-gnueabihf/lib \
    SODIUM_INC_DIR=/usr/arm-linux-gnueabihf/include
```


