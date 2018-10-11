# rust-cross-compile

<!-- vim-markdown-toc GitLab -->

- [What is this for?](#what-is-this-for)
- [Usage](#usage)
- [Status](#status)
- [More](#more)

<!-- vim-markdown-toc -->

## What is this for?

Firstly, a big shout out to the [Rust Embedded Tools Team](https://github.com/rust-embedded/wg#the-tools-team), particularly the [cross](https://github.com/rust-embedded/cross) project.

However, these projects are focussed on pure Rust development and I have found that they mostly fail when using C-based libraries, such as [libsodium](https://github.com/jedisct1/libsodium). These have some very nice Rust binding, in the case of `libsodium` the crate I use is [sodiumoxide](https://github.com/sodiumoxide/sodiumoxide), but I find I cannot simply cross-compile everything using the `cross` tooling as there are issues with the underlying Docker images.

So, what we have here is a set of tuned cross-compile Docker images that work with the rust-embedded/cross crate.

## Usage

Simply clone locally and then use as a project template for your work.


## Status

Under current development, after a time away. 

## More

Look at the [docs](docs) folder for more information on cross-compiling Rust. In particular, the [Development Notes](docs/Development-Notes.md) and the [Docker Configuration](docs/Docker-Configuration.md).

