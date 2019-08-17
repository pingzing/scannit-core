# scannit-core

This is a library designed to read a V2 [HSL Travel Card](https://dev.hsl.fi/#travel-card). It's heavily based on the [HSL-provided](https://github.com/hsldevcom/hsl-card-java), and an evolution of my V1, C#-based version of the library, [HSLTravelSharp](https://github.com/pingzing/hsl-travel-sharp/).

# Usage

Add the following to your Cargo.toml:

```toml
[dependencies]
scannit-core = "0.1.0"
```

Getting a `TravelCard` object requires communicating via NFC with your physical travel card. An example of doing so is contained in the `scannit-cli` subproject in this repository.

Creating the `TravelCard` object will look something like this:

```rust
use scannit_core::travelcard::create_travel_card;

// function declaration here somewhere...

let app_info: &[u8] = get_app_info_from_nfc_card();
let control_info: &[u8] = get_control_info_from_nfc_card();
let period_pass: &[u8] = get_period_pass_from_nfc_card();
let stored_value: &[u8] = get_stored_value_from_nfc_card();
let e_ticket: &[u8] = get_e_ticket_from_nfc_card();
let all_history : &[u8] = get_history_from_nfc_card();

let travel_card = create_travel_card(
    app_info,
    control_info,
    period_pass,
    stored_value,
    e_ticket,
    all_history,
);
```

This crate also exposes the commands by which you communicate with the NFC card in the `desfire` module.

# FFI

This crate also includes the `scannit-core-ffi` subproject, which contains FFI-friendly projections of the data models in the main crate, as well as FFI-friendly functions that can be used to create (and free) `TravelCard` objects.
See [the ScannitSharp](https://github.com/pingzing/scannitsharp) library for a C# example of using the FFI crate.

## Building the FFI crate

```bash
> cd scannit-core-ffi
> cargo build
```

or just:

```bash
> cargo build --all
```

# Building

`> cargo build`

That's it!

Cross-compilation should work without too much issue, but requires the standard tools for cross-compiling to your target triple. Android requires the NDK and a ~/.cargo/config setup for `ar` and a `linker`, for example.
