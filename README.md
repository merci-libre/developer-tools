# devtools
A simple cli-tool written in Rust to give you simple information that you can use to help debug binaries. Currently only returns the maximum and minimums for integer and float types. Can also convert decimal numbers to hexadecimal.

## Requirements:
- Rust (duh.)
- 64-bit Processor, untested on ARM devices and 32-bit systems.

## Installation:
**Windows (64-bit)**: Check releases for most current compiled binary.
**Linux**: binaries are also in releases,

#### From Source
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

### Shellcode generation
Can generate raw Hexvalues for characters for executing shellcode.
`devtools hex -xs "[STRING TO CONVERT]"`
also includes a rotational argument for ROT-13 like string manipulation.

### Chmod octal value conversion
`devtools` also features a chmod file conversion toolkit that can parse through a string such as:

`rwx-wx-wx` -> `733` << and return the chmod octal value

It also provides you with a description of the corresponding permissions based on the inputted string.

*From our previous example:*
```
# Sample output
Permissions String:
rwx-wx-wx
Chmod Octal Value: 733

All members of a group have the following permissions:
read: false
write: true
execute: true

The owner has the following permissions:
read: true
write: true
execute: true

All users have the following permissions:
read: false
write: true
execute: true
```
