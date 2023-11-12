mod maze;
use maze::maze_operations;
use std::io;

use crate::maze_operations::*;
fn main() {
    let mut input = String::new();
    loop {
        println!("Type 1 to create and solve a maze.\nType 2 to quit.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("No line given.");
        let x: usize = input.trim().parse().expect("Please input an integer.");
        if x == 1 {
            println!("Give the dimensions for the maze in format: rows cols.");
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("No line given.");
                let mut nums = input.trim().split_whitespace();
                let rows: usize = nums
                    .next()
                    .expect("No Next")
                    .parse()
                    .expect("Not a Valid Integer");
                let cols: usize = nums
                    .next()
                    .expect("No Next")
                    .parse()
                    .expect("Not a Valid Integer");

                ////////
                loop {
                    println!("Type 1 to use prim algorithm.\n Type 2 to use random walk algorithm.\n Type 3 to use recursive division algorithm.");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("No line given");
                    let x: usize = input.trim().parse().expect("Please input an integer");
                    match x{
                        1 => {
                            let maze: Maze = Maze::new_from((rows,cols), CreationAlgorithm::Prim);
                            break;
                        }
                        2 => {
                            let maze: Maze = Maze::new_from((rows,cols), CreationAlgorithm::RandomWalk);
                            break;
                        }
                        3 => {
                            let maze: Maze = Maze::new_from((rows,cols), CreationAlgorithm::RecursiveDivision);
                            break;
                        }
                        _ => println!("Please input an acceptable integer")
                    }
                }
                ////////
                println!("{} {}", rows, cols);
                let maze: Maze = Maze::new_from((rows, cols), CreationAlgorithm::Prim);
                println!("{}", maze);
            }
        } else if x == 2 {
            break;
        } else {
            // h
            println!("Please input an acceptable integer");
        }
    }
}
