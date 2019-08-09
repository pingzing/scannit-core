# scannit-core

This is a library designed to read a V2 [HSL Travel Card](https://dev.hsl.fi/#travel-card). It's heavily based on the [HSL-provided](https://github.com/hsldevcom/hsl-card-java), and an evolution of my V1, C#-based version of the library, [HSLTravelSharp](https://github.com/pingzing/hsl-travel-sharp/).

# Usage

TODO, publish this sucker to crates.io.

# Building

`> cargo build`

That's it!

Cross-compilation should work without too much issue, but requires the standard tools for cross-compiling to your target triple. Android requires the NDK and a ~/.cargo/config setup for `ar` and a `linker`, for example.
