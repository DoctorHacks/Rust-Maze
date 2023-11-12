mod maze;
use maze::maze_operations;
use std::io;

use crate::maze_operations::*;

fn main() {
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    loop {
        println!("Type 1 to Create and Solve a Maze.\n Type 2 to Quit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("No Line Given");
        let x: usize = input.trim().parse().expect("Please Input an Integer");
        if x == 1 {
            println!("Give the Dimensions for the Maze in format: rows cols");
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("No Line Given");
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
