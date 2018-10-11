FROM debian:stretch

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    crossbuild-essential-armhf \
    clang \
    gcc-arm-linux-gnueabihf \
    libc6-dev-armhf-cross \
    qemu-user-static
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER=arm-linux-gnueabihf-gcc \
    QEMU_LD_PREFIX=/usr/arm-linux-gnueabihf \
    RUST_TEST_THREADS=1

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
  && export CC=arm-linux-gnueabihf-gcc \
  && ../../configure --host=arm-linux-gnueabihf --prefix=/usr/arm-linux-gnueabihf --target arm-linux-gnueabihf\
  && make && make check \
  && make install && /sbin/ldconfig \
  && mv src/libsodium /usr/local/ \
  && rm -Rf /tmp/libsodium-${LIBSODIUM_VERS}/

ENV PKG_CONFIG_ALLOW_CROSS=1 \
    SODIUM_LIB_DIR=/usr/arm-linux-gnueabihf/lib \
    SODIUM_INC_DIR=/usr/arm-linux-gnueabihf/include
