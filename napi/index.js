/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'apimokka.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./apimokka.android-arm64.node')
          } else {
            nativeBinding = require('@apimokka/bin-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'apimokka.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./apimokka.android-arm-eabi.node')
          } else {
            nativeBinding = require('@apimokka/bin-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'apimokka.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./apimokka.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@apimokka/bin-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'apimokka.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./apimokka.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@apimokka/bin-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'apimokka.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./apimokka.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@apimokka/bin-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'apimokka.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./apimokka.darwin-universal.node')
      } else {
        nativeBinding = require('@apimokka/bin-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'apimokka.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./apimokka.darwin-x64.node')
          } else {
            nativeBinding = require('@apimokka/bin-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'apimokka.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./apimokka.darwin-arm64.node')
          } else {
            nativeBinding = require('@apimokka/bin-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'apimokka.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./apimokka.freebsd-x64.node')
      } else {
        nativeBinding = require('@apimokka/bin-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'apimokka.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./apimokka.linux-x64-musl.node')
            } else {
              nativeBinding = require('@apimokka/bin-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'apimokka.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./apimokka.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@apimokka/bin-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'apimokka.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./apimokka.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@apimokka/bin-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'apimokka.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./apimokka.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@apimokka/bin-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'apimokka.linux-arm-musleabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./apimokka.linux-arm-musleabihf.node')
            } else {
              nativeBinding = require('@apimokka/bin-linux-arm-musleabihf')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'apimokka.linux-arm-gnueabihf.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./apimokka.linux-arm-gnueabihf.node')
            } else {
              nativeBinding = require('@apimokka/bin-linux-arm-gnueabihf')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'apimokka.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./apimokka.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('@apimokka/bin-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'apimokka.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./apimokka.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('@apimokka/bin-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 's390x':
        localFileExisted = existsSync(
          join(__dirname, 'apimokka.linux-s390x-gnu.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./apimokka.linux-s390x-gnu.node')
          } else {
            nativeBinding = require('@apimokka/bin-linux-s390x-gnu')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { napiRun } = nativeBinding

module.exports.napiRun = napiRun
