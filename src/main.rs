/*
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
    }

    pub enum SolvingAlgorithm {
        RecursiveBacktracking,
        WallHugging,
    }

    #[derive(Clone, Copy)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    /*
     * Creates a new Maze of specified size and with the specified algorithm for doing so.
     * Expects the dimensions to be at least 3x3, and each should be odd; if an even number is
     * passed, the dimension will be incremented by 1 (for example, trying to create a 10x10 Maze
     * will result in an 11x11 Maze).
     */
    impl Maze {
        pub fn new(dimensions: (usize, usize), algorithm: CreationAlgorithm) -> Self {
            use CreationAlgorithm::*;
            // mazes smaller than 3x3 don't make sense
            if dimensions.0 <= 2 || dimensions.1 <= 2 {
                panic!("Can't create a maze this small")
            }
            // mazes only work well with odd-number dimensions
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
                            // recursive division starts with a grid of paths and constricts the
                            // movable spaces with new walls
                            RecursiveDivision => false,
                            // all other algorithms start with a grid of walls and carve out a path
                            _ => true,
                        },
                        visited: false
                    };
                    width
                ];
                height
            ];

            match algorithm {
                RandomWalk => Self::gen_from_walk(cells),
                RecursiveDivision => Self::gen_from_divide(cells),
                Prim => Self::gen_from_prim(cells),
            }
        }

        /*
         * Solves this Maze--sets the visited bool of each Cell on the way to the goalpoint to true.
         */
        pub fn solve(&mut self, algorithm: SolvingAlgorithm) {
            use SolvingAlgorithm::*;
            match algorithm {
                RecursiveBacktracking => self.solve_from_backtracking(self.entrypoint),
                WallHugging => self.solve_from_hugging(self.entrypoint),
            };
        }

        /*
         * Generates a Maze using a random non-self-intersecting walk, beginning at a
         * randomly-selected cell.
         */
        fn gen_from_walk(mut cells: Vec<Vec<Cell>>) -> Self {
            let dimensions: (usize, usize) = (cells.len(), cells[0].len());

            let entrypoint: (usize, usize) = (1, 0);
            cells[entrypoint.0][entrypoint.1].wall = false;

            let goalpoint: (usize, usize) = (dimensions.0 - 2, dimensions.1 - 1);
            cells[goalpoint.0][goalpoint.1].wall = false;

            // random starting point
            let mut rng = thread_rng();
            let mut starter: (usize, usize) = (
                rng.gen_range(1..(dimensions.0 - 1)),
                rng.gen_range(1..(dimensions.1 - 1)),
            );
            loop {
                if starter.0 % 2 == 0 {
                    starter.0 = rng.gen_range(1..(dimensions.0 - 1));
                } else if starter.1 % 2 == 0 {
                    starter.1 = rng.gen_range(1..(dimensions.1 - 1));
                } else {
                    break;
                }
            }

            Self::walk(&mut cells, starter, (dimensions.0, dimensions.1));

            // set each cell back to unvisited, so the Maze will print properly
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

        /*
         * Recursively steps forward in a non-self-intersecting random walk and carves out the path
         * in the Maze as it goes.
         */
        fn walk(cells: &mut Vec<Vec<Cell>>, pos: (usize, usize), dimensions: (usize, usize)) {
            // remove wall at current cell and mark it as visited
            let current: &mut Cell = &mut cells[pos.0][pos.1];
            current.wall = false;
            current.visited = true;

            use Direction::*;
            // the size of raw arrays must be known at compile-time
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

        /*
         * Generates a Maze by recursively adding walls at random positions that divide the
         * remaining accessible area.
         */
        fn gen_from_divide(mut cells: Vec<Vec<Cell>>) -> Self {
            let dimensions: (usize, usize) = (cells.len(), cells[0].len());

            let entrypoint: (usize, usize) = (1, 0);

            let goalpoint: (usize, usize) = (dimensions.0 - 2, dimensions.1 - 1);

            // walls on top and bottom
            for i in 0..cells[0].len() {
                cells[0][i].wall = true;
                cells[dimensions.0 - 1][i].wall = true;
            }

            // walls on left and right
            for row in &mut cells {
                row[0].wall = true;
                row[dimensions.1 - 1].wall = true;
            }

            cells[entrypoint.0][entrypoint.1].wall = false;
            cells[goalpoint.0][goalpoint.1].wall = false;

            Self::divide(&mut cells, (0, 0), (dimensions.0 - 1, dimensions.1 - 1));

            Maze {
                dimensions,
                goalpoint,
                entrypoint,
                cells,
            }
        }

        /*
         * Recursively the specified section of the Maze in half, leaving a hole in the wall to
         * maintain access between the left/right or top/bottom halves.
         */
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

            let mut rng = thread_rng();
            // if the area to divide is at least as tall as it is wide, draw a new horizontal wall
            // on a random even index that divides the area, and punch a hole in it at a random odd
            // index to retain the maze's connectedness
            if height >= width {
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

        /*
         * Iteratively generates a Maze using iterative randomized Prim's algorithm. The starting
         * point is randomly selected.
         */
        fn gen_from_prim(mut cells: Vec<Vec<Cell>>) -> Self {
            let dimensions: (usize, usize) = (cells.len(), cells[0].len());

            let entrypoint: (usize, usize) = (1, 0);
            cells[entrypoint.0][entrypoint.1].wall = false;

            let goalpoint: (usize, usize) = (dimensions.0 - 2, dimensions.1 - 1);
            cells[goalpoint.0][goalpoint.1].wall = false;

            // compute frontier cells of a random (odd, odd) cell and add them to a list
            let mut frontiers: Vec<(usize, usize)> = vec![];
            let mut rng = thread_rng();
            let mut starter: (usize, usize) = (
                rng.gen_range(1..(dimensions.0 - 1)),
                rng.gen_range(1..(dimensions.0 - 1)),
            );
            loop {
                if starter.0 % 2 == 0 {
                    starter.0 = rng.gen_range(1..(dimensions.0 - 1));
                } else if starter.1 % 2 == 0 {
                    starter.1 = rng.gen_range(1..(dimensions.1 - 1));
                } else {
                    break;
                }
            }
            cells[starter.0][starter.1].wall = false;
            Self::append_frontiers(&cells, &mut frontiers, starter);

            use Direction::*;
            // while the list of frontier cells is not empty:
            while frontiers.len() > 0 {
                // pick a random frontier cell from the list. Mark it as not a wall.
                let rand_frontier_index: usize = rng.gen_range(0..frontiers.len());
                let current: (usize, usize) = frontiers[rand_frontier_index];
                cells[current.0][current.1].wall = false;

                // let its neighbors be all the cells 2 apart from it that aren't walls
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

                // pick a random neighbor and connect the randomly chosen frontier cell with its
                // neighbor by setting the cell in-between them to not a wall
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

                // compute the frontier cells of the randomly chosen frontier cell and add them to
                // the list, if they aren't already in the list
                Self::append_frontiers(&cells, &mut frontiers, current);

                // remove the (current) randomly chosen frontier cell from the list
                frontiers.remove(rand_frontier_index);
            }

            Maze {
                dimensions,
                entrypoint,
                goalpoint,
                cells,
            }
        }

        /*
         * Accepts a reference to the current (mid-Prim generation) state of the Maze and a position
         * within, and appends the frontier cells of this position to the list of frontier cells if
         * the cell doesn't already appear in the list.
         */
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

        /*
         * Returns an array of length 4 of the cardinal directions in random order. Used by the
         * random walk algorithm to determine which way to go next.
         */
        fn shuffle_directions() -> [Direction; 4] {
            use Direction::*;
            let mut directions = [North, South, East, West];
            let mut rng = thread_rng();
            directions.shuffle(&mut rng);
            directions
        }

        /*
         * Solves this Maze via recursive depth-first search. Returns true as long as the Maze has
         * been successfully solved--false otherwise, though this should never occur.
         */
        fn solve_from_backtracking(&mut self, pos: (usize, usize)) -> bool {
            // base case: if we're on a wall, or a path we've been before, we can't be going towards
            // the solution
            if self.cells[pos.0][pos.1].wall || self.cells[pos.0][pos.1].visited {
                return false;
            }

            // mark current cell as visited
            self.cells[pos.0][pos.1].visited = true;

            // base case: if we're at the goalpoint, no more work needs to be done
            if pos == self.goalpoint {
                return true;
            }

            // recursive case: try going each direction
            // this function's return type--bool--stops future recursion branches as it unwinds; we
            // take advantage of short-circuit boolean evaluation
            ({
                // South
                if pos.0 + 1 < self.dimensions.0 {
                    self.solve_from_backtracking((pos.0 + 1, pos.1))
                } else {
                    false
                }
            }) || ({
                // East
                if pos.1 + 1 < self.dimensions.1 {
                    self.solve_from_backtracking((pos.0, pos.1 + 1))
                } else {
                    false
                }
            }) || ({
                // North
                if pos.0 as isize - 1 > 0 {
                    self.solve_from_backtracking((pos.0 - 1, pos.1))
                } else {
                    false
                }
            }) || ({
                // West
                if pos.1 as isize - 1 > 0 {
                    self.solve_from_backtracking((pos.0, pos.1 - 1))
                } else {
                    false
                }
            }) || ({
                // This cell isn't on the path to the solution, since going each direction results
                // in a dead end
                self.cells[pos.0][pos.1].visited = false;
                false
            })
        }

        /*
         * Solves this Maze by always hugging the wall to the right. This always results in a
         * solution, because the Mazes that can be generated using the algorithms we've implemented
         * are simply connected.
         */
        fn solve_from_hugging(&self, pos: (usize, usize)) -> bool {
            unimplemented!()
        }
    }

    /*
     * Formats a Maze to be pretty-printable with the println!() macro.
     */
    impl fmt::Display for Maze {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (y, row) in self.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    write!(f, "{}", {
                        // no idea why, but Unicode 00A0 (NO-BREAK SPACE) was the only Unicode
                        // character we could find that would print the highlight color and still
                        // behave predictably when resizing the terminal window
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
    let mut maze: Maze = Maze::new((15, 15), CreationAlgorithm::RecursiveDivision);
    println!("{}", maze);
    maze.solve(SolvingAlgorithm::RecursiveBacktracking);
    println!("{}", maze);
}
