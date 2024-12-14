### devtools
A simple cli-tool written in Rust to give you simple information that you can use to help debug binaries. Currently only returns the maximum and minimums for integer and float types.
Also returns hexadecimal values up to 15. Built this because I kept forgetting these things, and would run into integer overflows. Doesn't do too much at the current moment,
but may have additional features added.

## Building from source:
1. run `cargo build --release`
2. extract binary (`devtools`) from target/release

## Usage:
linux: `$ ./devtools`
powershell: `.\devtools.exe`
