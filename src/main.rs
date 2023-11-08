mod maze_operations {
    extern crate rand;
    use rand::Rng;
    use std::fmt;

    pub struct Maze {
        dimensions: (usize, usize), // (height, width)
        entrypoint: (u8, u8),
        goalpoint: (u8, u8),
        cells: Vec<Vec<Cell>>,
    }

    #[derive(Clone)] // required because vec![] uses .clone()
    struct Cell {
        wall: bool,
        visited: bool,
    }

    pub enum CreationAlgorithm {
        RandomWalk,
        RecursiveDivision,
        Debug,
    }

    pub enum SolvingAlgorithm {
        RecursiveBacktracking,
        Tremaux,
    }

    impl Maze {
        pub fn new(height: usize, width: usize, algorithm: CreationAlgorithm) -> Self {
            // Mazes only work well with odd-number dimensions
            // Example of name shadowing--you can redeclare variables in the
            // same scope
            let height = if height % 2 == 0 { height + 1 } else { height };
            let width = if width % 2 == 0 { width + 1 } else { width };

            let cells: Vec<Vec<Cell>> = vec![
                vec![
                    Cell {
                        wall: match algorithm {
                            CreationAlgorithm::RandomWalk => true,
                            CreationAlgorithm::Debug => true,
                            _ => false,
                        },
                        visited: false
                    };
                    width
                ];
                height
            ];

            match algorithm {
                CreationAlgorithm::RandomWalk => Self::gen_from_walk(cells),
                CreationAlgorithm::RecursiveDivision => Self::gen_from_divide(cells),
                CreationAlgorithm::Debug => Self::debug(cells),
            }
        }

        fn gen_from_walk(mut cells: Vec<Vec<Cell>>) -> Self {
            let entrypoint: (u8, u8) = (0, 1);
            Self::walk(&mut cells, (1, 1));
            unimplemented!()
        }

        fn walk(cells: &mut Vec<Vec<Cell>>, pos: (u8, u8)) {
            cells
                .get_mut(pos.0 as usize)
                .unwrap()
                .get_mut(pos.1 as usize)
                .unwrap()
                .visited = true;
            unimplemented!()
        }

        fn debug(cells: Vec<Vec<Cell>>) -> Self {
            Maze {
                dimensions: (cells.len(), cells.get(0).unwrap().len()),
                goalpoint: (cells.len() as u8, cells.get(0).unwrap().len() as u8 - 1),
                entrypoint: (0, 0),
                cells, // if you specify cells before dimensions or goal, it
                       // will not work, because the Maze struct takes ownership
                       // of cells, which means cells goes out of scope in
                       // walk()
                       // This is field init shorthand syntax for: cells: cells,
                       // by the way--when you specify a variable for
                       // initializing a struct, if the variable is the same
                       // name as the struct attribute, you only have to specify
                       // the name of the local variable/struct attribute once
            }
        }

        fn gen_from_divide(mut cells: Vec<Vec<Cell>>) -> Self {
            unimplemented!();
        }
    }

    impl fmt::Display for Maze {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for row in &self.cells {
                for cell in row {
                    write!(f, "{}", {
                        if cell.wall {
                            "#"
                        } else if cell.visited {
                            "."
                        } else {
                            " "
                        }
                    })?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }
}

use maze_operations::*;
fn main() {
    let maze: Maze = Maze::new(5, 5, CreationAlgorithm::Debug);
    println!("{}", maze);
}
