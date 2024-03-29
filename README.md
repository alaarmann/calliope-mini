# calliope-mini

_calliope-mini_ contains everything required getting started using Rust to create firmwares for the
[Calliope mini](https://www.calliope.cc) microcontroller board. This little board has a lot built-in,
even a capable debugging interface.

## Important notes

- this project is currently under active development, APIs are expected to change.
- it is a fork of [microbit](https://github.com/nrf-rs/microbit), a Board Support Package (BSB) for the BBC micro:bit
- currently supported is Calliope mini _V1_ exclusively, support for V2 and V3 is planned

## Getting started

All you need to start programming this device is:

- A Calliope mini board
- A computer: Linux is tested
- A bit of open source software

### Install dependencies

On Linux you have the options to use `cargo install` or `nix-shell`.

#### Cargo install

In order to run the examples you need to install [`flip-link`](https://github.com/knurling-rs/flip-link#installation) and [`cargo-embed`](https://probe.rs/docs/tools/cargo-embed/#installation).

```bash
> cargo install flip-link cargo-embed
```

#### Nix

Start a Nix shell in the project's base directory

```bash
> nix-shell
```

### Run an example

The first thing to try is one of the [examples](./examples) in this repository. Plug in your Calliope mini and
run one of the commands below.

_For Calliope mini V1_

```bash
> cargo embed --release --manifest-path ./examples/display-blocking/Cargo.toml --features v1 --target thumbv6m-none-eabi
```

You should see a lot of build output, the orange LED on the back of the micro:bit should flash quickly, and
a message should appear on the LED display.

Congratulations! You've flashed your first Rust program onto your Calliope mini!

## License

[MIT](LICENSE)
