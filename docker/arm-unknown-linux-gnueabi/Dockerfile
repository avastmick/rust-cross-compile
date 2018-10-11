FROM debian:stretch

# See docs/Docker-Configuration.md for commentary on this Dockerfile.

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    crossbuild-essential-armel \
    clang \
    gcc-arm-linux-gnueabi \
    libc6-dev-armel-cross \
    qemu-user-static
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER=arm-linux-gnueabi-gcc \
    QEMU_LD_PREFIX=/usr/arm-linux-gnueabi \
    RUST_TEST_THREADS=1

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
  && export CC=arm-linux-gnueabi-gcc \
  && ../../configure --host=arm-linux-gnueabi --prefix=/usr/arm-linux-gnueabi \
  && make && make check \
  && make install && /sbin/ldconfig \
  && mv src/libsodium /usr/local/ \
  && rm -Rf /tmp/libsodium-${LIBSODIUM_VERS}/

ENV PKG_CONFIG_ALLOW_CROSS=1 \
    SODIUM_LIB_DIR=/usr/arm-linux-gnueabi/lib\
    SODIUM_INC_DIR=/usr/arm-linux-gnueabi/include
