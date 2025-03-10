# Picoboy hal boards

Rust support for Picoboy boards based on the "Raspberry Silicon" family of microcontrollers.

Based on the [`rp-hal-boards`](https://github.com/rp-rs/rp-hal-boards) project.

## Getting Started

So, you want to program your new Raspberry Silicon microcontroller, using the
Rust programming language. You've come to the right place!

These board support packages are based on
[`rp-hal`](https://github.com/rp-rs/rp-hal) - a collection of high-level
drivers for the Raspberry Silicon RP2040 microcontroller and various
associated boards, like the Raspberry Pi Pico and the Adafruit Feather
RP2040.

If you want to write an application for Raspberry Silicon, check out our
[RP2040 Project Template](https://github.com/rp-rs/rp2040-project-template).

If you want to try out some examples on one of our supported boards, check out
the list of *Board Support Packages* below, and click through to see the various
examples for each board.

Before trying any of the examples, please ensure you have the latest stable
version of Rust installed, along with the right target support:

```sh
rustup self update
rustup update stable
rustup target add thumbv6m-none-eabi
```

You may also want to install these helpful tools:

```sh
# Useful to creating UF2 images for the RP2040 USB Bootloader
cargo install --locked elf2uf2-rs
# Useful for flashing over the SWD pins using a supported JTAG probe
cargo install --locked probe-rs-tools
```

## Packages

This git repository is organised as a [Cargo Workspace].

If you are writing code that should work on any RP2040 device, use
the [HAL crate]. If you are running code on a specific board, use
the appropriate _BSP_ crate (which will include the _HAL_ crate for
you). Please note, you cannot depend on multiple _BSP_ crates; you have
to pick one, or use [Cargo Features] to select one at build time.

Each BSP includes some examples to show off the features of that particular board.

[HAL crate]: https://github.com/rp-rs/rp-hal
[Cargo Workspace]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[Embedded HAL]: https://github.com/rust-embedded/embedded-hal
[Cargo Features]: https://doc.rust-lang.org/cargo/reference/features.html
[rp2040-hal]: https://crates.io/crates/rp2040-hal

### [picoboy] - Board Support for the [Picoboy]

You should include this crate if you are writing code that you want to run on
a [Picoboy] - the original iteration of the Picoboy board.

This crate includes the [rp2040-hal], but also configures each pin of the
RP2040 chip according to how it is connected up on the Picoboy.

[Picoboy]: https://picoboy.de/einfuehrung/das-geraet/
[picoboy]: https://github.com/seeseekey/picoboy-hal-boards/tree/main/boards/picoboy

### [picoboy-color] - Board Support for the [Picoboy Color]

You should include this crate if you are writing code that you want to run on
a [Picoboy Color] - the second iteration of the Picoboy series.

This crate includes the [rp2040-hal], but also configures each pin of the
RP2040 chip according to how it is connected up on the Picoboy Color.

[Picoboy Color]: https://picoboy.de/der-picoboy-color/
[picoboy-color]: https://github.com/seeseekey/picoboy-hal-boards/tree/main/boards/picoboy-color

## Programming

Rust generates standard Arm ELF files, which you can load onto your Raspberry Pi
Silicon device with your favourite Arm flashing/debugging tool. In addition, the
RP2040 contains a ROM bootloader which appears as a Mass Storage Device over USB
that accepts UF2 format images. You can use the `elf2uf2-rs` package to convert
the Arm ELF file to a UF2 format image.

For boards with USB Device support like the Raspberry Pi Pico, we recommend you
use the UF2 process.

The RP2040 contains two Cortex-M0+ processors, which execute Thumb-2 encoded
ARMv6-M instructions. There are no operating-specific features in the binaries
produced - they are for 'bare-metal' systems. For compatibilty with other Arm
code (e.g. as produced by GCC), Rust uses the *Arm Embedded-Application Binary
Interface* standard or EABI. Therefore, any Rust code for the RP2040 should be
compiled with the target *`thumbv6m-none-eabi`*.

More details can be found in the [Project Template](https://github.com/rp-rs/rp2040-project-template).

### Loading a UF2 over USB

*Step 1* - Install [`elf2uf2-rs`](https://github.com/JoNil/elf2uf2-rs):

```console
$ cargo install elf2uf2-rs --locked
```

*Step 2* - Make sure your .cargo/config.toml contains the following (it should by
default if you are working in this repository):

```toml
[target.thumbv6m-none-eabi]
runner = "elf2uf2-rs -d"
```

The `thumbv6m-none-eabi` target may be replaced by the all-Arm wildcard
`'cfg(all(target_arch = "arm", target_os = "none"))'`.

*Step 3* - Boot your RP2040 into "USB Bootloader mode", typically by rebooting
whilst holding some kind of "Boot Select" button. On Linux, you will also need
to 'mount' the device, like you would a USB Thumb Drive.

*Step 4* - Use `cargo run`, which will compile the code and started the
specified 'runner'. As the 'runner' is the elf2uf2-rs tool, it will build a UF2
file and copy it to your RP2040.

```console
$ cargo run --release --example pico_pwm_blink
```

### Loading with probe-rs
[probe-rs](https://github.com/probe-rs/probe-rs) is a library and a
command-line tool which can flash a wide variety of microcontrollers
using a wide variety of debug/JTAG probes. Unlike using, say, OpenOCD,
probe-rs can autodetect your debug probe, which can make it easier to use.

*Step 1* - Install `probe-rs`:

```console
$ cargo install --locked probe-rs-tools
```

Alternatively, follow the installation instructions on https://probe.rs/.

*Step 2* - Make sure your .cargo/config.toml contains the following:

```toml
[target.thumbv6m-none-eabi]
runner = "probe-rs run --chip RP2040"
```

*Step 3* - Connect your USB JTAG/debug probe (such as a Raspberry Pi Pico
running [this firmware](https://github.com/majbthrd/DapperMime)) to the SWD
programming pins on your RP2040 board. Check the probe has been found by
running:

```console
$ probe-rs list
The following debug probes were found:
[0]: J-Link (J-Link) (VID: 1366, PID: 0101, Serial: 000099999999, JLink)
```

There is a SEGGER J-Link connected in the example above - the mesage you see
will reflect the probe you have connected.

*Step 4* - Use `cargo run`, which will compile the code and start the specified
'runner'. As the 'runner' is the `probe-rs` tool, it will connect to the
RP2040 via the first probe it finds, and install your firmware into the Flash
connected to the RP2040.

```console
$ cargo run --release --example pico_pwm_blink
```

### Loading with picotool

As ELF files produced by compiling Rust code are completely compatible with ELF
files produced by compiling C or C++ code, you can also use the Raspberry Pi
tool [picotool](https://github.com/raspberrypi/picotool). The only thing to be
aware of is that picotool expects your ELF files to have a `.elf` extension, and
by default Rust does not give the ELF files any extension. You can fix this by
simply renaming the file.

Also of note is that the special
[pico-sdk](https://github.com/raspberrypi/pico-sdk) macros which hide
information in the ELF file in a way that `picotool info` can read it out, are
not supported in Rust. An alternative is TBC.

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

The steps are:

1. Fork the Project by clicking the 'Fork' button at the top of the page.
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Make some changes to the code or documentation.
4. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
5. Push to the Feature Branch (`git push origin feature/AmazingFeature`)
6. Create a [New Pull Request](https://github.com/seeseekey/picoboy-hal-boards/pulls)
7. An admin will review the Pull Request and discuss any changes that may be required.
8. Once everyone is happy, the Pull Request can be merged by an admin, and your work is part of our project!

## License

The contents of this repository are dual-licensed under the _MIT OR Apache
2.0_ License. That means you can choose either the MIT license or the
Apache-2.0 license when you re-use this code. See `MIT` or `APACHE2.0` for more
information on each specific license.

Any submissions to this project (e.g. as Pull Requests) must be made available
under these terms.

## Contact

Raise an issue: [https://github.com/seeseekey/picoboy-hal-boards/issues](https://github.com/seeseekey/picoboy-hal-boards/issues)

## Acknowledgements

* [Othneil Drew's README template](https://github.com/othneildrew)
* [Rust Embedded Working Group](https://github.com/rust-embedded)
* [Raspberry Pi](https://raspberrypi.org) and the [Pico SDK](https://github.com/raspberrypi/pico-sdk)
