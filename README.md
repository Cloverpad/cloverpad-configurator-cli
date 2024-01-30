# Cloverpad - CLI Configurator

This repository contains a CLI configurator application for the [Cloverpad](https://github.com/Cloverpad), a 3-key keypad for osu! themed after [MORE MORE JUMP!](https://www.sekaipedia.org/wiki/MORE_MORE_JUMP!) from Project Sekai.

## Usage

**WIP**

On Linux, `udev` rules also need to be setup to grant user access to the relevant keypads.

- Download a copy of [`99-cloverpad.rules`](./udev/99-cloverpad.rules)
- Move this into `/etc/udev/rules.d`
- Restart `udev`: `sudo udevadm control --reload-rules && sudo udevadm trigger`

## Setting Up Development Environment

The CLI configurator is a Rust project. Make sure you have the following things installed:

- [Stable Rust](https://www.rust-lang.org/learn/get-started) (tested on 1.74.0)
- On Linux, a few other dependencies are needed:
  - [serialport-rs Dependencies](https://github.com/serialport/serialport-rs#dependencies)

Clone this repository including submodules:

```bash
git clone --recurse-submodules https://github.com/Cloverpad/cloverpad-configurator-cli.git
```

Then use one of the following commands to run the project:

```bash
# Interactive Shell
cargo run             # Run interactive shell in debug mode
cargo run --release   # Run interactive shell in release mode
```

## License

This project is licensed under [GPL-3.0](./LICENSE).
