# devtools
A simple cli-tool written in Rust to give you simple information that you can use to help debug binaries. Currently only returns the maximum and minimums for integer and float types. Can also convert decimal numbers to hexadecimal.

## Building from source:
1. run `cargo build --release`
2. extract binary (`devtools`) from target/release

## Usage:
linux: `$ ./devtools help`
powershell: `.\devtools.exe help`

### Showing type limits
To show the type limits for integers and float types, run the command:

`devtools limits <args>`

where `<args>` can be: `--uint --int --float` for each respective datatype.

Passing no arguments shows you the limits for all data types.


### Converting decimal to hex
Supports up to 128-bit unsigned integers. To convert a decimal number:

`devtools hex -d <number_to_be_converted>`

