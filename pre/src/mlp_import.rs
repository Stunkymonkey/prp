use super::*;
use crate::fmi_import::file_reader;

pub fn read_file(
    file_path: &str,
    nodes: &mut Vec<Node>,
    mlp_layers: &mut Vec<usize>,
) -> std::io::Result<()> {
    let mut mlp_amount = 0;
    let mut node_amount = 0;

    let mut reader = file_reader::BufReader::open(file_path)?;
    let mut buffer = String::new();

    if let Some(line) = reader.read_line(&mut buffer) {
        mlp_amount = line?.trim().parse().unwrap();
    }
    mlp_layers.reserve_exact(mlp_amount);
    for _ in 0..mlp_amount {
        if let Some(line) = reader.read_line(&mut buffer) {
            mlp_layers.push(line?.trim().parse().unwrap());
        }
    }

    let max_partition = mlp_layers.iter().product::<usize>();

    if let Some(line) = reader.read_line(&mut buffer) {
        node_amount = line?.trim().parse().unwrap();
    }

    for node in nodes.iter_mut().take(node_amount) {
        if let Some(line) = reader.read_line(&mut buffer) {
            let partition = line?.trim().parse().unwrap();
            assert!(partition < max_partition);
            node.partition = partition;
        }
    }

    match reader.read_line(&mut buffer) {
        Some(line) => panic!("file should end here, but still recieved {:?}", line),
        None => Ok(()),
    }
}
