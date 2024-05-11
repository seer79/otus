use std::fs;
use std::fs::*;
use std::os::*;

fn main() {
    let x = fs::File::open("/tmp/out.txt");
    println!("{:?}", x)
}
