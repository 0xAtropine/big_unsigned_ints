extern crate big_unsigned_ints as bui;

#[test]
fn instantiate_u256(){
    let x = bui::U256([86557u64;4]);
    println!("{}",x);
}
#[test]
fn instantiate_u384(){
    let x = bui::U384([393283u64;6]);
    println!("{}",x)
}
#[test]
fn bytes_to_u256 (){
    let x = [243u8;32];
    let y = bui::U256::from(x);
    println!("{}",y)
}
#[test]
fn u256_into_bytes (){
    let x = [243u8;32];
    let y = bui::U256::from(x);
    let b: [u8;32] = y.into();
    println!("{:?}",b)
}