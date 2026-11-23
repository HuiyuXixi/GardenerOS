#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("==========================================");
    println!("This is my own application in exp3!");
    println!("Student ID: 24301031, Name: Xianghong Fei");
    println!("The Trap mechanism works perfectly!");
    println!("==========================================");
    0
}
