# Embassy RP Skeleton
## Summary
This project skeleton is intended as a starting point for developing your own firmware for the [`rp2040`][1] based on
the [`embassy`][2] asynchronous embedded development framework.

It includes all of the [`knurling-rs`][3] tooling ([`defmt`][4], [`defmt-rtt`][4], [`panic-probe`][4], [`flip-link`][5],
[`probe-run`][6]) to improve the development process.

The default [`Cargo`][7] runner is configured as [`probe-run`][6], so you can build, flash and run your firmware _with_
output from the device via RTT with:

```shell
cargo run --release
```

If you want to use a different runner with your debugger (e.g., [`cargo-embed`][8], [`probe-rs-debugger`][9], etc.) or
if you _aren't_ using a debugger and want the runner to flash the firmware via USB (e.g., [`elf2uf2-rs`][10],
[`picotool`][11], etc.) then see the [Runners](#runners) section.

## Table of Contents
1. [Requirements](#requirements)
2. [Setup](#setup)
3. [Usage](#usage)
4. [Runners](#runners)
5. [Appendix](#appendix)

## Requirements
* Rust
    - Rustup
    - Cargo
* Toolchain Support
    - Nightly
    - Cortex Target
* Flip-Link
* Probe-Run
* CMSIS-DAP Probe

## Setup
...

## Usage
...

## Runners
...

## Appendix
...

<!-- Reference -->
[1]: https://www.raspberrypi.com/documentation/microcontrollers/rp2040.html
[2]: https://embassy.dev/dev/index.html
[3]: https://github.com/knurling-rs/app-template
[4]: https://github.com/knurling-rs/defmt
[5]: https://github.com/knurling-rs/flip-link
[6]: https://github.com/knurling-rs/probe-run
[7]: https://doc.rust-lang.org/cargo/
[8]: https://github.com/probe-rs/cargo-embed
[9]: https://github.com/probe-rs/vscode
[10]: https://github.com/JoNil/elf2uf2-rs
[11]: https://github.com/raspberrypi/picotool
