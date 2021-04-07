pub fn read_file(eval_file: &str) -> std::io::Result<Vec<Vec<String>>> {
    let mut reader = file_reader::BufReader::open(eval_file)?;
    let mut buffer = String::new();

    let mut lines = Vec::new();
    while let Some(line) = reader.read_line(&mut buffer) {
        let line = line?;
        lines.push(
            line.split(&[' ', ',', ';'][..])
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }

    Ok(lines)
}

pub mod file_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}
