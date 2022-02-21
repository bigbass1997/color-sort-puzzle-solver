[![License: BSD 2-Clause](https://img.shields.io/badge/License-BSD%202--Clause-blue)](LICENSE)
### Description
Uses a screenshot from each level in the "Water Sort - Color Puzzle Game" mobile app, and provides a step by step solution to the level.

Unknown/hidden colors are not supported. Image parsing has only been tested on a Galaxy S10+.

Made this solver as a fun little challenge after hearing about it from a friend. The app itself definitely is _not_ worth playing, but it presents interesting challenges in image recognition and sorting. 

### Building
If you wish to build from source, for your own system, Rust is integrated with the `cargo` build system. To install Rust and `cargo`, just follow [these instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html). Once installed, while in the project directory, run `cargo build --release` to build, or use `cargo run --release` to run directly. The built binary will be available in `./target/release/`

To cross-compile builds for other operating systems, you can use [rust-embedded/cross](https://github.com/rust-embedded/cross).