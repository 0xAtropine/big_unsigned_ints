# Big_Unsigned_Ints

## Description

**Big_Unsigned_Ints** is a crate for Rust that allows you to use large unsigned integers ranging from U256 through U2048. These are implemented using fixed-sized arrays and converts from u8 to u64 types. It also allows you to convert back from u64 to an array of u8s without using unsafe code.

The u8 -> u64 conversion uses mem::transmute with unsafe code to perform the process.

## How To Use

Write the following inside your cargo.toml file under dependencies:

`big_unsigned_ints = "0.1.0"`

Then, you can import it like so:

...

## License

* MIT License
