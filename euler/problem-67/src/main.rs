pub mod smallest_path_sum {

    fn make_row_stack<'a>(rows: &'a [i64]) -> Result<Vec<&'a [i64]>, &'a str> {
        let mut stack = Vec::new();
        let mut len = 1;
        let mut i = 0;

        while i < rows.len() {
            if i + len > rows.len() {
                return Err("Your triangle doesn't seem to be triangular");
            }

            stack.push(&rows[i..(i + len)]);
            i += len;
            len += 1;
        }

        Ok(stack)
    }

    use std::cmp::max;

    pub fn smallest_path_sum<'a>(rows: &'a [i64]) -> Result<i64, &'a str> {
        make_row_stack(rows).and_then(|mut stack| {

            // get the first row, which we load into our 'accumulator'
            if let Some(first_row) = stack.pop() {
                let mut active: Vec<i64> = Vec::with_capacity(first_row.len());

                for num in first_row.iter() { active.push(*num); }

                // process each row above the bottom row
                while let Some(row) = stack.pop() {
                    for (i, num) in row.iter().enumerate() {
                        active[i] = num + max(active[i], active[i + 1]);
                    }

                    active.pop();
                }

                // we assume that there is at least one value in here, or one
                // of the other error conditions should have gone off
                Ok(*active.iter().max().unwrap())

            } else {
                Err("No rows input")
            }
        }) // end Result map
    }

    use std::path::Path;
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::string::String;

    pub fn load_data(path: &Path) -> io::Result<Vec<i64>> {
        {
            let mut data = Vec::new();
            File::open(path).and_then(|mut file| {
                file.read_to_end(&mut data).map(|_| data)
            })

        }.map(|data| {
            let string = unsafe {
                String::from_utf8_unchecked(data)
            };

            string.split_whitespace().map(|substr| {
                i64::from_str_radix(substr, 10).unwrap()
            }).collect()
        })
    }
}

use smallest_path_sum::*;
use std::path::Path;

fn main() {
    let result = load_data(Path::new("data.txt")).map(|data| {
        println!("{:?}", smallest_path_sum(&data));
        ()
    });

    println!("{:?}", result);
}
