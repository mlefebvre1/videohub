use clap::Parser;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Error {
    ParseIntError(ParseIntError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
impl std::error::Error for Error {}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(
        short = 'i',
        long = "ip-address",
        help = "IPv4 Address of the videohub device. Ex: -i 10.0.0.1"
    )]
    pub ip_address: String,
    #[clap(
        long,
        help = "Change an output label. For example, to change the label of the port 1 to 'my new label' use: 1=\"my new label\" "
    )]
    pub output_label: Option<IndexAndStr>,
    #[clap(
        long,
        help = "Change an input label. For example, to change the label of the port 2 to 'foo' use: 2=foo "
    )]
    pub input_label: Option<IndexAndStr>,
    #[clap(
        short = 'o',
        long = "output-route",
        help = "Route an Input to an Output: To route the input 15 to the output 40 write: 15=40 "
    )]
    pub output_route: Option<IndexAndIndex>,
    #[clap(
        short = 'd',
        long = "display",
        help = "Display Videohub info and current state",
        action
    )]
    pub display: bool,
    #[clap(short = 'u', long = "unlock", help = "Unlock a given port")]
    pub unlock: Option<usize>,
    #[clap(short = 'l', long = "lock", help = "Lock a given port")]
    pub lock: Option<usize>,
}

impl Cli {
    pub fn get() -> Self {
        Self::parse()
    }
}

#[derive(Debug)]
pub struct IndexAndStr {
    pub index: usize,
    pub value: String,
}

impl FromStr for IndexAndStr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let index = chars
            .by_ref()
            .take_while(|&c| c != '=')
            .collect::<String>()
            .parse::<usize>()
            .map_err(Error::ParseIntError)?;
        let value: String = chars.collect();
        Ok(Self { index, value })
    }
}

#[derive(Debug)]
pub struct IndexAndIndex {
    pub a: usize,
    pub b: usize,
}

impl FromStr for IndexAndIndex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_as_int = s.split('=').map(|word| word.parse::<usize>());
        let a = s_as_int.next().unwrap().map_err(Error::ParseIntError)?;
        let b = s_as_int.next().unwrap().map_err(Error::ParseIntError)?;
        Ok(Self { a, b })
    }
}
