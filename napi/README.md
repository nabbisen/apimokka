# API mokka

[apimock-rs](https://github.com/nabbisen/apimock-rs) (API mock) based visual mocking helper to handle HTTP/JSON req/res written in Rust. Rust GUI app Binding for Node.js.

Mock with mokka ☕️🌄

## Usage

### Install

```sh
npm install -D apimokka
```

### Run

```sh
npm exec @apimokka/bin
# in case app args passed:
npm exec @apimokka/bin -- -c apimock.toml -p 3002 --middleware apimock-middleware.rhai
```

📖 Docs around server [Configure](https://github.com/nabbisen/apimock-rs/blob/main/docs/CONFIGURE.md), and [examples](https://github.com/nabbisen/apimock-rs/blob/main/examples/config/full/)

## Supported platforms

- `x86_64-unknown-linux-gnu`
- `arm64-apple-darwin`
- `x86_64-pc-windows-msvc`

ref: https://napi.rs/docs/cli/napi-config
