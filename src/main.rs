/**
 * A module containing a Maze struct, capable of generating, representing, and solving a
 * two-dimensional labyrinth with exactly one path from the entry point at the top-left to the goal
 * point at the bottom-right.
 *
 * Authors: Brandon Ikeler, Travis Hahn
 */
extern crate colored;
extern crate rand;

mod maze_operations {
    use colored::*;
    use rand::{seq::SliceRandom, thread_rng, Rng};
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
        pub fn new(dimensions: (usize, usize), algorithm: CreationAlgorithm) -> Self {
            use CreationAlgorithm::*;
            // Mazes smaller than 3x3 don't make sense (dimensions.0/1 == 2 case handled below)
            if dimensions.0 < 2 || dimensions.1 < 2 {
                panic!("Can't create a maze this small")
            }
            // Mazes only work well with odd-number dimensions
            let height = if dimensions.0 % 2 == 0 {
                dimensions.0 + 1
            } else {
                dimensions.0
            };
            let width = if dimensions.1 % 2 == 0 {
                dimensions.1 + 1
            } else {
                dimensions.1
            };

            let cells: Vec<Vec<Cell>> = vec![
                vec![
                    Cell {
                        // match statements must enumerate all variants in their arms
                        wall: match algorithm {
                            // Recursive division starts with a grid of paths and constricts the
                            // movable spaces with new walls
                            RecursiveDivision => false,
                            // All other algorithms start with a grid of walls and carve out a path
                            _ => true,
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
            // these are declared up here, because if their values were copied...
            let height = cells.len();
            let width = cells[0].len();

            // walls on top and bottom
            for i in 0..cells[0].len() {
                cells[0][i].wall = true;
                // here...
                cells[height - 1][i].wall = true;
            }

            // walls on left and right
            for row in &mut cells {
                row[0].wall = true;
                // or here,
                row[width - 1].wall = true;
                // it would cause a compilation error, because the loops borrow cells (or cells[0])
                // mutably, and cells.len() (or cells[0].len()) borrows cells immutably, and we
                // can't have a mutable and an immutable reference to cells (or cells[0]) active at
                // the same time
            }

            Self::divide(&mut cells, (0, 0), (height - 1, width - 1));

            Maze {
                dimensions: (height, width),
                goalpoint: (height - 2, width - 1),
                entrypoint: (1, 0),
                cells,
            }
        }

        fn divide(
            cells: &mut Vec<Vec<Cell>>,
            top_left: (usize, usize),
            bottom_right: (usize, usize),
        ) {
            let height = bottom_right.0 - top_left.0;
            let width = bottom_right.1 - top_left.1;

            // we can't divide further if the area to divide is already 1 cell narrow
            if height == 2 || width == 2 {
                return;
            }

            let mut rng = rand::thread_rng();
            // if the area to divide is at least as tall as it is wide, draw a new horizontal wall
            // on a random even index that divides the area, and punch a hole in it at a random odd
            // index to retain the maze's connectedness
            if height >= width {
                // ranges exclude the max value
                let mut wall_index = rng.gen_range((top_left.0 + 1)..bottom_right.0);
                while wall_index % 2 != 0 {
                    wall_index = rng.gen_range((top_left.0 + 1)..bottom_right.0);
                }

                for i in (top_left.1 + 1)..bottom_right.1 {
                    cells[wall_index][i].wall = true;
                }

                let mut hole_index = rng.gen_range((top_left.1 + 1)..bottom_right.1);
                while hole_index % 2 != 1 {
                    hole_index = rng.gen_range((top_left.1 + 1)..bottom_right.1);
                }

                cells[wall_index][hole_index].wall = false;
                Self::divide(cells, top_left, (wall_index, bottom_right.1));
                Self::divide(cells, (wall_index, top_left.1), bottom_right);
            } else {
                let mut wall_index = rng.gen_range((top_left.1 + 1)..bottom_right.1);
                while wall_index % 2 != 0 {
                    wall_index = rng.gen_range((top_left.1 + 1)..bottom_right.1);
                }

                for i in (top_left.0 + 1)..bottom_right.0 {
                    cells[i][wall_index].wall = true;
                }

                let mut hole_index = rng.gen_range((top_left.0 + 1)..bottom_right.0);
                while hole_index % 2 != 1 {
                    hole_index = rng.gen_range((top_left.0 + 1)..bottom_right.0);
                }

                cells[hole_index][wall_index].wall = false;
                Self::divide(cells, top_left, (bottom_right.0, wall_index));
                Self::divide(cells, (top_left.0, wall_index), bottom_right);
            }
        }

        fn shuffle_directions() -> [Direction; 4] {
            use Direction::*;
            let mut directions = [North, South, East, West];
            let mut rng = thread_rng();
            directions.shuffle(&mut rng);
            directions
        }

        fn debug(cells: Vec<Vec<Cell>>) -> Self {
            Maze {
                dimensions: (cells.len(), cells[0].len()),
                goalpoint: (cells.len(), cells[0].len() - 1),
                entrypoint: (0, 0),
                cells, // if you specify cells before dimensions or goal, it will not work, because
                       // because the Maze struct takes ownership of cells, which means cells goes
                       // out of scope in debug()
                       // This is field init shorthand syntax for: cells: cells,by the way--when you
                       // specify a variable for initializing a struct, if the variable is the same
                       // name as the struct attribute, you only have to specify the name of the
                       // local variable/struct attribute once
            }
        }
    }

    impl fmt::Display for Maze {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (y, row) in self.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    write!(f, "{}", {
                        // No idea why, but Unicode 00A0 (NO-BREAK SPACE) was the only Unicode
                        // character I could find that would behave predictably when resizing the
                        // terminal window
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
                write!(f, "{}", "\u{00A0}\n".clear())?;
            }
            Ok(())
        }
    }
}

use maze_operations::*;
fn main() {
    let maze: Maze = Maze::new((25, 35), CreationAlgorithm::RecursiveDivision);
    println!("{}", maze);
}
