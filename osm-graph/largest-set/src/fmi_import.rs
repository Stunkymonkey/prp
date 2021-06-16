use crate::structs::*;

pub fn read_file(
    file_path: &str,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<Edge>,
) -> std::io::Result<String> {
    let mut node_amount = 0;
    let mut edge_amount = 0;
    let mut file_edge_dimensions = 1;
    let mut header = "".to_string();

    let mut reader = file_reader::BufReader::open(file_path)?;
    let mut buffer = String::new();

    // iterate over comments
    while let Some(line) = reader.read_line(&mut buffer) {
        let line = line?;
        if line.starts_with('#') {
            header.push_str(line);
        } else {
            break;
        }
    }

    if let Some(line) = reader.read_line(&mut buffer) {
        file_edge_dimensions = line?.trim().parse().unwrap();
    }
    if let Some(line) = reader.read_line(&mut buffer) {
        node_amount = line?.trim().parse().unwrap();
    }
    if let Some(line) = reader.read_line(&mut buffer) {
        edge_amount = line?.trim().parse().unwrap();
    }

    // allocate space for nodes & edges
    nodes.reserve_exact(node_amount);
    edges.reserve_exact(edge_amount);

    for _ in 0..node_amount {
        if let Some(line) = reader.read_line(&mut buffer) {
            let v: Vec<&str> = line?.trim().split(' ').collect();
            nodes.push(Node {
                osm_id: v[1].parse().unwrap(),
                latitude: v[2].parse().unwrap(),
                longitude: v[3].parse().unwrap(),
                height: v[4].parse().unwrap(),
            });
        }
    }

    for _ in 0..edge_amount {
        if let Some(line) = reader.read_line(&mut buffer) {
            let v: Vec<&str> = line?.trim().split(' ').collect();
            let mut edge_weights = Vec::new();
            for weight in v.iter().skip(2).take(file_edge_dimensions) {
                edge_weights.push(weight.parse().unwrap());
            }
            let contracted_edges: Vec<isize> = v
                .iter()
                .skip(2 + file_edge_dimensions)
                .take(2)
                .map(|value| value.parse().unwrap())
                .collect();

            edges.push(Edge::new(
                v[0].parse().unwrap(),
                v[1].parse().unwrap(),
                edge_weights,
                (contracted_edges[0], contracted_edges[1]),
            ));
        }
    }

    match reader.read_line(&mut buffer) {
        Some(line) => panic!("file should end here, but still recieved {:?}", line),
        None => Ok(header),
    }
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
