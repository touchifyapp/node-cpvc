# `@touchifyapp/cpvc` (Cross-Platform Volume Control)

![https://github.com/touchifyapp/cpvc/actions](https://github.com/touchifyapp/cpvc/workflows/CI/badge.svg)

> Cross platform module for interacting with Audio Devices and handling System Audio.
> Node bindings for the [cpvc](https://crates.io/crates/cpvc) crate.

## Compatibility

- Windows
- Mac OS X
- Linux (PulseAudio only)

> Important<br>
> `cpvc` requires [PulseAudio](https://en.wikipedia.org/wiki/PulseAudio) server to work on Linux.

## Installation

```bash
# npm
npm i @touchifyapp/cpvc

# pnpm
pnpm add @touchifyapp/cpvc

#yarn
yarn add @touchifyapp/cpvc
```

## Usage

```javascript
import { getSoundDevices, getMute, setMute, getSystemVolume, setSystemVolume } from "../index.js";

// get system sound devices names
const devices = getSoundDevices();
console.log(devices); // prints ['devicename']

// check whether the system is muted
const muted = getMute();
console.log("is muted:", muted); // prints is muted: false

// set whether the system is muted
setMute(true);
const muted = getMute();
console.log("is muted:", muted); // prints is muted: true

// get current system volume
const volume = getSystemVolume();
console.log("volume:", volume); // prints volume: 30

// set system volume
setSystemVolume(50);
const volume = getSystemVolume();
console.log("volume:", volume); // prints volume: 50
```

## Contribute

### Prerequistes

- Install the latest `Rust`
- Install `Node.js@20+` which fully supported `Node-API`
- Run `corepack enable`

### Getting started

```bash
# clone repository
git clone https://github.com/touchifyapp/node-cpvc
cd node-cpvc

# install packages
pnpm i

# build and test
pnpm build
pnpm test
```

### Build

After `pnpm build` command, you can see `cpvc.[darwin|win32|linux].node` file in project root.
This is the native addon built from [lib.rs](./src/lib.rs).

### Test

Run `pnpm test` to testing native addon.

> Note: a build is necessary before running `pnpm test` if changes were applied on the rust code.

### CI

With GitHub Actions, each commit and pull request will be built and tested automatically in [`node@20`, `node@22`, `node@24`] x [`macOS`, `Linux`, `Windows`] matrix.

## LICENSE

[MIT](./LICENSE)
