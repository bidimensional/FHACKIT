use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    let executable_path =  "stuff/hello";
    let f = fs::read(executable_path.to_string());
    println!("{:X?}", &f.unwrap()[..5]);
}

