// Crate-Name: big-unsigned-ints
// Author: Joseph Paul Tortorelli | 0xSilene
// Github: 0xSilene | 228Labs
// Version: 1.0.0
// Date: 4 Sept 2019

/*
|-----METADATA-----|
Rustc Version Tested: 1.37.0 Stable / 1.39.0 Nightly

|-----LICENSE-----|
MIT License

Copyright (c) [2019] [Joseph Paul Tortorelli]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


|-----INFORMATION-----|
Description:
    A Simple Rust Library with no dependencies that allows you to use larger unsigned integers as tulpe structs made up as fixed-length u64 arrays
Notes:
    - This Library is !#[no_std]
    - The Structs all use differing length arrays of u64 primitives while all being able to implement the standard traits by keeping the array size under 32
    - By Default, the traits Clone and Copy are not implemented. Only Debug is implemented.
Warnings:
    - An Unsafe Block is Used In Each Of The from_u8_byte_array_to_<struct>() functions because it treats an array of u8s as an array of u64s using the transmute memory function
Development Notes:
    - Implement conversion from U<bits> to u8 byte array
        - Add safety check for type as types are not checked in args
    - Implementing the array len check in the unsafe block may not be necessary; Look into some more
    - Write some more tests to check for failures

On Transmute:
    Transmute will check the length of the two values but NOT the type.
*/

#![no_std]
use core::mem::transmute;
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct U256 (pub[u64;4]); // 256 bits (32 bytes)

#[derive(Debug, Clone, Copy)]
pub struct U384 (pub[u64;6]); // 384 bits (48 bytes)

#[derive(Debug, Clone, Copy)]
pub struct U512 (pub [u64;8]); // 512 bits (64 bytes)

#[derive(Debug, Clone, Copy)]
pub struct U1024 (pub [u64;16]); // 1024 bits (128 bytes)

#[derive(Debug, Clone, Copy)]
pub struct U2048 (pub [u64;32]); // 2048 bits (256 bytes)

impl U256 {
    // If Needed, a Simple Method To Initialize an empty U256 struct, which its member can be accessed like so U256::init().0
    pub fn init () -> U256 {
        return U256([0u64;4]);
    }
    // UNSAFE | Wrapper For Main From Function For Easier Usage / Synatx
    pub fn convert_from_bytes (bytes: [u8;32]) -> [u64;4] {
        return U256::from(bytes).0;
    }
    // SAFE | Wrapper For Main Into Function For Easier Usage / Synatx
    pub fn convert_to_bytes (largebytes: [u64;4]) -> [u8;32] {
        let output: [u8;32] = U256(largebytes).into();
        return output;
    }
}
impl From<[u8;32]> for U256 {
    fn from(byte_array: [u8;32]) -> Self {
        unsafe {
            return transmute::<[u8;32], U256>(byte_array);
        }
    }
}
impl Into<[u8;32]> for U256 {
    fn into(self) -> [u8;32] {
        let U256(array) = self;
        let mut output: [u8;32] = [0;32];
        let mut counter: usize = 0;

        for &x in array.iter() {
            let bytes = x.to_be_bytes();
                for &byte in bytes.iter() {
                    output[counter] = byte;
                    counter += 1;
                }
        }
        return output;
    }
}
impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = self.0;
        
        // i for index and max - 1 so we can return the final write!
        let mut i: usize = 0;
        let max = x.len() - 1;
        //write!(f, "{:016X}", x[i]);
        while i < max {
            i += 1;
            write!(f, "0x{:X} ", x[i]);
        }
        write!(f, "0x{:X}", x[i])
    }
}

