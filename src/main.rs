mod version;

use std::env;
use std::process::ExitCode;
use std::str::Chars;

use crate::version::{Ordering, Version};

fn main() -> ExitCode {
    let args : Vec<String> = env::args().collect();

    match args.len() {
        2 => validate(args[1].chars()),
        3 => compare(args[1].chars(), args[2].chars()),
        _ => std::process::ExitCode::from(255),
    }
}

fn validate(version : Chars) -> ExitCode {
    match Version::parse(version) {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::from(255),
    }
}

fn compare(left_text : Chars, right_text : Chars) -> ExitCode {
    let status_code = if let Ok(left) = Version::parse(left_text) {
        if let Ok(right) = Version::parse(right_text) {
            let comparison = left.compare_to(&right);

            match comparison {
                Ordering::LessThan => 2, // second version is greater
                Ordering::GreaterThan => 1, // first version is greater
                Ordering::Equal => 0,
            }
        } else {
            255
        }
    } else {
        255
    };
    
    ExitCode::from(status_code)
}