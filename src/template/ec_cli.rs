/// Wrapper module around the "ec-cli" command-line.
use std::{
    fmt::Display,
    process::{Command, Output, Stdio},
};

use crate::template::Day;

#[derive(Debug)]
pub enum EcCommandError {
    CommandNotFound,
    CommandNotCallable,
    BadExitStatus(Output),
}

impl Display for EcCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EcCommandError::CommandNotFound => write!(f, "ec-cli is not present in environment."),
            EcCommandError::CommandNotCallable => write!(f, "ec-cli could not be called."),
            EcCommandError::BadExitStatus(_) => {
                write!(f, "ec-cli exited with a non-zero status.")
            }
        }
    }
}

pub fn check() -> Result<(), EcCommandError> {
    Command::new("ec-cli")
        .arg("--version")
        .output()
        .map_err(|_| EcCommandError::CommandNotFound)?;
    Ok(())
}

pub fn read(day: Day) -> Result<Output, EcCommandError> {
    let mut args = vec!["read".to_string(), "-d".to_string(), day.to_string()];

    if let Some(year) = get_year() {
        args.push("-y".to_string());
        args.push(year.to_string());
    }

    call_ec_cli(&args)
}

pub fn download(day: Day) -> Result<Output, EcCommandError> {
    // Download all 3 parts for Everybody Codes
    for part in 1..=3 {
        let input_path = get_input_path(day, part);
        let sample_path = get_sample_path(day, part);
        let sample_answer_path = get_sample_answer_path(day, part);
        let desc_path = get_description_path(day, part);

        let mut args = vec![
            "fetch".to_string(),
            "-d".to_string(),
            day.to_string(),
            "-p".to_string(),
            part.to_string(),
            "--sample-path".to_string(),
            sample_path.clone(),
            "--sample-answer-path".to_string(),
            sample_answer_path.clone(),
            "--input-path".to_string(),
            input_path.clone(),
            "--description-path".to_string(),
            desc_path.clone(),
        ];

        if let Some(year) = get_year() {
            args.push("-y".to_string());
            args.push(year.to_string());
        }

        let result = call_ec_cli(&args);

        if part == 1 {
            println!("---");
        }

        match result {
            Ok(_) => {
                println!("📝 Successfully wrote description to \"{}\".", &desc_path);
                println!("📥 Successfully wrote input to \"{}\".", &input_path);
                println!("🧪 Successfully wrote sample to \"{}\".", &sample_path);
                println!("✅ Successfully wrote sample answer to \"{}\".", &sample_answer_path);
            }
            Err(_) => {
                // If download fails (e.g., part not available yet), write "0" to sample and answer
                if part > 1 {
                    if let Err(e) = std::fs::write(&sample_path, "0") {
                        eprintln!("Failed to write default sample to {}: {}", &sample_path, e);
                    }
                    if let Err(e) = std::fs::write(&sample_answer_path, "0") {
                        eprintln!(
                            "Failed to write default sample answer to {}: {}",
                            &sample_answer_path, e
                        );
                    }
                    println!("⚠️  Part {} not available, wrote defaults to sample and answer files.", part);
                } else {
                    // Part 1 should always be available, so return the error
                    return Err(result.unwrap_err());
                }
            }
        }

        if part < 3 {
            println!();
        }
    }

    println!("---");
    Ok(Output {
        status: std::process::ExitStatus::default(),
        stdout: vec![],
        stderr: vec![],
    })
}

pub fn submit(day: Day, part: u8, result: &str) -> Result<Output, EcCommandError> {
    let mut args = vec![
        "submit".to_string(),
        "-d".to_string(),
        day.to_string(),
        "-p".to_string(),
        part.to_string(),
        result.to_string(),
    ];

    if let Some(year) = get_year() {
        args.push("-y".to_string());
        args.push(year.to_string());
    }

    call_ec_cli(&args)
}

fn get_input_path(day: Day, part: u8) -> String {
    format!("data/inputs/{day}-{part}.txt")
}

fn get_sample_path(day: Day, part: u8) -> String {
    format!("data/samples/{day}-{part}.txt")
}

fn get_sample_answer_path(day: Day, part: u8) -> String {
    format!("data/answers/{day}-{part}.txt")
}

fn get_description_path(day: Day, part: u8) -> String {
    format!("data/descriptions/{day}-{part}.html")
}

fn get_year() -> Option<u16> {
    match std::env::var("EC_YEAR") {
        Ok(x) => x.parse().ok().or(None),
        Err(_) => None,
    }
}

fn call_ec_cli(args: &[String]) -> Result<Output, EcCommandError> {
    // println!("Calling >ec-cli with: {}", args.join(" "));
    let output = Command::new("ec-cli")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| EcCommandError::CommandNotCallable)?;

    if output.status.success() {
        Ok(output)
    } else {
        Err(EcCommandError::BadExitStatus(output))
    }
}
