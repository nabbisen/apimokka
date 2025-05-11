# API mokka Development

## Where are files around multiple platforms support

napi's `npm/` directory will be automatically generated in GitHub Actions CI workflow with `napi create-npm-dir`.

## Maintenance

### Packages update

```console
$ npm update # update package-lock.json
```

### Just version modify

```console
$ npm version 0.0.0 # next version
```

## Supported platforms

### napi

Default replacing darwin x64 with arm64 (ref: https://napi.rs/docs/cli/napi-config )
