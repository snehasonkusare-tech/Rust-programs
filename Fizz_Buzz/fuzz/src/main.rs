
fn main() {
    for a in 0..=100{
        if a % 3 == 0{
            println!("Fizz");
        }
        else if a % 5 == 0{
            println!("Buzz");
        }
        else{
            println!("{}", a);
        }
        }
    }