impl U384 {
    // Function That Sets 6 Array Elements to 0u64
    pub fn new () -> U384 {
        let x = U384([0u64;6]);
        return x;
    }
    // Function That Takes an Array of 48 u8s as input and outputs the tulpe struct U384, or [u64;6]
    pub fn from_u8_byte_array_to_u384 (input: [u8;48]) -> U384 {
        if input.len() == 48 {
            unsafe {
                // Requires the type (u8) and an array of 48 elements and is required to return [u64;6]
                return transmute::<[u8;48], U384>(input);
            }
        }
        else {
            panic!("Byte Array Not Size of 48 for U384");
        }
    }
}
impl From<[u8;48]> for U384 {
    fn from(byte_array: [u8;48]) -> Self {
        unsafe {
            return transmute::<[u8;48], U384>(byte_array);
        }
    }
}
impl Into<[u8;48]> for U384 {
    fn into(self) -> [u8;48] {
        let U384(array) = self;
        let mut output: [u8;48] = [0;48];
        let mut counter: usize = 0;

        for &x in array.iter() {
            let bytes = x.to_be_bytes();
                for &byte in bytes.iter() {
                    output[counter] = byte;
                    counter += 1;
                }
        }
        return output;
    }
}
impl fmt::Display for U384 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = self.0;
        
        // i for index and max - 1 so we can return the final write!
        let mut i: usize = 0;
        let max = x.len() - 1;
        //write!(f, "{:016X}", x[i]);
        while i < max {
            i += 1;
            write!(f, "0x{:X} ", x[i]);
        }
        write!(f, "0x{:X}", x[i])
    }
}

