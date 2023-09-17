# schemas

This directory contains the Cap'N'Proto schemas used in the examples. They are written in the [Cap'N'Proto schema language](https://capnproto.org/language.html).

The generated files are written to `./generated/<language>`, and made available as an module for that language.

## Usage

```bash
make generate
```

```toml
# Cargo.toml
[dependencies]
schemas = { path = "./schemas/generated/rust" }
```
