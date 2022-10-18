use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let mut rng = rand::thread_rng();
    let mut input = String::new();
    let out;

    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            out = input.trim();
            println!("{n} bytes read");
            println!("'{out}'");
        }
        Err(error) => println!("error: {error}"),
    }
    
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));

    if let result = out.cmp(rng){
        println!("'{result}'");
    }
}