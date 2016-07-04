extern crate grid;

use grid::Grid;
use std::cmp::max;

type GridNum = i64;

fn get_grid() -> Grid<GridNum> {
    let file_name = "input.txt";
    match Grid::from_file(file_name) {
        Err(err) => panic!("{:?}", err),
        Ok(grid) => grid,
    }
}

fn straight_max<F>(grid: &Grid<GridNum>, at: F) -> GridNum
    where F: Fn(usize, usize) -> GridNum {

    let mut max = None;

    for j in 0..grid.width() {
        for i in 0..(grid.width() - 3) {
            let mut sum = 1;

            for ii in i..(i + 4) {
                sum *= at(ii, j);
            }

            match max {
                None => max = Some(sum),
                Some(prev) => if prev < sum { max = Some(sum) },
            }
        }
    }

    max.unwrap()
}

fn diag_max(grid: &Grid<GridNum>) -> GridNum {
    let mut best = None;

    for j in 0..(grid.height() - 3) {
        for i in 0..(grid.width() - 3) {
            let mut right = 1;
            let mut left = 1;

            for ii in 0..4 {
                right *= *grid.at(i + ii, j + ii);
                left *= *grid.at(i + 3 - ii, j + ii);
            }

            let this_best = max(right, left);

            match best {
                None => best = Some(this_best),
                Some(prev) => if prev < this_best { best = Some(this_best) },
            }
        }
    }

    best.unwrap()
}

fn answer(grid: &Grid<GridNum>) -> GridNum {
    let mut best = straight_max(grid, |x, y| *grid.at(x, y));
    best = max(best, straight_max(grid, |x, y| *grid.at(y, x)));
    best = max(best, diag_max(grid));
    best
}

fn main() {
    let grid = get_grid();
    println!("{}", answer(&grid));
}