impl U512 {
    // Function That Sets 8 Array Elements to 0
    pub fn new () -> U512 {
        let x = U512([0u64;8]);
        return x;
    }
    // Function That Takes an Array of 64 u8s as input and outputs the tulpe struct U512, or [u64;8]
    pub fn from_u8_byte_array_to_u512 (input: [u8;64]) -> U512 {
        if input.len() == 64 {
            unsafe {
                // Requires the type (u8) and an array of 68 elements and is required to return [u64;8]
                return transmute::<[u8;64], U512>(input);
            }
        }
        else {
            panic!("Byte Array Not Size of 64 for U512");
        }
    }
}
impl From<[u8;64]> for U512 {
    fn from(byte_array: [u8;64]) -> Self {
        unsafe {
            return transmute::<[u8;64], U512>(byte_array);
        }
    }
}
impl Into<[u8;64]> for U512 {
    fn into(self) -> [u8;64] {
        let U512(array) = self;
        let mut output: [u8;64] = [0;64];
        let mut counter: usize = 0;

        for &x in array.iter() {
            let bytes = x.to_be_bytes();
                for &byte in bytes.iter() {
                    output[counter] = byte;
                    counter += 1;
                }
        }
        return output;
    }
}
impl fmt::Display for U512 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = self.0;
        
        // i for index and max - 1 so we can return the final write!
        let mut i: usize = 0;
        let max = x.len() - 1;
        //write!(f, "{:016X}", x[i]);
        while i < max {
            i += 1;
            write!(f, "0x{:X} ", x[i]);
        }
        write!(f, "0x{:X}", x[i])
    }
}
impl U1024 {
    // Function That Sets 16 Array Elements to 0
    pub fn new () -> U1024 {
        let x = U1024([0u64;16]);
        return x;
    }
    // Function That Takes an Array of 128 u8s as input and outputs the tulpe struct U1024, or [u64;16]
    pub fn from_u8_byte_array_to_u1024 (input: [u8;128]) -> U1024 {
        if input.len() == 128 {
            unsafe {
                // Requires the type (u8) and an array of 128 elements and is required to return [u64;16]
                return transmute::<[u8;128], U1024>(input);
            }
        }
        else {
            panic!("Byte Array Not Size of 128 for U1024");
        }
    }
}
impl From<[u8;128]> for U1024 {
    fn from(byte_array: [u8;128]) -> Self {
        unsafe {
            return transmute::<[u8;128], U1024>(byte_array);
        }
    }
}
impl Into<[u8;128]> for U1024 {
    fn into(self) -> [u8;128] {
        let U1024(array) = self;
        let mut output: [u8;128] = [0;128];
        let mut counter: usize = 0;

        for &x in array.iter() {
            let bytes = x.to_be_bytes();
                for &byte in bytes.iter() {
                    output[counter] = byte;
                    counter += 1;
                }
        }
        return output;
    }
}
impl fmt::Display for U1024 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = self.0;
        
        // i for index and max - 1 so we can return the final write!
        let mut i: usize = 0;
        let max = x.len() - 1;
        //write!(f, "{:016X}", x[i]);
        while i < max {
            i += 1;
            write!(f, "0x{:X} ", x[i]);
        }
        write!(f, "0x{:X}", x[i])
    }
}
impl U2048 {
    // Function That Sets 32 Array Elements to 0
    pub fn new () -> U2048 {
        let x = U2048([0u64;32]);
        return x;
    }
    // Function That Takes an Array of 256 u8s as input and outputs the tulpe struct U1024, or [u64;32]
    pub fn from_u8_byte_array_to_u2048 (input: [u8;256]) -> U2048 {
        if input.len() == 256 {
            unsafe {
                // Requires the type (u8) and an array of 256 elements and is required to return [u64;32]
                return transmute::<[u8;256], U2048>(input);
            }
        }
        else {
            panic!("Byte Array Not Size of 256 for U2048");
        }
    }
}
impl From<[u8;256]> for U2048 {
    fn from(byte_array: [u8;256]) -> Self {
        unsafe {
            return transmute::<[u8;256], U2048>(byte_array);
        }
    }
}
impl Into<[u8;256]> for U2048 {
    fn into(self) -> [u8;256] {
        let U2048(array) = self;
        let mut output: [u8;256] = [0;256];
        let mut counter: usize = 0;

        for &x in array.iter() {
            let bytes = x.to_be_bytes();
                for &byte in bytes.iter() {
                    output[counter] = byte;
                    counter += 1;
                }
        }
        return output;
    }
}
impl fmt::Display for U2048 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = self.0;
        
        // i for index and max - 1 so we can return the final write!
        let mut i: usize = 0;
        let max = x.len() - 1;
        //write!(f, "{:016X}", x[i]);
        while i < max {
            i += 1;
            write!(f, "0x{:X} ", x[i]);
        }
        write!(f, "0x{:X}", x[i])
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_structs_zero() {
        assert_eq!(U256::new().0,[0u64;4]);
        assert_eq!(U384::new().0,[0u64;6]);
        assert_eq!(U512::new().0,[0u64;8]);
        assert_eq!(U1024::new().0,[0u64;16]);
        assert_eq!(U2048::new().0,[0u64;32]);
    }
    #[test]
    fn check_from_byte_array_conversion(){
        assert_eq!(U256::from_u8_byte_array_to_u256([0xFFu8;32]).0,[0xFFFFFFFFFFFFFFFFu64;4]);
        assert_eq!(U384::from_u8_byte_array_to_u384([0xFFu8;48]).0,[0xFFFFFFFFFFFFFFFFu64;6]);
        assert_eq!(U512::from_u8_byte_array_to_u512([0xFFu8;64]).0,[0xFFFFFFFFFFFFFFFFu64;8]);
        assert_eq!(U1024::from_u8_byte_array_to_u1024([0xFFu8;128]).0,[0xFFFFFFFFFFFFFFFFu64;16]);
        assert_eq!(U2048::from_u8_byte_array_to_u2048([0xFFu8;256]).0,[0xFFFFFFFFFFFFFFFFu64;32]);
    }
    #[test]
    fn check_to_byte_array_conversion(){
        //assert_eq!(U256::from_u256_to_u8_byte_array(U256([0xFFFFFFFFFFFFFFFFu64;4])),[0xFFu8;32]);
        assert_eq!(U256::from_u256_to_byte_array(U256([0xFFFFFFFFFFFFFFFFu64;4])),[0xFFu8;32]);
    }

}
*/