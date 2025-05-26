mod version;

use std::result::Result;
use std::env;
use std::error::Error;

use crate::version::{Ordering, Version};

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err("exactly two version numbers expected")?;
    }

    let left = Version::parse(args[1].chars())?;
    let right = Version::parse(args[2].chars())?;
    
    let comparison = left.compare_to(&right);
    
    println!("{}", comparison);

    let status_code = match comparison {
        Ordering::LessThan => 2, // second version is greater
        Ordering::GreaterThan => 1, // first version is greater
        Ordering::Equal => 0,
    };

    std::process::exit(status_code)
}
