use std::env;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

#[derive(Debug)]
struct Stats{
    count: usize,
    sum:f64,
    average:f64,
    min:f64,
    max:f64,
}

impl Stats {
    fn from_values(values:&[f64])->Option<Stats>{
        if values.is_empty(){
            return None;
        }
        let count=values.len();
        let sum: f64=values.iter().sum();
        let average=sum/count as f64;
        let min=values.iter().cloned().fold(f64::INFINITY,f64::min);
        let max=values.iter().cloned().fold(f64::NEG_INFINITY,f64::max);
        Some(Stats{
            count,
            sum,
            average,
            min,
            max,
        })
    }
}

fn read_grades_from_csv(path: &str, col_index: usize) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut grades = Vec::new();

    for result in rdr.records() {
        let record = result?;
        if let Some(grade_str) = record.get(col_index) {
            match grade_str.trim().parse::<f64>() {
                Ok(num) => grades.push(num),
                Err(_) => eprintln!("Warning: could not parse grade: {}", grade_str),
            }
        }
    }
    Ok(grades)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args:Vec<String> = env::args().collect();
    let csv_path =args.get(1).map(|s| s.as_str()).unwrap_or("grades.csv");
    let grade_col_index=args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1);
    let grades=read_grades_from_csv(csv_path, grade_col_index)?;
    if grades.is_empty(){
        println!("No valid grades found in the CSV file.");
        return Ok(());  
    }
    if let Some(stats) = Stats::from_values(&grades) {
        println!("Csv Grade stats: {}", csv_path);
        println!("Count: {}", stats.count);
        println!("Sum: {:.2}", stats.sum);
        println!("Average: {:.2}", stats.average);
        println!("Min: {:.2}", stats.min);
        println!("Max: {:.2}", stats.max);
        Ok(())
    } else {
        Err("Failed to calculate statistics from grades.".into())
    }
    }
    
    
