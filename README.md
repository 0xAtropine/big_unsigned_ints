# Big_Unsigned_Ints

![Crates.io](https://img.shields.io/crates/v/big_unsigned_ints) [![Build Status](https://travis-ci.org/0xSilene/big_unsigned_ints.svg?branch=master)](https://travis-ci.org/0xSilene/big_unsigned_ints)

## Description

**Big_Unsigned_Ints** is a crate for Rust that allows you to use large unsigned integers ranging from U256 through U2048. These are implemented using fixed-sized arrays and converts from u8 to u64 types. It also allows you to convert back from u64 to an array of u8s without using unsafe code.

The u8 -> u64 conversion uses mem::transmute with unsafe code to perform the process.

No functions should be used aside from the standard traits From and Into to convert between u8 and u64.

## How To Use

Write the following inside your cargo.toml file under dependencies:

`big_unsigned_ints = "0.1.2"`

Then, you can import it like so:

```rust
extern crate big_unsigned_ints;
```

Please view the tests folder for integration testing and how you can use the library itself.

## Standard Documentation On Usage

### Converting Array of Bytes To U256 Type (Quick Version)

```rust
fn bytes_to_u256 (){
    let x = [243u8;32];
    // Conversion Occurs From Bytes of Array To The Type
    let y = big_unsigned_ints::U256::from(x);
}
```

### Converting Array of Bytes To U256 Type Back To An Array Of Bytes

```rust
fn bytes_to_big (){
    // Create an array of 32x 243u8, or 256bits
    let x = [243u8;32];
    // Convert From the Array of Bytes Into a U256 Type ([u64;4])
    let y = big_unsigned_ints::U256::from(x);
    // Convert Back Into An Array of Bytes
    let b: [u8;32] = y.into();
    // Prints Out The Statements
    println!("{:?}",b)
}
```

## Available Types

* U256 `[u64;4]` | `[u8;32]`

* U384 `[u64;6]` | `[u8;48]`

* U512 `[u64;8]` | `[u8;64]`

* U1024 `[u64;16]` | `[u8;128]`

* U2048 `[u64;32]` | `[u8;256]`

## License

* MIT License

* Apache 2.0
