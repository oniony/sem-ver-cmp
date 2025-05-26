use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;

pub struct Version {
    major: Major,
    minor: Minor,
    patch: Patch,
    prerelease: Prerelease,
    build: Build,
}

impl Version {
    pub fn parse(text : Chars) -> Result<Version, &'static str> {
        let mut peekable = text.peekable();

        // ignore any `v` prefix
        if let Some(c) = peekable.peek() {
            if c == &'v' {
                peekable.next();
            }
        } else {
            return Err("missing version number component");
        }

        let version = Self::read_version(&mut peekable)?;

        Ok(version)
    }

    pub fn compare_to(&self, other: &Version) -> Ordering {
        if self.major < other.major {
            Ordering::LessThan
        } else if self.major > other.major {
            Ordering::GreaterThan
        } else {
            if self.minor < other.minor {
                Ordering::LessThan
            } else if self.minor > other.minor {
                Ordering::GreaterThan
            } else {
                if self.patch < other.patch {
                    Ordering::LessThan
                } else if self.patch > other.patch {
                    Ordering::GreaterThan
                } else {
                    //TODO compare labels
                    Ordering::Equal
                }
            }
        }
    }

    // <valid semver> ::= <version core>
    //                  | <version core> "-" <pre-release>
    //                  | <version core> "+" <build>
    //                  | <version core> "-" <pre-release> "+" <build>
    fn read_version(text : &mut Peekable<Chars>) -> Result<Version, &'static str> {
        let version_core = Self::read_version_core(text)?;
        let mut prerelease = Prerelease::default();
        let mut build = Build::default();

        if let Some(c) = text.peek() {
            match c {
                '-' => {
                    text.next();
                    prerelease = Self::read_prerelease(text)?;

                    if let Some(c) = text.peek() {
                        match c {
                            '+' => {
                                text.next();
                                build = Self::read_build(text)?;
                            },
                            _ => return Err("extraneous characters"),
                        }
                    }
                },
                '+' => {
                    text.next();
                    build = Self::read_build(text)?;

                },
                _ => return Err("extraneous characters"),
            }
        }

        if text.peek().is_some() {
            return Err("extraneous characters")
        }

        let version = Version {
            major: version_core.major,
            minor: version_core.minor,
            patch: version_core.patch,
            prerelease,
            build,
        };

        Ok(version)
    }

    // <version core> ::= <major> "." <minor> "." <patch>
    fn read_version_core(text : &mut Peekable<Chars>) -> Result<VersionCore, &'static str> {
        let major = Self::read_major(text)?;
        Self::read_period(text)?;
        let minor = Self::read_minor(text)?;
        Self::read_period(text)?;
        let patch = Self::read_patch(text)?;

        let version_core = VersionCore {
            major,
            minor,
            patch,
        };

        Ok(version_core)
    }

    // <major> ::= <numeric identifier>
    fn read_major(text : &mut Peekable<Chars>) -> Result<Major, &'static str>{
        let numeric_identifier = Self::read_numeric_identifier(text)?;
        let major = Major(numeric_identifier);

        Ok(major)
    }

    // <minor> ::= <numeric identifier>
    fn read_minor(text : &mut Peekable<Chars>) -> Result<Minor, &'static str> {
        let numeric_identifier = Self::read_numeric_identifier(text)?;
        let minor = Minor(numeric_identifier);

        Ok(minor)

    }

    // <patch> ::= <numeric identifier>
    fn read_patch(text : &mut Peekable<Chars>) -> Result<Patch, &'static str> {
        let numeric_identifier = Self::read_numeric_identifier(text)?;
        let patch = Patch(numeric_identifier);

        Ok(patch)

    }

    // <pre-release> ::= <dot-separated pre-release identifiers>
    // 
    // <dot-separated pre-release identifiers> ::= <pre-release identifier>
    //                                           | <pre-release identifier> "." <dot-separated pre-release identifiers>
    //
    // <pre-release identifier> ::= <alphanumeric identifier>
    //                            | <numeric identifier>
    fn read_prerelease(_text : &mut Peekable<Chars>) -> Result<Prerelease, &'static str> {
        Ok(Prerelease{})
    }

    // <build> ::= <dot-separated build identifiers>
    //
    // <dot-separated build identifiers> ::= <build identifier>
    //                                     | <build identifier> "." <dot-separated build identifiers>
    //
    // <build identifier> ::= <alphanumeric identifier>
    //                      | <digits>
    //
    // <alphanumeric identifier> ::= <non-digit>
    //                             | <non-digit> <identifier characters>
    //                             | <identifier characters> <non-digit>
    //                             | <identifier characters> <non-digit> <identifier characters>
    //
    // <identifier characters> ::= <identifier character>
    //                           | <identifier character> <identifier characters>
    //
    // <identifier character> ::= <digit>
    //                          | <non-digit>
    //
    // <non-digit> ::= <letter>
    //               | "-"
    fn read_build(_text : &mut Peekable<Chars>) -> Result<Build, &'static str> {
        Ok(Build{})
    }

    fn read_period(text : &mut Peekable<Chars>) -> Result<(), &'static str> {
        if let Some(c) = text.next() {
            match c {
                '.' => Ok(()),
                _ => Err("expected period")
            }
        } else {
            Err("unexpected EOF")
        }
    }

    // <numeric identifier> ::= "0"
    //                        | <positive digit>
    //                        | <positive digit> <digits>
    fn read_numeric_identifier(text : &mut Peekable<Chars>) -> Result<NumericIdentifier, &'static str> {
        if let Some(c) = text.peek() {
            if *c == '0' {
                text.next();
                Ok(NumericIdentifier(0))
            } else {
                let positive_number = Self::read_positive_number(text)?;
                Ok(NumericIdentifier(positive_number))
            }
        } else {
            Err("unexpected EOF")
        }
    }

    fn read_positive_number(text : &mut Peekable<Chars>) -> Result<u64, &'static str> {
        let mut number : u64 = 0;

        if let Some(c) = text.next() {
            match c {
                '1'..='9' => number = c as u64 - b'0' as u64,
                _ => return Err("expected positive digit")
            }
        }

        while let Some(c) = text.peek() {
            match c {
                '0'..='9' => {
                    number = number * 10 + *c as u64 - b'0' as u64;
                    text.next();
                },
                _ => break,
            }
        }

        Ok(number)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;

        // if !self.labels.is_empty() {
        //     f.write_str("-")?;
        // 
        //     for (index, label) in self.labels.iter().enumerate() {
        //         if index > 0 {
        //             write!(f, ".")?;
        //         }
        // 
        //         write!(f, "{}", label)?;
        //     }
        // }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct VersionCore {
    major: Major,
    minor: Minor,
    patch: Patch,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Major(NumericIdentifier);

impl Display for Major {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Minor(NumericIdentifier);

impl Display for Minor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Patch(NumericIdentifier);

impl Display for Patch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Default)]
struct Prerelease {
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Default)]
struct Build {
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct NumericIdentifier(u64);

impl Display for NumericIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum Ordering {
    LessThan,
    GreaterThan,
    Equal,
}

impl Display for Ordering {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Ordering::LessThan => write!(f, "less than"),
            Ordering::GreaterThan => write!(f, "greater than"),
            Ordering::Equal => write!(f, "equal"),
        }
    }
}
