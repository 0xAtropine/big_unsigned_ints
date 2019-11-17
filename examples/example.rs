extern crate big_unsigned_ints as bui;

fn main(){
    // [u8;32] -> [u64;4]
    let a = bui::U256::from([0u8;32]);
    // [u64;4] -> [u8;32]
    let b: [u8;32] = a.into();
}