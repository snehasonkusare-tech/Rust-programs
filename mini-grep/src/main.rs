use std::env;
use std::fs;
use std::io
fn main() {
    //collect the comand-line arguments in vector format
    let args : Vec<String> = env::args().collect();

    //let mut query = String::new();
    //io::stdio.read_line(&mut query).expect("failed to read input")
     //let query = query.trim();

    //take the second and the third arguments from the command line:
    let query = &args[1];
    let filename = &args[2];

    //read the whole file to the string
    let contents = fs::read_to_string(filename).expect("something went wrong reading file name");

    //iterating over line and returing those that contains file
    for line in contents.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
}
