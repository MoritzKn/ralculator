# Ralculator [![Build Status](https://travis-ci.org/MoritzKn/ralculator.svg?branch=master)](https://travis-ci.org/MoritzKn/ralculator)

Ralculator is a simple calculator [GTK](http://www.gtk.org/) desktop application
written in [Rust](https://www.rust-lang.org/) using [GTK-RS](http://gtk-rs.org/).

![a screenshot showing the calculator application](doc/img/screenshot.png)

## Build instructions
1. Download the source code from [here](https://github.com/MoritzKn/ralculator/releases)
   or clone the git project.
2. Download the dependencies and complie the source code using
   [Cargo](https://crates.io/install).

After cargo is done, you can find the binaries under`./target/release/ralculator`.

```sh
git clone https://github.com/MoritzKn/ralculator.git
cd ralculator
cargo build --release
./target/release/ralculator
```

Alter that you can optionally add the binaries to your `$PATH` and copy
the `ralculator.desktop` file to `/usr/share/applications`. To do so execute:
```sh
./install.sh
```

## License
This project is licensed under the terms of the MIT license.
A copy of the license can be found in the root directory of
the project in the file [LICENSE](./LICENSE).
