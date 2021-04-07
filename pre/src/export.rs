use super::*;
use bincode::serialize_into;
use std::fs::File;
use std::io::BufWriter;

pub fn write_to_disk(filename: &str, result: &BinFile) -> Result<(), Box<bincode::ErrorKind>> {
    let mut writer = BufWriter::new(File::create(&filename).unwrap());
    serialize_into(&mut writer, &result)
}
