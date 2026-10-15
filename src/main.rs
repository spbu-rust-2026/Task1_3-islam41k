use std::fs;
use std::io;

fn main() {
    let mut file = String::new();
    io::stdin().read_line(&mut file).unwrap();
    match fs::read(file.trim()) {
        Ok(_) => {
            println!("success");
            return;
        }
        Err(_) => {
            println!("failure");
            return;
        }
    }
}
