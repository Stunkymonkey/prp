use super::*;
use bincode::deserialize_from;
use std::fs::File;
use std::io::BufReader;

pub fn read_file(fmi_file: &str) -> Result<BinFile, Box<dyn std::error::Error>> {
    let file = File::open(fmi_file)?;
    let mut reader = BufReader::new(file);
    let data: BinFile = deserialize_from(&mut reader)?;
    Ok(data)
}
