# API mokka

[apimock-rs](https://github.com/nabbisen/apimock-rs) (API mock) based visual mocking helper to handle HTTP/JSON req/res written in Rust.

[![crates.io](https://img.shields.io/crates/v/apimokka?label=latest)](https://crates.io/crates/apimokka)
[![Documentation](https://docs.rs/apimokka/badge.svg?version=latest)](https://docs.rs/apimokka)
[![Dependency Status](https://deps.rs/crate/apimokka/latest/status.svg)](https://deps.rs/crate/apimokka)
[![Releases Workflow](https://github.com/nabbisen/apimokka/actions/workflows/release-executable.yaml/badge.svg)](https://github.com/nabbisen/apimokka/actions/workflows/release-executable.yaml)
[![License](https://img.shields.io/github/license/nabbisen/apimokka)](https://github.com/nabbisen/apimokka/blob/main/LICENSE)

## Summary

Mock with mokka ☕️🌄

📖 Docs around server [Configure](https://github.com/nabbisen/apimock-rs/blob/main/docs/CONFIGURE.md), and [examples](https://github.com/nabbisen/apimock-rs/blob/main/examples/config/full/)

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

```sh
npm install -D @apimockka/bin

npm exec @apimokka/bin
# in case app args passed:
npm exec @apimokka/bin -- -c apimock.toml -p 3002 --middleware apimock-middleware.rhai
```

## Development

```sh
cargo run

cargo test
```

## Acknowledgements

Depends on [apimock-rs](https://github.com/nabbisen/apimock-rs) and [fltk-rs](https://github.com/fltk-rs/fltk-rs). [napi-rs](https://github.com/napi-rs/napi-rs) for binding for [Node.js](https://nodejs.org/).
