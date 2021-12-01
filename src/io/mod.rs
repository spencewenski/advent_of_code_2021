use anyhow::Result;
use std::fs::File;
use std::io;

pub fn reader(filename: Option<&String>) -> Result<Box<dyn io::BufRead>> {
    let reader: Box<dyn io::Read> = match filename {
        Some(name) => {
            let file = File::open(name)?;
            Box::new(io::BufReader::new(file))
        }
        None => Box::new(io::stdin()),
    };

    Ok(Box::new(io::BufReader::new(reader)))
}
