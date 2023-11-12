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

    pub struct Maze {
        dimensions: (usize, usize), // (height, width)
        entrypoint: (usize, usize), // (y, x) of start
        goalpoint: (usize, usize),  // (y, x) of end
        cells: Vec<Vec<Cell>>,
    }

    #[derive(Clone)] // required because vec![] uses .clone() on cell structs
    struct Cell {
        wall: bool,
        visited: bool,
    }

    #[derive(Clone, Copy)]
    pub enum CreationAlgorithm {
        RandomWalk,
        RecursiveDivision,
        Prim,
    }

    #[derive(Clone, Copy)]
    pub enum SolvingAlgorithm {
        RecursiveBacktracking,
        DeadEndFilling,
    }

    impl Maze {
        /*
         * Creates a new Maze of specified size. Since the user didn't specify an algorithm, we've
         * opted to make the default the Prim algolrithm, since it's our only iterative generation
         * implementation (and we just think it's neat).
         */
        pub fn new(dimensions: (usize, usize)) -> Self {
            Self::new_from(dimensions, CreationAlgorithm::Prim)
        }

        /*
         * Creates a new Maze of specified size and with the specified algorithm for doing so.
         * Expects the dimensions to be at least 3x3, and each should be odd; if an even number is
         * passed, the dimension will be incremented by 1 (for example, trying to create a 10x10
         * Maze will result in an 11x11 Maze).
         */
        pub fn new_from(dimensions: (usize, usize), algorithm: CreationAlgorithm) -> Self {
            use CreationAlgorithm::*;
            // mazes smaller than 3x3 don't make sense
            if dimensions.0 <= 2 || dimensions.1 <= 2 {
                panic!("Can't create a maze this small")
            }
            // mazes only work well with odd-number dimensions
            let height = dimensions.0 + if dimensions.0 % 2 == 0 { 1 } else { 0 };
            let width = dimensions.1 + if dimensions.1 % 2 == 0 { 1 } else { 0 };

            let cells: Vec<Vec<Cell>> = vec![
                vec![
                    Cell {
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
         * Solves this Maze--sets the visited bool of each Cell on the way to the goalpoint to
         * true--using our default choice of recursive backtracking.
         */
        pub fn solve(&mut self) {
            self.solve_from_backtracking(self.entrypoint);
        }

        /*
         * Solves this Maze--sets the visited bool of each Cell on the way to the goalpoint to
         * true--using the specified algorithm for doing so.
         */
        pub fn solve_from(&mut self, algorithm: SolvingAlgorithm) {
            use SolvingAlgorithm::*;
            match algorithm {
                RecursiveBacktracking => self.solve_from_backtracking(self.entrypoint),
                DeadEndFilling => self.solve_from_dead_end_filling(),
            };
        }

        /*
         * Returns whether this Maze currently has the solution computed.
         */
        pub fn is_solved(&self) -> bool {
            self.cells[self.goalpoint.0][self.goalpoint.1].visited
        }

        /*
         * Removes the solution to this Maze.
         */
        pub fn unsolve(&mut self) {
            for row in &mut self.cells {
                for cell in row {
                    cell.visited = false;
                }
            }
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
            let mut starter: (usize, usize) = (0, 0);
            while starter.0 % 2 == 0 || starter.1 % 2 == 0 {
                starter = (
                    rng.gen_range(1..(dimensions.0 - 1)),
                    rng.gen_range(1..(dimensions.1 - 1)),
                );
            }

            /*
             * Recursively steps forward in a non-self-intersecting random walk and carves out the
             * path in the Maze as it goes.
             */
            fn walk(cells: &mut Vec<Vec<Cell>>, pos: (usize, usize), dimensions: (usize, usize)) {
                // remove wall at current cell and mark it as visited
                let current: &mut Cell = &mut cells[pos.0][pos.1];
                current.wall = false;
                current.visited = true;

                let get_two_offset_shuffle = || -> [(isize, isize); 4] {
                    let mut two_offsets = [(-2, 0), (2, 0), (0, -2), (0, 2)];
                    let mut rng = thread_rng();
                    two_offsets.shuffle(&mut rng);
                    two_offsets
                };

                let mut walk_to = |two_offset: (isize, isize)| {
                    let two_neighbor: (isize, isize) =
                        (pos.0 as isize + two_offset.0, pos.1 as isize + two_offset.1);

                    // if the cell specified by the offset is within the allowable area,
                    if two_neighbor.0 > 0
                        && two_neighbor.0 < dimensions.0 as isize - 1
                        && two_neighbor.1 > 0
                        && two_neighbor.1 < dimensions.1 as isize - 1
                        // and hasn't yet been visited,
                        && !cells[two_neighbor.0 as usize][two_neighbor.1 as usize].visited
                    {
                        // set the cell between them to not a wall (sneaky integer division)
                        cells[(pos.0 as isize + two_offset.0 / 2) as usize]
                            [(pos.1 as isize + two_offset.1 / 2) as usize]
                            .wall = false;
                        // and walk from said cell specified by the offset
                        walk(
                            cells,
                            (
                                (pos.0 as isize + two_offset.0) as usize,
                                (pos.1 as isize + two_offset.1) as usize,
                            ),
                            dimensions,
                        );
                    }
                };

                for two_offset in get_two_offset_shuffle() {
                    walk_to(two_offset);
                }
            }

            walk(&mut cells, starter, (dimensions.0, dimensions.1));

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
         * Generates a Maze by recursively adding walls at random positions that divide the
         * remaining accessible area.
         */
        fn gen_from_divide(mut cells: Vec<Vec<Cell>>) -> Self {
            let dimensions: (usize, usize) = (cells.len(), cells[0].len());

            let entrypoint: (usize, usize) = (1, 0);

            let goalpoint: (usize, usize) = (dimensions.0 - 2, dimensions.1 - 1);

            // set walls on top and bottom
            for i in 0..cells[0].len() {
                cells[0][i].wall = true;
                cells[dimensions.0 - 1][i].wall = true;
            }

            // set walls on left and right
            for row in &mut cells {
                row[0].wall = true;
                row[dimensions.1 - 1].wall = true;
            }

            // make sure the entrypoint and goalpoint aren't walls
            cells[entrypoint.0][entrypoint.1].wall = false;
            cells[goalpoint.0][goalpoint.1].wall = false;

            /*
             * Recursively divide the section of the Maze uniquely defined by its specified top-left
             * and bottom-right corners in half, leaving a hole in the wall to maintain access
             * between the left/right or top/bottom halves.
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
                // if the area to divide is at least as tall as it is wide,
                if height >= width {
                    // pick an even y-index to draw a wall that divides the remaining area, and draw
                    // it
                    let mut wall_index = 1;
                    while wall_index % 2 != 0 {
                        wall_index = rng.gen_range((top_left.0 + 1)..bottom_right.0);
                    }
                    for i in (top_left.1 + 1)..bottom_right.1 {
                        cells[wall_index][i].wall = true;
                    }

                    // pick an odd x-index to draw the hole on, and punch it out
                    let mut hole_index = 0;
                    while hole_index % 2 != 1 {
                        hole_index = rng.gen_range((top_left.1 + 1)..bottom_right.1);
                    }
                    cells[wall_index][hole_index].wall = false;

                    // recursively divide the remaining halves
                    divide(cells, top_left, (wall_index, bottom_right.1));
                    divide(cells, (wall_index, top_left.1), bottom_right);
                }
                // if the area to divide is wider than it is tall (same procedure as above)
                else {
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

                    divide(cells, top_left, (bottom_right.0, wall_index));
                    divide(cells, (top_left.0, wall_index), bottom_right);
                }
            }

            divide(&mut cells, (0, 0), (dimensions.0 - 1, dimensions.1 - 1));

            Maze {
                dimensions,
                goalpoint,
                entrypoint,
                cells,
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

            let two_offsets = [(-2, 0), (2, 0), (0, 2), (0, -2)];

            // compute frontier cells of a random (odd, odd) cell and add them to a list
            let mut frontiers: Vec<(usize, usize)> = vec![];
            let mut rng = thread_rng();
            let mut starter: (usize, usize) = (0, 0);
            while starter.0 % 2 == 0 || starter.1 % 2 == 0 {
                starter = (
                    rng.gen_range(1..(dimensions.0 - 1)),
                    rng.gen_range(1..(dimensions.1 - 1)),
                );
            }
            cells[starter.0][starter.1].wall = false;

            let append_frontiers = |cells: &Vec<Vec<Cell>>,
                                    frontiers: &mut Vec<(usize, usize)>,
                                    pos: (usize, usize)| {
                for &(dy, dx) in &two_offsets {
                    let two_neighbor: (isize, isize) = (pos.0 as isize + dy, pos.1 as isize + dx);

                    // as long as the two-neighbor is in bounds (and not on the edge),
                    if two_neighbor.0 > 0
                        && two_neighbor.0 < cells.len() as isize - 1
                        && two_neighbor.1 > 0
                        && two_neighbor.1 < cells[0].len() as isize - 1
                        // not a wall,
                        && cells[two_neighbor.0 as usize][two_neighbor.1 as usize].wall
                        // and not already in the list,
                        && !frontiers.contains(&(two_neighbor.0 as usize, two_neighbor.1 as usize))
                    {
                        // add it to the list of frontier cells
                        frontiers.push((two_neighbor.0 as usize, two_neighbor.1 as usize));
                    }
                }
            };

            append_frontiers(&cells, &mut frontiers, starter);

            // while the list of frontier cells is not empty:
            while frontiers.len() > 0 {
                // pick a random frontier cell from the list, and mark it as not a wall
                let rand_frontier_index: usize = rng.gen_range(0..frontiers.len());
                let current: (usize, usize) = frontiers[rand_frontier_index];
                cells[current.0][current.1].wall = false;

                // compute its two-neighbors (that aren't walls)
                let mut two_neighbors = vec![];
                for &(dy, dx) in &two_offsets {
                    let two_neighbor: (isize, isize) =
                        (current.0 as isize + dy, current.1 as isize + dx);

                    // if the two-neighbor is in the allowable area (not OOB and not on the very
                    // edge),
                    if two_neighbor.0 > 0
                        && two_neighbor.0 < dimensions.0 as isize - 1
                        && two_neighbor.1 > 0
                        && two_neighbor.1 < dimensions.1 as isize - 1
                        // and not a wall,
                        && !cells[two_neighbor.0 as usize][two_neighbor.1 as usize].wall
                    {
                        // add it to the list of two-neighbors
                        two_neighbors.push((two_neighbor.0 as usize, two_neighbor.1 as usize));
                    }
                }

                // Pick a random two-neighbor and connect the randomly chosen frontier cell with it
                // by setting the cell in-between to not a wall
                if let Some((two_neighbor_y, two_neighbor_x)) = two_neighbors.choose(&mut rng) {
                    cells[(*two_neighbor_y + current.0) / 2][(*two_neighbor_x + current.1) / 2]
                        .wall = false;
                }

                // compute the frontier cells of the randomly chosen frontier cell and add them to
                // the list, if they aren't already in the list
                append_frontiers(&cells, &mut frontiers, current);

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
                pos.0 + 1 < self.dimensions.0 && self.solve_from_backtracking((pos.0 + 1, pos.1))
            }) || ({
                // East
                pos.1 + 1 < self.dimensions.1 && self.solve_from_backtracking((pos.0, pos.1 + 1))
            }) || ({
                // North
                pos.0 as isize - 1 > 0 && self.solve_from_backtracking((pos.0 - 1, pos.1))
            }) || ({
                // West
                pos.1 as isize - 1 > 0 && self.solve_from_backtracking((pos.0, pos.1 - 1))
            }) || ({
                // This cell isn't on the path to the solution, since going each direction results
                // in a dead end
                self.cells[pos.0][pos.1].visited = false;
                false
            })
        }

        /*
         * Solves this Maze iteratively by filling in the dead-ends, which leaves only the correct
         * path.
         */
        fn solve_from_dead_end_filling(&mut self) -> bool {
            self.cells[self.entrypoint.0][self.entrypoint.1].visited = true;
            self.cells[self.goalpoint.0][self.goalpoint.1].visited = true;

            let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

            // find the dead-ends
            let mut dead_ends: Vec<(usize, usize)> = vec![];
            for i in 1..self.cells.len() - 1 {
                for j in 1..self.cells[0].len() - 1 {
                    // if the cell isn't a wall, check each neighboring cell to see if it's a dead
                    // end--if it is, it's marked unvisited and added to the list; otherwise, it's
                    // marked visited
                    if !self.cells[i][j].wall {
                        let mut neighbor_count = 0;
                        for &(dy, dx) in &directions {
                            let neighbor = ((i as isize + dy) as usize, (j as isize + dx) as usize);
                            if !self.cells[neighbor.0][neighbor.1].wall {
                                neighbor_count += 1;
                            }
                        }

                        self.cells[i][j].visited = neighbor_count != 1;

                        if !self.cells[i][j].visited {
                            dead_ends.push((i as usize, j as usize));
                        }
                    }
                }
            }
            while dead_ends.len() != 0 {
                // pick a dead end out of the list, and find the cell in the Maze that connects to
                // it
                let dead_end: (usize, usize) = dead_ends.pop().unwrap();
                let connector = directions
                    .iter()
                    .find_map(|&(dy, dx)| {
                        let neighbor = (
                            (dead_end.0 as isize + dy) as usize,
                            (dead_end.1 as isize + dx) as usize,
                        );

                        if !self.cells[neighbor.0][neighbor.1].wall
                            && self.cells[neighbor.0][neighbor.1].visited
                        {
                            Some(neighbor)
                        } else {
                            None // unreachable
                        }
                    })
                    .unwrap();

                // if there are two or more ways to go from connector (excluding dead_end), we've
                // finished closing in a dead end; otherwise, we need to add connector as a new dead
                // end
                let mut paths_out_of_connector = 0;

                for &(dy, dx) in &directions {
                    let neighbor: (usize, usize) = (
                        (connector.0 as isize + dx) as usize,
                        (connector.1 as isize + dy) as usize,
                    );

                    if !self.cells[neighbor.0][neighbor.1].wall
                        && self.cells[neighbor.0][neighbor.1].visited
                    {
                        paths_out_of_connector += 1;
                    }
                }

                // if we're still at a dead end, make the connector a new dead end
                if paths_out_of_connector == 1 {
                    self.cells[connector.0][connector.1].visited = false;
                    dead_ends.push(connector);
                }
            }
            true
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
    let mut maze: Maze = Maze::new_from((55, 55), CreationAlgorithm::Prim);
    println!("{}", maze);
    maze.solve_from(SolvingAlgorithm::DeadEndFilling);
    println!("{}", maze);
}
