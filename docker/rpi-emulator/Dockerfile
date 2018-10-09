FROM resin/rpi-raspbian

RUN \
  apt-get update && \
  apt-get install -y curl unzip git build-essential clang && \
  rm -rf /var/lib/apt/lists/*

# Rust set up
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.29.0

# install rustup
RUN curl -fsSL https://sh.rustup.rs -sSf -o install_rustup.sh; \
    chmod +x install_rustup.sh; \
    ./install_rustup.sh -y --default-toolchain $RUST_VERSION; \
    rm install_rustup.sh; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    rustup component add rustfmt-preview

# Crypto libraries
# Libsodium - the reason to use this method instead of the Debian packaged versions is to ensure 
#   we have something more up to date
ENV LIBSODIUM_VERS 1.0.16
RUN \
  mkdir -p /tmp/libsodium \
  && cd /tmp/libsodium \
  && curl -fsSL https://download.libsodium.org/libsodium/releases/libsodium-${LIBSODIUM_VERS}.tar.gz \
  -o libsodium-${LIBSODIUM_VERS}.tar.gz \
  && tar xfvz libsodium-${LIBSODIUM_VERS}.tar.gz \
  && cd libsodium-${LIBSODIUM_VERS}\
  && ./configure \
  && make && make check \
  && make install && /sbin/ldconfig \
  && mv src/libsodium /usr/local/ \
  && rm -Rf /tmp/libsodium-${LIBSODIUM_VERS}/

# Need to add in the Tini entrypoint to prevent moaning about it being subprocess
ENTRYPOINT ["/sbin/tini", "-s", "--"]
