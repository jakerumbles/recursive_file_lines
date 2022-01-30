// Author: Jake Edwards
// For: Composable Questionaire
// Problem #4

use regex::Regex;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Path to directory to start recursive search from
    let path = String::from("./example_folder_structure");
    // File extension to search for
    let file_extension = String::from("txt");

    let _ = recursive_line_find(path, file_extension);
}

/// Given a `dir_path` to start from and a `file_extension` to search for, this function recursively finds all files in a file tree with the given extension and prints out the # of lines in the respective file.
fn recursive_line_find(dir_path: String, file_extension: String) -> std::io::Result<()> {
    let ext_str = format!(r".*.{}", file_extension);
    let extension = Regex::new(&ext_str).unwrap();

    for entry in fs::read_dir(dir_path)? {
        let dir = entry?;
        let file_type = dir.file_type()?;
        let file_name = match dir.file_name().into_string() {
            Ok(name) => name,
            Err(e) => panic!("Failed to convert filename to String, error: {:?}", e),
        };

        // Would do this but compiler giving error for some reason???
        // let file_path = dir.path().into_os_string().into_string() {
        //     Ok(path) => path,
        //     Err(e) => panic!("Failed to convert path to String, error: {}", e)
        // };

        let file_path = dir.path().into_os_string().into_string().unwrap();

        if file_type.is_file() {
            if extension.is_match(&file_name) {
                print_num_lines(file_path);
            }
        } else if file_type.is_dir() {
            let _ = recursive_line_find(file_path, file_extension.clone());
        } else {
            println!("Symlink???");
        }
    }
    Ok(())
}

/// Given a path to a file, read the file into a string and print the number of lines it contains.
fn print_num_lines(path: String) {
    let mut file_handle = read_file(&path);

    // Read file contents into a string
    let mut s = String::new();
    let _ = match file_handle.read_to_string(&mut s) {
        Ok(num_bytes) => num_bytes,
        Err(e) => panic!("Couldn't read file, error: {}", e),
    };

    let num_lines = s.lines().count();
    println!("File \"{}\" has {} lines", path, num_lines);
}

/// Given a path to a file, return a File object for that file.
fn read_file(file_path: &str) -> File {
    let path = Path::new(file_path);
    let display = path.display();

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open {}: {}", display, e),
    };

    file
}
