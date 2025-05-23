# API mokka

## 🛠️ App overhaul announcement

### ⚠️ Major update in development (apimock-rs, the core lib, v4)

apimock-rs, the core lib, v4 introduces a fundamental shift in design. Existing configuration files will no longer work as-is. Some features will be deprecated.
In return: cleaner configs, dynamic resolution, and better extensibility.

## Summary

[apimock-rs](https://github.com/nabbisen/apimock-rs) (API mock) based visual mocking helper to handle HTTP/JSON req/res written in Rust. Rust GUI app Binding for Node.js.

Mock with mokka ☕️🌄

### Features

- GUI but lightweight as feather
- Intutive terminal and productive tabs
- Built as native and supports cross-platform

### [Screenshot](https://github.com/nabbisen/apimokka#screenshot)

## Usage

### Install

```sh
npm install -D apimokka
```

### Run

```sh
npm install -D @apimockka/bin

npm exec @apimokka/bin

# case when passing app args:
npm exec @apimokka/bin -- -c apimock.toml -p 3002 --middleware apimock-middleware.rhai

# case when initializing app config and middleware files:
npm exec @apimokka/bin -- --init
```

📖 Docs around server [Configure](https://github.com/nabbisen/apimock-rs/blob/main/docs/CONFIGURE.md), and [examples](https://github.com/nabbisen/apimock-rs/blob/main/examples/config/full/)

## Supported platforms

- Linux GNU x64 (`x86_64-unknown-linux-gnu`)
- macOS arm64 (`arm64-apple-darwin`)
- Windows MSVC x64 (`x86_64-pc-windows-msvc`)
