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


### Converting decimal to hex (and vice versa)
Supports up to 128-bit unsigned integers. To convert a decimal number:

`devtools hex -d <number_to_be_converted>`

As of version 1.0.0, you can also convert a number back into decimal form. You can do this with the new `decimal` command.

`devtools decimal -t 0x32` OR: `devtools decimal -t 32`

both will output '50', as the prefix is stripped prior to being entered into the function.

