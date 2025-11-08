use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    process,
};

use crate::template::Day;

const MODULE_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/template.txt"));

fn safe_create_file(path: &str, overwrite: bool) -> Result<File, std::io::Error> {
    let mut file = OpenOptions::new();
    if overwrite {
        file.create(true);
    } else {
        file.create_new(true);
    }
    file.truncate(true).write(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

pub fn handle(day: Day, overwrite: bool) {
    // Ensure directories exist
    if let Err(e) = fs::create_dir_all("data/inputs") {
        eprintln!("Failed to create data/inputs directory: {e}");
        process::exit(1);
    }
    if let Err(e) = fs::create_dir_all("data/samples") {
        eprintln!("Failed to create data/samples directory: {e}");
        process::exit(1);
    }
    if let Err(e) = fs::create_dir_all("data/descriptions") {
        eprintln!("Failed to create data/descriptions directory: {e}");
        process::exit(1);
    }

    let module_path = format!("src/bin/{day}.rs");

    let mut file = match safe_create_file(&module_path, overwrite) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {e}");
            process::exit(1);
        }
    };

    match file.write_all(
        MODULE_TEMPLATE
            .replace("%DAY_NUMBER%", &day.into_inner().to_string())
            .as_bytes(),
    ) {
        Ok(()) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            process::exit(1);
        }
    }

    // Create input and sample files for all 3 parts
    for part in 1..=3 {
        let input_path = format!("data/inputs/{day}-{part}.txt");
        let sample_path = format!("data/samples/{day}-{part}.txt");

        match create_file(&input_path) {
            Ok(_) => {
                println!("Created empty input file \"{}\"", &input_path);
            }
            Err(e) => {
                eprintln!("Failed to create input file: {e}");
                process::exit(1);
            }
        }

        match create_file(&sample_path) {
            Ok(_) => {
                println!("Created empty sample file \"{}\"", &sample_path);
            }
            Err(e) => {
                eprintln!("Failed to create sample file: {e}");
                process::exit(1);
            }
        }
    }

    println!("---");
    println!("🎯 Type `cargo solve {day}` to run your solution.");
}
