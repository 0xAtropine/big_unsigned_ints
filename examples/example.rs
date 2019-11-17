extern crate big_unsigned_ints as bui;

fn main(){
    // [u8;32] -> [u64;4]
    let a = bui::U256::from([0u8;32]);

    // Prints as Hexadecimal
    println!("{}",a);
    
    // [u64;4] -> [u8;32]
    let _b: [u8;32] = a.into();
}