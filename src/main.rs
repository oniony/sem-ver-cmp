use std::result::Result;
use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter, Write};
use std::hash::Hasher;
use std::iter::Peekable;
use std::str::Chars;
use crate::Label::{TextLabel, VersionLabel};

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err("exactly two version numbers expected")?;
    }

    let left = Version::parse(&mut args[1].chars().peekable())?;
    let right = Version::parse(&mut args[2].chars().peekable())?;
    
    let comparison = left.compare_to(&right);
    
    println!("{}", comparison);

    Ok(())
}

struct Version {
    major: u64,
    minor: u64,
    patch: u64,
    labels: Vec<Label>,
}

impl Version {
    pub fn parse(text : &mut Peekable<Chars>) -> Result<Version, &'static str> {
        if let Some(c) = text.peek() {
            if c == &'v' {
                text.next();
            }
        } else {
            return Err("missing version number component");
        }
        
        let major = Self::read_component(text, '.')?;
        let minor = Self::read_component(text, '.')?;
        let patch = Self::read_component(text, '-')?;
        let labels = Self::read_labels(text)?;

        let version = Version{
            major,
            minor,
            patch,
            labels,
        };
        
        Ok(version)
    }
    
    pub fn compare_to(&self, other: &Version) -> Ordering {
        if self.major < other.major {
            Ordering::Less
        } else if self.major > other.major {
            Ordering::Greater
        } else {
            if self.minor < other.minor {
                Ordering::Less
            } else if self.minor > other.minor {
                Ordering::Greater
            } else {
                if self.patch < other.patch {
                    Ordering::Less
                } else if self.patch > other.patch {
                    Ordering::Greater
                } else {
                    //TODO compare labels
                    Ordering::Equal
                }
            }
        }
    }
    
    fn read_component(text : &mut Peekable<Chars>, sep: char) -> Result<u64, &'static str>{
        // if first char is zero, then that must be whole number
        if let Some(c) = text.peek() {
            if c == &'0' {
                text.next();

                return match text.next() {
                    None => Ok(0),
                    Some(s) if s == sep => Ok(0),
                    _ => Err("invalid version number component"),
                }
            }
        }

        let mut component_text = String::new();
        
        loop {
            let c = text.next();
            match c {
                None => break,
                Some(s) if s == sep => break,
                Some(d) if ('0'..='9').contains(&d) => component_text.push(d),
                _ => return Err("invalid version number component"),
            }
        }
        
        if let Ok(n) = component_text.parse::<u64>() {
            Ok(n)
        } else {
            Err("invalid version number component")
        }
    }
    
    fn read_labels(text: &mut Peekable<Chars>) -> Result<Vec<Label>, &'static str> {
        let mut labels = Vec::new();
        
        loop {
            let label = Self::read_label(text)?;
            
            match label {
                TextLabel(t) if t.is_empty() => break,
                _ => labels.push(label),
            }
        }
        
        Ok(labels)
    }

    fn read_label(text : &mut Peekable<Chars>) -> Result<Label, &'static str>{
        let mut label_text = String::new();

        loop {
            let c = text.next();
            match c {
                None => break,
                Some(s) if s == '.' => break,
                Some(d) if ('0'..='9').contains(&d) || ('a'..='z').contains(&d) || ('A'..='Z').contains(&d) || d == '-' => label_text.push(d),
                _ => return Err("invalid version number component"),
            }
        }

        let label = if let Ok(n) = label_text.parse::<u64>() {
            VersionLabel(n)
        } else {
            TextLabel(label_text)
        };
        
        Ok(label)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        
        if !self.labels.is_empty() {
            f.write_str("-")?;

            for (index, label) in self.labels.iter().enumerate() {
                if index > 0 {
                    write!(f, ".")?;
                }

                write!(f, "{}", label)?;
            }
        }
        
        Ok(())
    }
}

enum Label {
    TextLabel(String),
    VersionLabel(u64),
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextLabel(t) => f.write_str(t),
            VersionLabel(n) => f.write_fmt(format_args!("{}", n))
        }
    }
}

enum Ordering {
    Less,
    Greater,
    Equal,
}

impl Display for Ordering {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Ordering::Less => write!(f, "less"),
            Ordering::Greater => write!(f, "greater"),
            Ordering::Equal => write!(f, "equal"),
        }
    }
}