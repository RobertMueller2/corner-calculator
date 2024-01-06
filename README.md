# Corner Calculator
## Overview

Corner Calculator is a library with command-line utility written in Rust, designed to calculate new x and y coordinates based on a direction key (1-9 keypad style) and screen and window dimensions. It can be useful for window management scripts where you need to position windows in different corners or sides of the screen.

## Features

- Calculate new coordinates based on screen and window dimensions.
- Supports keypad-style direction input (1-9).
- Implementations for multiple numeric types (u16, u32, i16, i32) when using as library.

## Requirements

- Rust programming environment.

## Installation

Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/RobertMueller2/corner-calculator.git
cd corner-calculator
cargo build --release
```

## Usage
### Command-line utility

To use Corner Calculator, run the executable with the following arguments:

```sh
corner-calculator <keypad direction|1-9> <screen width|0-65535> <screen height|0-65535> <window width|0-65535> <window height|0-65535>
```

Example:

```sh
corner-calculator 5 1920 1080 500 300
```

### Library

The crate is on crates.io.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Credits

README kindly provided by ChatGPT. ;)
