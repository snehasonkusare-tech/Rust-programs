use std::fs;
use std::io;
fn main() {
    println!("Enter the file path:");
    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("Failed to read line");
    let file_path =path.trim();

    let contents =fs::read_to_string(file_path);

    match contents {
        Ok(data) => println!("File contents:\n{}", data),
        Err(e) => eprintln!("Error reading file: {}", e),
    }

}
