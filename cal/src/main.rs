use std::io;
fn main() {
    loop {
    println!("Enter a first number:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let num1: f64 = a.trim().parse().expect("Please enter a valid number");

    
    println!("Enter a second number:");
    let mut b =String::new();
    io::stdin().read_line(&mut b).unwrap();
    let num2:f64 = b.trim().parse().expect("Please enter a valid number");

    println!("Enter the operator");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).unwrap();
    let operator = operator.trim();


    println!("the result is {}",cal(num1,num2, operator));
    }
   
}

fn cal(x: f64, y: f64, operator: &str) -> f64 {
    if operator == "+" {
        x + y
    } else if operator == "-" {
        x - y
    } else if operator == "*" {
        x * y
    } else if operator == "/" {
        if y == 0.0 {
            println!("Cannot divide by zero!");
            0.0
        } else {
            x / y
        }
    } else {
        println!("Invalid operator!");
        0.0
    }
}

