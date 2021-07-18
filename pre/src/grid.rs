use super::*;

/// get min and max of lat and lng
fn get_min_max(nodes: &[Node]) -> GridBounds {
    let lat_min = nodes
        .par_iter()
        .map(|node| node.latitude)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();
    let lat_max = nodes
        .par_iter()
        .map(|node| node.latitude)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();
    let lng_min = nodes
        .par_iter()
        .map(|node| node.longitude)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();
    let lng_max = nodes
        .par_iter()
        .map(|node| node.longitude)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();
    GridBounds {
        lat_amount: LAT_GRID_AMOUNT,
        lat_min,
        lat_max,
        lng_amount: LNG_GRID_AMOUNT,
        lng_min,
        lng_max,
    }
}

#[allow(clippy::suspicious_operation_groupings)]
fn get_grid_lat(node: &Node, grid_bounds: &GridBounds) -> usize {
    let lat_percent =
        (node.latitude - grid_bounds.lat_min) / (grid_bounds.lat_max - grid_bounds.lat_min);
    (lat_percent * (LAT_GRID_AMOUNT - 1) as Angle) as usize
}

#[allow(clippy::suspicious_operation_groupings)]
fn get_grid_lng(node: &Node, grid_bounds: &GridBounds) -> usize {
    let lng_percent =
        (node.longitude - grid_bounds.lng_min) / (grid_bounds.lng_max - grid_bounds.lng_min);
    (lng_percent * (LNG_GRID_AMOUNT - 1) as Angle) as usize
}

fn calculate_grid_id(lat_index: usize, lng_index: usize) -> GridId {
    lng_index * LAT_GRID_AMOUNT + lat_index
}

fn get_grid_id(node: &Node, grid_bounds: &GridBounds) -> GridId {
    let lat_index = get_grid_lat(node, grid_bounds);
    let lng_index = get_grid_lng(node, grid_bounds);
    calculate_grid_id(lat_index, lng_index)
}

pub fn generate_grid(
    grid: &mut Vec<GridId>,
    grid_offset: &mut Vec<usize>,
    nodes: &[Node],
) -> GridBounds {
    let grid_bounds: GridBounds = get_min_max(nodes);

    *grid_offset = vec![0; (LAT_GRID_AMOUNT * LNG_GRID_AMOUNT) + 1];

    // calculate how much nodes go into each cell
    let mut target_cells: Vec<usize> = vec![0; LAT_GRID_AMOUNT * LNG_GRID_AMOUNT];
    for node in nodes {
        target_cells[get_grid_id(node, &grid_bounds)] += 1;
    }

    // generate offset based on target_cells
    for i in 1..grid_offset.len() {
        grid_offset[i] = grid_offset[i - 1] + target_cells[i - 1];
    }

    *grid = vec![INVALID_NODE; nodes.len()];

    // fill offsets, where not already filled
    for (i, node) in nodes.iter().enumerate() {
        let grid_id = get_grid_id(node, &grid_bounds);
        let start_index = grid_offset[grid_id];
        let end_index = grid_offset[grid_id + 1];
        for j in grid.iter_mut().take(end_index).skip(start_index) {
            if *j == INVALID_NODE {
                *j = i;
                break;
            }
        }
    }
    grid_bounds
}

#[test]
fn grid_grid_bounds() {
    let mut nodes = Vec::new();
    nodes.push(Node {
        latitude: 10.0,
        longitude: 30.0,
        rank: 0,
        partition: INVALID_PARTITION,
        level_height: INVALID_LEVEL,
        old_id: None,
    });
    nodes.push(Node {
        latitude: 20.0,
        longitude: 30.0,
        rank: 0,
        partition: INVALID_PARTITION,
        level_height: INVALID_LEVEL,
        old_id: None,
    });
    nodes.push(Node {
        latitude: 10.0,
        longitude: 40.0,
        rank: 0,
        partition: INVALID_PARTITION,
        level_height: INVALID_LEVEL,
        old_id: None,
    });
    nodes.push(Node {
        latitude: 20.0,
        longitude: 40.0,
        rank: 0,
        partition: INVALID_PARTITION,
        level_height: INVALID_LEVEL,
        old_id: None,
    });

    let grid_bounds: GridBounds = get_min_max(&nodes);

    let error_margin = Angle::EPSILON;
    assert!(grid_bounds.lat_min - 10.0_f64 < error_margin);
    assert!(grid_bounds.lat_max - 20.0_f64 < error_margin);
    assert!(grid_bounds.lng_min - 30.0_f64 < error_margin);
    assert!(grid_bounds.lng_max - 40.0_f64 < error_margin);
}

#[test]
fn grid_id() {
    let mut nodes = Vec::new();
    nodes.push(Node {
        latitude: 10.0,
        longitude: 10.0,
        rank: 0,
        partition: INVALID_PARTITION,
        level_height: INVALID_LEVEL,
        old_id: None,
    });
    nodes.push(Node {
        latitude: 20.0,
        longitude: 10.0,
        rank: 0,
        partition: INVALID_PARTITION,
        level_height: INVALID_LEVEL,
        old_id: None,
    });
    nodes.push(Node {
        latitude: 10.0,
        longitude: 20.0,
        rank: 0,
        partition: INVALID_PARTITION,
        level_height: INVALID_LEVEL,
        old_id: None,
    });
    nodes.push(Node {
        latitude: 20.0,
        longitude: 20.0,
        rank: 0,
        partition: INVALID_PARTITION,
        level_height: INVALID_LEVEL,
        old_id: None,
    });

    let grid_bounds: GridBounds = get_min_max(&nodes);

    let node_id = get_grid_id(
        &Node {
            latitude: 10.1,
            longitude: 10.1,
            rank: 0,
            partition: INVALID_PARTITION,
            level_height: INVALID_LEVEL,
            old_id: None,
        },
        &grid_bounds,
    );
    assert_eq!(node_id, 1);

    let node_id = get_grid_id(
        &Node {
            latitude: 10.4,
            longitude: 10.4,
            rank: 0,
            partition: INVALID_PARTITION,
            level_height: INVALID_LEVEL,
            old_id: None,
        },
        &grid_bounds,
    );
    assert_eq!(node_id, 413);

    let node_id = get_grid_id(
        &Node {
            latitude: 17.3,
            longitude: 12.7,
            rank: 0,
            partition: INVALID_PARTITION,
            level_height: INVALID_LEVEL,
            old_id: None,
        },
        &grid_bounds,
    );
    assert_eq!(node_id, 3634);
}
