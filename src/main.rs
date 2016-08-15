use std::io;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}