extern crate grid;

use std::collections::HashMap;

use grid::Grid;

type Weight = i64;
type Index = (usize, usize);

struct Graph {
    map: HashMap<Index, Weight>,
    width: usize,
    height: usize,
}

impl Graph {

    fn from_grid(grid: &Grid<Weight>) -> Self {
        let mut graph = Graph {
            map: HashMap::new(),
            width: grid.width(),
            height: grid.height(),
        };

        for j in 0..grid.height() {
            for i in 0..grid.width() {
                graph.map.insert((i, j), *grid.at(i, j));
            }
        }

        graph
    }

    // TODO possible optimization: use stack-allocated arrays here?
    fn neighbors(&self, i: usize, j: usize) -> Vec<Index> {
        let on_axis = |k, len| {
            let mut base = vec![k];
            if k > 0 { base.push(k - 1); }
            if k < len { base.push(k + 1); }
            base
        };

        let i_neighbors = on_axis(i, self.width);
        let j_neighbors = on_axis(j, self.height);

        let mut neighbors = Vec::new();
        for jj in &j_neighbors {
            for ii in &i_neighbors {
                if !(*jj == j && *ii == i) {
                    neighbors.push((*ii, *jj));
                }
            }
        }

        neighbors
    }

    fn least_weight_path(&self, start: Index, end: Index) -> Vec<(Index, Weight)> {
        unimplemented!();
    }
}

fn main() {
    let grid: Grid<Weight> = match Grid::from_file("input.txt") {
        err @ Err(_) => panic!("{:?}", err),
        ok @ Ok(_) => ok.unwrap(),
    };

    let graph = Graph::from_grid(&grid);
    println!("{:?}", graph.neighbors(0, 0));
    println!("{:?}", graph.neighbors(3, 0));
    println!("{:?}", graph.neighbors(0, 3));
    println!("{:?}", graph.neighbors(3, 3));
}
