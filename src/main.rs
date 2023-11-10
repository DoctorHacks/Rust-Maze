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
        Prim,
        Debug,
    }

    pub enum SolvingAlgorithm {
        RecursiveBacktracking,
        Tremaux,
    }

    #[derive(Clone, Copy)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    impl Maze {
        pub fn new(dimensions: (usize, usize), algorithm: CreationAlgorithm) -> Self {
            use CreationAlgorithm::*;
            // Mazes smaller than 3x3 don't make sense (dimension == 2 case handled below)
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
                Prim => Self::gen_from_prim(cells),
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
                // if the area to divide is wider than it is tall (same procedure as above)
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

        fn gen_from_prim(mut cells: Vec<Vec<Cell>>) -> Self {
            let dimensions: (usize, usize) = (cells.len(), cells[0].len());
            let entrypoint: (usize, usize) = (1, 0);
            let goalpoint: (usize, usize) = (cells.len() - 2, cells[0].len() - 1);

            cells[1][1].wall = false;
            let mut rng = rand::thread_rng();
            let mut frontiers: Vec<(usize, usize)> = vec![];

            // Compute frontier cells of (1, 1) and add them to the vector
            Self::append_frontiers(&cells, &mut frontiers, (1, 1));

            use Direction::*;
            // While the list of frontier cells is not empty:
            while frontiers.len() > 0 {
                // Pick a random cell from the list. Mark it as not a wall.
                let rand_frontier_index: usize = rng.gen_range(0..frontiers.len());
                let current: (usize, usize) = frontiers[rand_frontier_index];
                cells[current.0][current.1].wall = false;

                // Let its neighbors be all the cells 2 apart from it that aren't walls.
                let mut neighbors: Vec<Direction> = vec![];
                if current.0 as isize - 2 > 0 && !cells[current.0 - 2][current.1].wall {
                    neighbors.push(North);
                }
                if current.0 + 2 < dimensions.0 - 1 && !cells[current.0 + 2][current.1].wall {
                    neighbors.push(South);
                }
                if current.1 + 2 < dimensions.1 - 1 && !cells[current.0][current.1 + 2].wall {
                    neighbors.push(East);
                }
                if current.1 as isize - 2 > 0 && !cells[current.0][current.1 - 2].wall {
                    neighbors.push(West);
                }

                // Pick a random neighbor and connect the randomly chosen frontier cell with its
                // neighbor by setting the cell in-between them to not a wall.
                let neighbor: Direction = neighbors[rng.gen_range(0..neighbors.len())];
                match neighbor {
                    North => {
                        cells[current.0 - 1][current.1].wall = false;
                    }
                    South => {
                        cells[current.0 + 1][current.1].wall = false;
                    }
                    East => {
                        cells[current.0][current.1 + 1].wall = false;
                    }
                    West => {
                        cells[current.0][current.1 - 1].wall = false;
                    }
                }

                // Compute the frontier cells of the randomly chosen frontier cell and add them to
                // the list, if they aren't already in the list.
                Self::append_frontiers(&cells, &mut frontiers, current);

                // remove the randomly chosen frontier cell from the list.
                frontiers.remove(rand_frontier_index);
            }

            Maze {
                dimensions,
                entrypoint,
                goalpoint,
                cells,
            }
        }

        fn append_frontiers(
            cells: &Vec<Vec<Cell>>,
            frontiers: &mut Vec<(usize, usize)>,
            pos: (usize, usize),
        ) {
            // North frontier
            if pos.0 as isize - 2 > 0
                && cells[pos.0 - 2][pos.1].wall
                && !frontiers.contains(&(pos.0 - 2, pos.1))
            {
                frontiers.push((pos.0 - 2, pos.1));
            }
            // South frontier
            if pos.0 + 2 < cells.len() - 1
                && cells[pos.0 + 2][pos.1].wall
                && !frontiers.contains(&(pos.0 + 2, pos.1))
            {
                frontiers.push((pos.0 + 2, pos.1));
            }
            // East frontier
            if pos.1 + 2 < cells[0].len() - 1
                && cells[pos.0][pos.1 + 2].wall
                && !frontiers.contains(&(pos.0, pos.1 + 2))
            {
                frontiers.push((pos.0, pos.1 + 2));
            }
            // West frontier
            if pos.1 as isize - 2 > 0
                && cells[pos.0][pos.1 - 2].wall
                && !frontiers.contains(&(pos.0, pos.1 - 2))
            {
                frontiers.push((pos.0, pos.1 - 2));
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
                entrypoint: (1, 0),
                goalpoint: (cells.len() - 2, cells[0].len() - 1),
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
    let maze: Maze = Maze::new((25, 25), CreationAlgorithm::Prim);
    println!("{}", maze);
}
