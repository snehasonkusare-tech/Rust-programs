use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::env;

const FILE_PATH: &str = "note.json";
#[derive(Serialize, Deserialize)]
struct Note {
    id: u32,
    content: String,
}
// read the file
fn read_notes() -> Vec<Note> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();
    }
    let mut file = File::open(FILE_PATH).expect("Failed to open the file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Failed to read the file");
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

// write to the file
fn write_notes(notes: &Vec<Note>) {
    let data = serde_json::to_string_pretty(notes).expect("Failed to serialize notes");
    let mut file = File::create(FILE_PATH).expect("failed to create file");
    file.write_all(data.as_bytes()).expect("failed to write file");
}

//main logic

fn add(content: String) {
    let mut notes = read_notes();
    let id: u32 = (notes.len() as u32) + 1;
    notes.push(Note { id, content });
    write_notes(&notes);
    println!("Note added successfully");
}

fn delete(id: u32) {
    let mut notes = read_notes();
    let initial_len = notes.len();
    notes.retain(|note| note.id != id);
    if notes.len() < initial_len {
        write_notes(&notes);
        println!("Note deleted successfully");
    } else {
        println!("Note with given id not found");
    }
}

fn list_notes() {
    let notes = read_notes();
    if notes.is_empty() {
        println!("no Notes found");
    } else {
        println!("your notes:");
        for note in notes {
            println!("{}:{}", note.id, note.content);
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage:");
        println!("add <note_content> - Add a new note");
        println!("delete <note_id> - Delete a note by id");
        println!("list - List all notes");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("please provide note content");
            } else {
                let content = args[2..].join(" ");
                add(content);
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("please provide note id");
            } else {
                match args[2].parse::<u32>() {
                    Ok(id) => delete(id),
                    Err(_) => println!("invalid id"),
                }
            }
        }
        "list" => {
            list_notes();
        }
        _ => {
            println!("unknown command");
        }
    }
}

