use std::io;
use std::fs;

fn main() {
    let mut file = String::new();
    io::stdin().read_line(&mut file).unwrap();
    match fs::read_to_string(file.trim()) {
        Ok(_) => {
            println!("success");
            return
        }
        Err(_) => {
            println!("failure");
            return
        }

    }
}
