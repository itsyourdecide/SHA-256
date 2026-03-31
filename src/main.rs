mod sha256;
use crate::sha256::{Sha256, to_hex};

pub fn test() {
    let mut data = String::new(); 
    println!("Enter a number (u128):");
    std::io::stdin().read_line(&mut data).expect("Failed to read line");
    let data: u128 = data.trim().parse().expect("Invalid input");
    
    let hash = Sha256::digest(data);
    
    println!("hash: {}", to_hex(&hash));
}

fn main() {
    test();
}