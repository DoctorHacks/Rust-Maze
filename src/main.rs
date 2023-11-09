/**
 * A module containing a Maze struct, capable of representing a two-dimensional labyrinth with
 * exactly one path from the entry point at the top-left to the goal point at the bottom-right.
 *
 * Authors: Brandon Ikeler, Travis Hahn
 */
extern crate colored;
extern crate rand;

mod maze_operations {
    use colored::*;
    use rand::{seq::SliceRandom, thread_rng};
    use std::fmt;

    #[derive(Debug)]
    pub struct Maze {
        dimensions: (usize, usize), // (height, width)
        entrypoint: (usize, usize), // (y, x) of start
        goalpoint: (usize, usize),  // (y, x) of end
        cells: Vec<Vec<Cell>>,
    }

    #[derive(Clone, Debug)] // required because vec![] uses .clone() on cell structs
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

    enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Maze {
        pub fn new(height: usize, width: usize, algorithm: CreationAlgorithm) -> Self {
            use CreationAlgorithm::*;
            if height < 2 || width < 2 {
                panic!("Can't create a maze this small")
            }
            // Mazes only work well with odd-number dimensions
            // Example of name shadowing--you can redeclare variables in the same scope
            // Also an example of how type names are optional if the compiler can infer them
            let height = if height % 2 == 0 { height + 1 } else { height };
            let width = if width % 2 == 0 { width + 1 } else { width };

            let cells: Vec<Vec<Cell>> = vec![
                vec![
                    Cell {
                        // match statements must enumerate all variants in their arms
                        wall: match algorithm {
                            RandomWalk => true,
                            Debug => true,
                            _ => false,
                        },
                        visited: false
                    };
                    width
                ];
                height
            ];

            match algorithm {
                RandomWalk => Self::gen_from_walk(cells, (height, width)),
                RecursiveDivision => Self::gen_from_divide(cells),
                Debug => Self::debug(cells),
            }
        }

        fn shuffle_directions() -> [Direction; 4] {
            use Direction::*;
            let mut directions = [North, South, East, West];
            let mut rng = thread_rng();
            directions.shuffle(&mut rng);
            directions
        }

        fn gen_from_walk(mut cells: Vec<Vec<Cell>>, dimensions: (usize, usize)) -> Self {
            let entrypoint: (usize, usize) = (1, 0);
            cells[entrypoint.0][entrypoint.1].wall = false;

            let goalpoint: (usize, usize) = (dimensions.0 - 2, dimensions.1 - 1);
            cells[goalpoint.0][goalpoint.1].wall = false;

            Self::walk(&mut cells, (1, 1), (dimensions.0, dimensions.1));

            for row in &mut cells {
                for cell in row {
                    cell.visited = false;
                }
            }

            Maze {
                dimensions,
                entrypoint,
                goalpoint,
                cells,
            }
        }

        fn walk(cells: &mut Vec<Vec<Cell>>, pos: (usize, usize), dimensions: (usize, usize)) {
            // Remove wall at current cell and mark it as visited
            let current: &mut Cell = &mut cells[pos.0][pos.1];
            current.wall = false;
            current.visited = true;

            use Direction::*;
            // the size of regular arrays must be known at compile-time
            let directions: [Direction; 4] = Self::shuffle_directions();
            for direction in directions {
                match direction {
                    North => {
                        // if the cell 2 positions North isn't on the border or OOB and it hasn't
                        // been visited yet, remove the wall 1 position North separating this cell
                        // from the current cell and walk from there
                        if pos.0 as isize - 2 > 0 && !cells[pos.0 - 2][pos.1].visited {
                            cells[pos.0 - 1][pos.1].wall = false;
                            Self::walk(cells, (pos.0 - 2, pos.1), (dimensions.0, dimensions.1));
                        }
                    }
                    South => {
                        if pos.0 + 2 < dimensions.0 - 1 && !cells[pos.0 + 2][pos.1].visited {
                            cells[pos.0 + 1][pos.1].wall = false;
                            Self::walk(cells, (pos.0 + 2, pos.1), (dimensions.0, dimensions.1));
                        }
                    }
                    East => {
                        if pos.1 + 2 < dimensions.1 - 1 && !cells[pos.0][pos.1 + 2].visited {
                            cells[pos.0][pos.1 + 1].wall = false;
                            Self::walk(cells, (pos.0, pos.1 + 2), (dimensions.0, dimensions.1));
                        }
                    }
                    West => {
                        if pos.1 as isize - 2 > 0 && !cells[pos.0][pos.1 - 2].visited {
                            cells[pos.0][pos.1 - 1].wall = false;
                            Self::walk(cells, (pos.0, pos.1 - 2), (dimensions.0, dimensions.1));
                        }
                    }
                }
            }
        }

        fn gen_from_divide(mut cells: Vec<Vec<Cell>>) -> Self {
            unimplemented!();
        }

        fn debug(cells: Vec<Vec<Cell>>) -> Self {
            Maze {
                dimensions: (cells.len(), cells[0].len()),
                goalpoint: (cells.len(), cells[0].len() - 1),
                entrypoint: (0, 0),
                cells, // if you specify cells before dimensions or goal, it
                       // will not work, because the Maze struct takes ownership
                       // of cells, which means cells goes out of scope in
                       // debug()
                       // This is field init shorthand syntax for: cells: cells,
                       // by the way--when you specify a variable for
                       // initializing a struct, if the variable is the same
                       // name as the struct attribute, you only have to specify
                       // the name of the local variable/struct attribute once
            }
        }
    }

    impl fmt::Display for Maze {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (y, row) in self.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    write!(f, "{}", {
                        if (y, x) == self.entrypoint {
                            "\u{00A0}\u{00A0}".on_red()
                        } else if (y, x) == self.goalpoint {
                            "\u{00A0}\u{00A0}".on_green()
                        } else if cell.wall {
                            "\u{00A0}\u{00A0}".on_white()
                        } else if cell.visited {
                            "\u{00A0}\u{00A0}".on_blue()
                        } else {
                            "\u{00A0}\u{00A0}".on_black()
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
    let maze: Maze = Maze::new(21, 21, CreationAlgorithm::RandomWalk);
    println!("{}", maze);
}
