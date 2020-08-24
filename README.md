# About

This repo is part of a larger project called Audio Anywhere(AA). Audio Anywhere a 
framework for working with audio plugins that are compiled once and run anywhere.
At the heart of Audio Anywhere is an audio engine whose Digital Signal Processing (DSP) components are written in Faust and deployed with WebAssembly. 

Details about the project can be found on the [project's homepage](https://muses-dmi.github.io/projects/).

## Introduction

This repo contains a number of AA module examples. 

* Default is a do nothing module that can be loaded, well to do nothing
* Nuke is an analogue synth based on one of the Muses instruments
* VL1 is a fairly close emulation of the [Casio VL-1](https://en.wikipedia.org/wiki/Casio_VL-1). (No GUI for this yet, coming soon.)
* Gain is a simple gain controller
* More in development

## Building

The example audio components are implmented in  [Rust](https://www.rust-lang.org/) and compiled to [WebAssembly](https://webassembly.org/).

They currenlty make use of features only on nightly.

To install Rust go you need simply to install [Rustup](https://rustup.rs/) and 
if you already have Rust installed, then you can update with the command rustup update.

Now you need to install nightly:

```bash
rustup toolchain install nightly
```
Don't forget to update if you already have nightly installed. Now install the WASM target:

```bash
rustup target add wasm32-unknown-unknown --toolchain nightly
```

Each of the examples have a Cargo config that automatically compiles to WASM. (Later sections detail how to create your own example.)

Most interfaces are written in HTML5, with pure Javascript. To build some of the GUIs into a single HTML file [Node.js](https://nodejs.org/en/) is required.

All that said there is no requirement that each module is implemented in the same fashion, and as such to build a particular module see the corresponding readme in its directory.

# Module Bundles

An AA module must consist of the following:

* **module.json** is a description of the module, which is used to load it.
* **module.wasm** is an AA WASM API conforming module that implements the audio part of a module.
* **module.html** (optional) UI implemented in HTML5.

# Deploying a set of Audio Anywhere Modules

TODO

# Adding a Module

TODO

# License
© 2020 [Benedict R. Gaster (cuberoo_)](https://bgaster.github.io/)

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
