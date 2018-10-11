# rust-cross-compile


<!-- vim-markdown-toc GitLab -->

- [Script](#script)

<!-- vim-markdown-toc -->

A set of cross-compile targets using the rust-embedded/cross crate and tuned Docker containers



## Script

```
# Cross compile for arm-unknown-linux-gnueabi (ARM v6)
# Build the docker image
docker build docker/arm-unknown-linux-gnueabi/. --tag avastmick/rust-cross-compile:arm-unknown-linux-gnueabi;
# Run the test
cross test --target arm-unknown-linux-gnueabi
#
# Now do the same for arm-unknown-linux-gnueabihf (Raspberrypi ARM v7) 
# Build the docker image
docker build docker/arm-unknown-linux-gnueabihf/. --tag avastmick/rust-cross-compile:arm-unknown-linux-gnueabihf;
# Run the test
cross test --target arm-unknown-linux-gnueabihf
```
