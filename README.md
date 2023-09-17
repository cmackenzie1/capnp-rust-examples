# Cap'N'Proto Rust Examples

[![Rust](https://github.com/cmackenzie1/capnp-rust-examples/actions/workflows/rust.yml/badge.svg)](https://github.com/cmackenzie1/capnp-rust-examples/actions/workflows/rust.yml)

This repository contains examples of using [Cap'N'Proto](https://capnproto.org/) in Rust.

The `schemas` directory contains the Cap'N'Proto schemas used in the examples. They are written in the [Cap'N'Proto schema language](https://capnproto.org/language.html) and compiled into Rust code using the `capnpc` crate.

## Usage

```bash
$ cargo run --example 01_hello_alice
$ cargo run --example 02_vec_u8
$ cargo run --example 03_bytes
```

## Examples

- [01_hello_alice](examples/01_hello_alice.rs) - A simple example creating a message and writing it to a file.
- [02_vec_u8](examples/02_vec_u8.rs) - An example of using a `Vec<u8>` as the writer and reader.
- [03_bytes](examples/03_bytes.rs) - An example of using a `bytes` crate as the writer and reader.

## FAQ

### Why do you compile the schemas separately?

Cap'N'Proto isn't tied to any single language. The schemas are compiled into Rust code using the `capnpc` crate. This allows the schemas to be compiled into other languages as well. For example, the schemas could be compiled into C++ code and used in a C++ project, or compiled into Java code and used in a Java project.

### How can I send a message across threads (`Send`) in an async context?

There are a few ways to do this. One simple way is to use a `Vec<u8>` as the writer and reader and send the `Vec<u8>` across threads. See [02_vec_u8](examples/02_vec_u8.rs) for an example. You can also use the `bytes` crate as the writer and reader. See [03_bytes](examples/03_bytes.rs) for an example.

There is an upstream feature request to have Cap'N'Proto support `Send` and `Sync` directly: https://github.com/capnproto/capnproto-rust/issues/256
