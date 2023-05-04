# Pico-Gen

A simple (albeit slightly hacky) project generator for the Raspberry Pi Pico using the C SDK.

## Description

`pico-gen` will generate a new project template with an appropriate `CMakeLists.txt` and a `justfile` for ergonomic builds and uploads. Using [just](https://github.com/casey/just) is completely optional. If you don't wish to install it, simply delete the `justfile` in the project root.

## Getting Started

### Dependencies

* [Pico C SDK](https://github.com/raspberrypi/pico-sdk) (and its dependencies, such as CMake)
* [Just](https://github.com/casey/just) (optional)

### Installing

Clone this repo and run `cargo install --path .` in the repo root. Currently there are no pre-built binaries and currently no plans to offer such. If you wish to package for your distro of choice, you have my blessing to do so.

### Usage

`pico-gen new` will create a new directory and generate a project inside it.
`pico-gen init` will generate a project inside the current directory

From nothing to building the included source template (using Just) looks like this:
```
$ pico-gen new blink
$ cd blink
$ just build
```

If you don't wish to use Just, that would look like this:
```
$ pico-gen new blink
$ cd blink
$ cmake .
$ make
```

The included `justfile` has two named commands, `build`, and `load`. Build does what you probably expect. Load will attempt to upload the built project to a connected Pico in BOOTSEL mode and then reboot the device. Running `just` without a command will run `build` and `load` in that order by default.

## Help

Pico-gen is a pretty simple tool, but it does include basic help as one might expect.
```
Usage: pico-gen <COMMAND>

Commands:
  new   Create a new project in a new directory
  init  Initialize a new project in the current directory
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## License

This project is licensed under the MIT License - see the LICENSE.md file for details
