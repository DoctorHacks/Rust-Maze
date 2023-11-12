/*
 * The main method for the program.
 * Asks the user if they want to create and solve a maze.
 * Then, asks the user what form of generation they want to use for the maze.
 * Finally, it prints out the solved maze using both solving algorithms and
 * prints the amount of time it took to solve each one.
 * This can be repeated as many times as the user requests, until they quit the program.
 * 
 */

mod maze;
use maze::maze_operations;
use std::io;
use std::time::Instant;

use crate::maze_operations::*;
fn main() {
    let mut rows;
    let mut cols;
    let mut maze;
    let mut input = String::new();
    loop {
        println!("Type 1 to create and solve a maze.\nType 2 to quit.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("No line given.");
        let x: usize = input.trim().parse().expect("Please input an integer.");
        if x == 1 {
            println!("Give the dimensions for the maze in format: rows cols.");

            //Asks the user for the dimensions of the maze
            //Both rows and cols must be 3 or greater, or it will
            //loop around and ask again until 2 good arguments are given.
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("No line given.");
                let mut nums = input.trim().split_whitespace();
                rows = nums
                    .next()
                    .expect("No Next.")
                    .parse()
                    .expect("Not a Valid Integer.");
                cols = nums
                    .next()
                    .expect("No Next")
                    .parse()
                    .expect("Not a Valid Integer.");

                if cols >= 3 && rows >= 3 {
                    break;
                } else {
                    println!("Rows and cols must be 3 or greater.");
                }
            }
            //Loop for user to input how they want their maze to be generated
            //1:Prim Algorithm 2:Random Walk Algorithm 3:Recursive Division Algorithm
            println!(
                "Choose which algorithm to use to generate maze\n
                        Type 1 to use prim algorithm.\n
                        Type 2 to use random walk algorithm.\n
                        Type 3 to use recursive division algorithm."
            );
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("No line given.");
                let x: usize = input.trim().parse().expect("Please input an integer.");
                match x {
                    1 => {
                        maze = Maze::new_from((rows, cols), CreationAlgorithm::Prim);
                        break;
                    }
                    2 => {
                        maze = Maze::new_from((rows, cols), CreationAlgorithm::RandomWalk);
                        break;
                    }
                    3 => {
                        maze = Maze::new_from((rows, cols), CreationAlgorithm::RecursiveDivision);
                        break;
                    }
                    _ => println!("Please input an acceptable integer."),
                }
            }
            println!("{} {}", rows, cols);
            println!("{}", maze);

            //Recursive Backtracking Solver
            println!("Press enter to show the solved maze using recursive backtracking");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            let timer = Instant::now();
            maze.solve_from(SolvingAlgorithm::RecursiveBacktracking);
            println!("It took {:?} Seconds", timer.elapsed());
            println!("{}", maze);

            //Dead End Filling Solver
            println!("Press enter to show the solved maze using dead end filling");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            let timer = Instant::now();
            maze.solve_from(SolvingAlgorithm::RecursiveBacktracking);
            println!("It took {:?} Seconds", timer.elapsed());
            println!("{}", maze);
            
        } else if x == 2 {
            break;
        } else {
            println!("Please input an acceptable integer.");
        }
    }
}
