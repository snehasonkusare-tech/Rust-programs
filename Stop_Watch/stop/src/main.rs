use std::io::{self,Write};
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let seconds: u64 = match parse_input_to_seconds(&input) {
        Some(s) if s > 0 => s,
        _ => {
            println!("Invalid input. Please give seconds (e.g., 45) or MM:SS (e.g., 01:30).");
            return;
        }
    };

    let start = Instant::now();
    let mut remaining = seconds;
    println!("time left : {}", format_mm_ss(remaining));
    io::stdout().flush().expect("Failed to flush stdout");

    while remaining > 0 {
        thread::sleep(Duration::from_secs(1));
        let elapsed = start.elapsed().as_secs();
        if elapsed >= seconds {
            remaining = 0;
        }else{
            remaining = seconds - elapsed;
        }

        println!("\rTime left: {}", format_mm_ss(remaining));
        io::stdout().flush().expect("Failed to flush stdout");
    }
    println!("\nTime's up!");
    
}

fn parse_input_to_seconds(input: &str) -> Option<u64> {
    let s  = input.trim();
    if s.contains(':'){
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 2 {
            let minutes:u64 =parts[0].parse::<u64>().ok()?;
            let seconds:u64 =parts[1].parse::<u64>().ok()?;
            Some(minutes * 60 + seconds)
        }
        else{
            None
        }
    } else{
        s.parse::<u64>().ok()
    }
    }

    fn format_mm_ss(total_secs:u64) ->String{
        let mins =total_secs /60;
        let secs = total_secs %60;
        format!("{:02}:{:02}",mins,secs)
    }