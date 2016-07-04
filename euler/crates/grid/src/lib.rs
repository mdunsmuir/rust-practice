use std::str::FromStr;
use std::fs::File;
use std::io::Error;
use std::io::Read;

#[derive(Debug)]
pub struct Grid<T> {
    values: Vec<T>,
    width: usize,
}

impl <T> Grid<T> {

    pub fn height(&self) -> usize {
        self.values.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn at(&self, x: usize, y: usize) -> &T {
        if x >= self.width {
            panic!("x index out of bounds");
        }

        &self.values[x + y * self.width]
    }

}

#[derive(Debug)]
pub enum FromStrErr {
    GenericErr,
    IOErr(Error),
}

use FromStrErr::*;

impl <T: FromStr> Grid<T> {

    pub fn from_file(path: &str) -> Result<Self, FromStrErr> {
        match File::open(path) {
            Err(err) => Err(IOErr(err)),
            Ok(mut file) => {
                let mut buf = String::new();
                match file.read_to_string(&mut buf) {
                    Err(err) => Err(IOErr(err)),
                    Ok(_) => Self::from_str(&buf),
                }
            },
        }
    }

}

impl <T: FromStr> FromStr for Grid<T> {

    type Err = FromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Grid<T> = Grid { values: Vec::new(), width: 0 };
        let mut width: Option<usize> = None;

        for line in s.lines() {
            let words: Vec<&str> = line.split_whitespace().collect();

            match width {
                None => {
                    grid.width = words.len();
                    width = Some(words.len());
                },
                Some(len) => if words.len() != len {
                    panic!("lines not same length!");
                },
            };

            for word in words {
                match T::from_str(word) {
                    Err(_) => return Err(GenericErr),
                    Ok(value) => grid.values.push(value),
                }
            }
        }

        Ok(grid)
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;

    #[test]
    fn it_works() {
        let string = "1 2 3\n4 5 6\n7 8 9";
        let grid: Grid<i64> = Grid::from_str(&string).unwrap();

        assert_eq!(3, grid.width);
        assert_eq!(3, grid.height());

        assert_eq!(1, *grid.at(0, 0));
        assert_eq!(6, *grid.at(2, 1));
    }

}
