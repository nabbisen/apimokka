# API mokka

Mock with mokka ☕️🌄 API mock (apimock-rs) based visual mocking helper to handle HTTP/JSON req/res written in Rust.

[![crates.io](https://img.shields.io/crates/v/apimokka?label=latest)](https://crates.io/crates/apimokka)
[![Documentation](https://docs.rs/apimokka/badge.svg?version=latest)](https://docs.rs/apimokka)
[![Dependency Status](https://deps.rs/crate/apimokka/latest/status.svg)](https://deps.rs/crate/apimokka)
[![Releases Workflow](https://github.com/nabbisen/apimokka/actions/workflows/release-executable.yaml/badge.svg)](https://github.com/nabbisen/apimokka/actions/workflows/release-executable.yaml)
[![License](https://img.shields.io/github/license/nabbisen/apimokka)](https://github.com/nabbisen/apimokka/blob/main/LICENSE)

## Summary

Visual mocking helper to handle API mock Server generating HTTP/JSON responses written in Rust.

## Usage

### Executable

[Assets](https://github.com/nabbisen/apimokka/releases/latest) in Releases offer executables for multiple platforms.

```sh
./apimokka
```

There are an example config file and directories by default.

### `cargo` install

```sh
cargo install apimokka
```

## Screenshot

![screenshot.png](screenshot.png)

## Binding for Node.js

\* Under development \*

```sh
npm install -D @apimockka/bin
```

## Development

```sh
cargo run
```

## Acknowledgements

Depends on [apimock-rs](https://github.com/nabbisen/apimock-rs) and [fltk-rs](https://github.com/fltk-rs/fltk-rs).
