use ndarray::Array2;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

use std::fmt::Debug;
use std::str::FromStr;

/// parses ascii based grids into Array2
pub fn parse_array2<T>(input: &str) -> Array2<T>
where
    T: FromStr + Default + Debug,
    <T as FromStr>::Err: Debug,
{
    let mut grid: Array2<T> =
        Array2::default((input.lines().count(), input.lines().next().unwrap().len()));
    for (y, line) in input.lines().enumerate() {
        for (x, digit) in line.chars().enumerate() {
            let val = digit.to_string().parse().unwrap();
            grid[(y, x)] = val;
        }
    }
    grid
}

/// shape of given array2 as tuple
pub fn shape2<T>(grid: &Array2<T>) -> (usize, usize) {
    let shape = grid.shape();
    (shape[0], shape[1])
}

/// converts given Array2 to a Graph
pub fn array2_to_graph4<T: Clone>(grid: &Array2<T>) -> (Graph<T, T>, Array2<NodeIndex>) {
    let mut graph: Graph<T, T> = Graph::new();
    let (height, width) = shape2(grid);
    let mut node_grid: Array2<NodeIndex> = Array2::default((height, width));
    for (y, row) in grid.rows().into_iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            node_grid[(y, x)] = graph.add_node(col.clone());
        }
    }
    for (y, row) in grid.rows().into_iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            for (ny, nx) in get_neighbors4(grid, &(y, x)) {
                let from_node = node_grid[(y, x)];
                let to_node = node_grid[(ny, nx)];
                graph.add_edge(from_node, to_node, grid[(ny, nx)].clone());
                graph.add_edge(to_node, from_node, col.clone());
            }
        }
    }
    (graph, node_grid)
}

/// gets 4 neighbor positions of given pos in grid
pub fn get_neighbors4<T>(grid: &Array2<T>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let (height, width) = shape2(grid);
    let mut list = Vec::new();
    // up
    if pos.0 > 0 {
        list.push((pos.0 - 1, pos.1));
    }
    // right
    if pos.1 > 0 {
        list.push((pos.0, pos.1 - 1));
    }
    // down
    if pos.0 < height - 1 {
        list.push((pos.0 + 1, pos.1));
    }
    // left
    if pos.1 < width - 1 {
        list.push((pos.0, pos.1 + 1));
    }
    list
}

/// gets 8 neighbor positions of given pos in grid
pub fn get_neighbors8(grid: &Array2<u8>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let (height, width) = shape2(grid);
    let mut list = get_neighbors4(grid, pos);
    // up/left
    if pos.0 > 0 && pos.1 > 0 {
        list.push((pos.0 - 1, pos.1 - 1));
    }
    // up/right
    if pos.0 > 0 && pos.1 < width - 1 {
        list.push((pos.0 - 1, pos.1 + 1));
    }
    // down/right
    if pos.0 < height - 1 && pos.1 > 0 {
        list.push((pos.0 + 1, pos.1 - 1));
    }
    // down/left
    if pos.0 < height - 1 && pos.1 < width - 1 {
        list.push((pos.0 + 1, pos.1 + 1));
    }
    list
}
