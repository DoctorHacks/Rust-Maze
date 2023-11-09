mod maze_operations {
    extern crate rand;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::fmt;

    pub struct Maze {
        // tuples
        dimensions: (usize, usize), // (height, width)
        entrypoint: (usize, usize), // (x, y) of start
        goalpoint: (usize, usize),  // (x, y) of end
        cells: Vec<Vec<Cell>>,
    }

    #[derive(Clone)] // required because vec![] uses .clone() on cell structs
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
            // Mazes only work well with odd-number dimensions
            // Example of name shadowing--you can redeclare variables in the
            // same scope
            // Also an example of how type names are optional if the compiler
            // can infer them
            let height = if height % 2 == 0 { height + 1 } else { height };
            let width = if width % 2 == 0 { width + 1 } else { width };

            let cells: Vec<Vec<Cell>> = vec![
                vec![
                    Cell {
                        // match statements must enumerate all variants in arms
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

        fn gen_from_walk(mut cells: Vec<Vec<Cell>>, dimensions: (usize, usize)) -> Self {
            let entrypoint: (usize, usize) = (0, 1);
            cells.get_mut(0).unwrap().get_mut(1).unwrap().wall = false;

            let goalpoint: (usize, usize) = (dimensions.0 - 1, dimensions.1 - 2);
            cells
                .get_mut(goalpoint.0)
                .unwrap()
                .get_mut(goalpoint.1)
                .unwrap()
                .wall = false;

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
            let current: &mut Cell = cells.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap();
            current.wall = false;
            current.visited = true;

            use Direction::*;
            // the size of regular arrays must be known at compile-time
            let directions: [Direction; 4] = Self::shuffle_directions();
            for direction in directions {
                match direction {
                    North => {
                        // if going North isn't on the outer wall or out-of-bounds
                        if pos.1 as isize - 2 > 0 {
                            // access cell that's 2 North of current cell (these should always work)
                            if let Some(row) = cells.get_mut(pos.0) {
                                if let Some(cell) = row.get_mut(pos.1 - 2) {
                                    // if cell 2 North of current pos not visited, remove wall
                                    // between and walk from the cell 2 North of current pos
                                    if !cell.visited {
                                        cells
                                            .get_mut(pos.0)
                                            .unwrap()
                                            .get_mut(pos.1 - 1)
                                            .unwrap()
                                            .wall = false;
                                        Self::walk(
                                            cells,
                                            (pos.0, pos.1 - 2),
                                            (dimensions.0, dimensions.1),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    East => {
                        if pos.0 + 2 < dimensions.0 - 1 {
                            if let Some(row) = cells.get_mut(pos.0 + 2) {
                                if let Some(cell) = row.get_mut(pos.1) {
                                    if !cell.visited {
                                        cells
                                            .get_mut(pos.0 + 1)
                                            .unwrap()
                                            .get_mut(pos.1)
                                            .unwrap()
                                            .wall = false;
                                        Self::walk(
                                            cells,
                                            (pos.0 + 2, pos.1),
                                            (dimensions.0, dimensions.1),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    South => {
                        if pos.1 + 2 < dimensions.1 - 1 {
                            if let Some(row) = cells.get_mut(pos.0) {
                                if let Some(cell) = row.get_mut(pos.1 + 2) {
                                    if !cell.visited {
                                        cells
                                            .get_mut(pos.0)
                                            .unwrap()
                                            .get_mut(pos.1 + 1)
                                            .unwrap()
                                            .wall = false;
                                        Self::walk(
                                            cells,
                                            (pos.0, pos.1 + 2),
                                            (dimensions.0, dimensions.1),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    West => {
                        if pos.0 as isize - 2 > 0 {
                            if let Some(row) = cells.get_mut(pos.0 - 2) {
                                if let Some(cell) = row.get_mut(pos.1) {
                                    if !cell.visited {
                                        cells
                                            .get_mut(pos.0 - 1)
                                            .unwrap()
                                            .get_mut(pos.1)
                                            .unwrap()
                                            .wall = false;
                                        Self::walk(
                                            cells,
                                            (pos.0 - 2, pos.1),
                                            (dimensions.0, dimensions.1),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
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
                dimensions: (cells.len(), cells.get(0).unwrap().len()),
                goalpoint: (cells.len(), cells.get(0).unwrap().len() - 1),
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
    let maze: Maze = Maze::new(15, 30, CreationAlgorithm::RandomWalk);
    println!("{}", maze);
}
