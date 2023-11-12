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
                    "Type 1 to use Prim algorithm, Type 2 to use Random Walk algorithm, Type 3 to use Recursive Division algorithm";
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("No Line Given");
                    let x: usize = input.trim().parse().expect("Please Input an Integer");
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
            println!("Please Input an Acceptable Integer.");
        }
    }
}
