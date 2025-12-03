# rust-libxml-190
Minimum Reproduce of [Issue 190 in Rust Libxml](https://github.com/KWARC/rust-libxml/issues/190).

## Environment

Host: M1 Mac (Aarch64)
Docker env: Podman Desktop 1.2.3 w/ Podman v5.4.0
Rust: 1.91

Rust Host version: `rustc 1.91.1 (ed61e7d7e 2025-11-07)`
Rust Container version: `rustc 1.91.1 (ed61e7d7e 2025-11-07)`

## Commands

In the `libxml-deadlock` folder:

Make a docker container for linux/amd64...

`docker build --platform=linux/amd64 -t morerust:latest -f Dockerfile .`

Run container with source in it.

`docker run --rm -v "$PWD:/data" -it morerust`

Run the unit tests in the container.

`./test_local.sh`

The tests hang on startup, yet work fine locally using `./test-local.sh`