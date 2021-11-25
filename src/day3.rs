use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct Slope {
    layout: Vec<Vec<bool>>,
    cursor: (i32, i32),
    vector: (i32, i32),
}

impl Slope {
    pub fn new(filename: &str) -> Slope {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut rows = Vec::<Vec<bool>>::new();

        for line in reader.lines() {
            let line = line.expect("Failed to read");
            let mut row = Vec::<bool>::new();
            for i in line.chars() {
                row.push(match i {
                    '#' => true,
                    _ => false,
                })
            }
            rows.push(row);
        }

        Slope {
            layout: rows,
            cursor: (0, 0),
            vector: (1, 3),
        }
    }

    fn iterate(&mut self) -> Option<bool> {
        let height = self.layout.len() as i32;
        let width = self.layout[0].len() as i32;
        if self.cursor.0 + self.vector.0 < height {
            self.cursor.0 = self.cursor.0 + self.vector.0;
            self.cursor.1 = self.cursor.1 + self.vector.1;
            if self.cursor.1 > (width - 1) {
                self.cursor.1 = self.cursor.1 - width;
            }
            Some(self.layout[self.cursor.0 as usize][self.cursor.1 as usize])
        } else {
            None
        }
    }

    pub fn iterate_to_completion(&mut self) -> u64 {
        let mut complete = false;
        let mut trees: u64 = 0;
        while !complete {
            let next = self.iterate();
            trees = trees + match next {
                Some(x) => {
                    if x {
                        1
                    } else {
                        0
                    }
                },
                None => 0,
            };
            if next.is_none() {
                complete = true;
            }
        }
        trees
    }

    pub fn display(&self) {
        for i in &self.layout {
            for j in i {
                print!("{}", match j {
                    true => "#",
                    false => ".",
                })
            }
            print!("\n");
        }
    }


    pub fn set_vector(&mut self, y: i32, x: i32) {
        self.vector.0 = y;
        self.vector.1 = x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope_new() {
        let mut slope = Slope::new("Data/day3_example.txt");
        assert_eq!(slope.layout[0].len(), 11);
        assert_eq!(slope.layout.len(), 11);
    }

    #[test]
    fn test_slope_iterate() {
        let mut slope = Slope::new("Data/day3_example.txt");

        let first_move = slope.iterate();
        assert!(first_move.is_some());
        assert_eq!(first_move.unwrap(), false);

        let second_move = slope.iterate();
        assert!(second_move.is_some());
        assert_eq!(second_move.unwrap(), true);

        for i in 0..9 {
            slope.iterate();
        }

        assert_eq!(slope.iterate(), None);
    }

    #[test]
    fn test_iterate_to_completion() {
        let mut slope = Slope::new("Data/day3_example.txt");

        let iterations = slope.iterate_to_completion();

        assert_eq!(iterations, 7);
    }

    #[test]
    fn test_alternate_vector() {
        let vectors: [(i32,i32); 5] = [
            (1, 1),
            (1, 3),
            (1, 5),
            (1, 7),
            (2, 1),
        ];

        let mut total_trees = 1;
        for vector in vectors {
            let mut slope = Slope::new("Data/day3_example.txt");
            slope.set_vector(vector.0, vector.1);
            let trees = slope.iterate_to_completion();
            println!("Vector: {},{} -> {}", vector.0, vector.1, trees);
            total_trees = total_trees * trees;
        }


        assert_eq!(total_trees, 336);
    }
}