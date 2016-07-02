use std::str::FromStr;

#[derive(Debug)]
pub struct Grid<T> {
    values: Vec<T>,
    width: usize,
}

impl <T> Grid<T> {

    pub fn height(&self) -> usize {
        self.values.len() / self.width
    }

    pub fn at(&self, x: usize, y: usize) -> &T {
        if x >= self.width {
            panic!("x index out of bounds");
        }

        &self.values[x + y * self.width]
    }

}

#[derive(Debug)]
pub struct FromStrErr;

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
                    Err(_) => return Err(FromStrErr),
                    Ok(value) => grid.values.push(value),
                }
            }
        }

        Ok(grid)
    }

}

fn main() {
    println!("Hello, world!");
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
