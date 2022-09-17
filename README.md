# Embassy RP Skeleton
## Summary
The **Embassy RP Skeleton** project template is intended as a starting point for developing your own firmware for the
[`rp2040`][1] based on the [`embassy`][2] asynchronous embedded development framework for [Rust][12].

It includes all of the [`knurling-rs`][3] tooling ([`defmt`][4], [`defmt-rtt`][4], [`panic-probe`][4], [`flip-link`][5],
[`probe-run`][6]) to improve the development process.

The default [`cargo`][7] runner is configured as [`probe-run`][6], so you can build, flash and run your firmware _with_
output from the device via RTT with:

```shell
$ cargo run --release
```

If you want to use a different runner with your debugger (e.g., [`cargo-embed`][8], [`probe-rs-debugger`][9], etc.) or
if you _aren't_ using a debugger and want the runner to flash the firmware via USB (e.g., [`elf2uf2-rs`][10],
[`picotool`][11], etc.) then see the [Runners](#runners) section.

## Table of Contents
1. [Requirements](#requirements)
2. [Setup](#setup)
  1. [System Setup](#system-setup)
  2. [Probe Setup](#probe-setup)
  3. [Hardware Setup](#hardware-setup)
3. [Usage](#usage)
4. [Runners](#runners)
5. [Appendix](#appendix)

## Requirements
* Ubuntu
* Raspberry Pi Pico
* CMSIS-DAP Debugger Probe
* [Rust][12]
* Rust Toolchain ([`cargo`][7], [`rustup`][13])
* Rust Cortex-M Target Toolchain Support (`thumbv6m-none-eabi`)
* Rust Embedded Tooling ([`probe-run`][6], [`flip-link`][5], etc.)

## Setup
### System Setup
1. Install [Rust][12] and [`cargo`][7] using [`rustup`][13]
```shell
# Install `rustup` for Rust Toolchain
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Cortex-M Target Toolchain Support for [`Rust`][12]
```shell
# Install `thumbv6m-none-eabi` Target for `rp2040`
$ rustup target add thumbv6m-none-eabi
```

3. Install [`probe-run`][6]
```shell
# Install Linux Dependencies
$ sudo apt install -y libusb-1.0-0-dev libudev-dev

# Install `probe-run`
$ cargo install probe-run

# (Optional) Install `udev` Rules and Reload
$ sudo curl https://probe.rs/files/69-probe-rs.rules -o /etc/udev/rules.d/69-probe-rs.rules
$ sudo udevadm control --reload
$ sudo udevadm trigger

# (Optional) Add User to `plugdev` Group
$ sudo usermod -aG plugdev $USER
```

4. Install [`flip-link`][5]
```shell
# Install `flip-link`
$ $ cargo install flip-link
```

### Probe Setup
Any CMSIS-DAP compatible debugger probe can be used with [`probe-run`][6].
A short list of compatible debug probes is available here: [Debug Probes][15].

You can also use a second Rasperry Pi Pico as your debugger probe.
1. Download CMSIS-DAP debugger firmware [`DapperMime`][14] for the Raspberry Pi Pico
2. Boot the Raspberry Pi Pico in "Bootloader Mode" by holding the _BOOTSEL_ button while plugging it in
3. Open the mounted Raspberry Pi Pico storage device
4. Copy the `raspberry_pi_pico-DapperMime.uf2` onto the Raspberry Pi Pico
5. Firmware will be flashed to the Raspberry Pi Pico and it will disconnect

### Hardware Setup
**TODO**
  - Connecting the debugger
  - Raspberry Pi Pico development board

## Usage
To run the firmware in debug mode:
```shell
$ cargo run
```

To run the firmware in release mode:
```shell
$ cargo run --release
```

To change the default [`defmt`][4] log level, see `.cargo/config.toml`:
```toml
[env]
DEFMT_LOG = "trace"
```

You can also set the log level inline:
```shell
$ DEFMT_LOG=debug cargo run
$ DEFMT_LOG=error cargo run --release
```

## Runners
**TODO**
  - Using other runners

## Appendix
**TODO**
  - Links, resources, references and guides

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
[12]: https://www.rust-lang.org/
[13]: https://rustup.rs/
[14]: https://github.com/majbthrd/DapperMime
[15]: https://github.com/rp-rs/rp2040-project-template/blob/main/debug_probes.md

<!-- Other Stuff -->
<!--
https://github.com/rust-embedded/awesome-embedded-rust
https://github.com/rp-rs/rp2040-project-template
https://timsavage.github.io/rpi-pico-devboard/
https://github.com/embassy-rs/embassy
https://probe.rs/docs/getting-started/probe-setup/
https://embedded-trainings.ferrous-systems.com/
https://docs.rust-embedded.org/book/
https://github.com/ferrous-systems/teaching-material
https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry
-->
