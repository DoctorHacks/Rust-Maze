/*
 * Asks the user if they'd like to create and solve a maze. If they do, they're prompted for the
 * dimensions of their maze, and which maze generation algorithm they'd like to employ. Then, their
 * maze is printed with the solution computed via recursive backtracking and dead-end filling, along
 * with the amount of time it took to compute each solution.
 * This can be repeated as many times as the user requests, until they quit the program.
 *
 * Author: Brandon Ikeler, Travis Hahn
 */

mod maze;
use maze::maze_operations;
use std::io;
use std::time::Instant;

use crate::maze_operations::*;
fn main() {
    let mut maze;

    // Prompt the user whether they'd like to continue. If so, ask what dimensions they'd like it to
    // be and which algorithm should be used to generate it.
    loop {
        let mut input = String::new();
        let mut continue_choice = 0;

        // Get user's choice--do they want to keep generating mazes, or are they done?
        while {
            input.clear();
            println!("Enter 1 to create and solve a maze.\nEnter 2 to quit.");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            // User pressed enter without typing anything
            input.trim().is_empty() && {
                println!("No input detected."); // side-effects are allowed in expressions!
                true
            } || {
                match input.trim().parse::<i32>() {
                    // User correctly input a value of 1 or 2
                    Ok(parsed) if 1 <= parsed && parsed <= 2 => {
                        continue_choice = parsed;
                        false
                    }
                    // User input an integer, but not a 1 or a 2
                    Ok(_) => {
                        println!("Please enter an acceptable integer.");
                        true
                    }
                    // User didn't input an integer
                    Err(_) => {
                        println!("Expected an integer.");
                        true
                    }
                }
            }
        } { /* this is technically the loop body */ }

        match continue_choice {
            // User is done making mazes. :(
            2 => {
                break;
            }
            // User wants to generate a maze!
            1 => {
                let mut input = String::new();
                let mut rows = 0;
                let mut cols = 0;

                // What dimensions do they want the maze to be?
                while {
                    input.clear();
                    println!("Enter the dimensions for the maze in format: rows cols.");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line.");
                    let mut nums = input.trim().split_whitespace();
                    ({
                        match nums.next() {
                            Some(next) => match next.parse::<usize>() {
                                // User correctly input an integer for rows
                                Ok(parsed_rows) => {
                                    rows = parsed_rows;
                                    false
                                }
                                // User input something, but it wasn't an integer
                                Err(_) => {
                                    println!("Failed to parse rows.");
                                    true
                                }
                            },
                            // User didn't input anything for rows
                            None => {
                                println!("Failed to read rows.");
                                true
                            }
                        }
                    }) || ({
                        match nums.next() {
                            Some(next) => match next.parse::<usize>() {
                                // User correctly input an integer for cols
                                Ok(parsed_cols) => {
                                    cols = parsed_cols;
                                    false
                                }
                                // User input something, but it wasn't an integer
                                Err(_) => {
                                    println!("Failed to parse cols.");
                                    true
                                }
                            },
                            // User didn't input anything for cols
                            None => {
                                println!("Failed to read cols.");
                                true
                            }
                        }
                    }) || {
                        // User input values for rows and cols, but at least one of them was less
                        // than 3--and mazes smaller than 3x3 don't make sense.
                        let unacceptable_size = cols < 3 || rows < 3;
                        unacceptable_size && {
                            println!("Rows and cols must be 3 or greater.");
                            true
                        }
                    }
                } {}

                let mut input = String::new();
                let mut algorithm_choice = 0;

                // Which maze generation algorithm would they like to employ?
                while {
                    input.clear();
                    println!(concat!(
                        "Choose which algorithm to use to generate the maze:\n",
                        "Enter 1 to use Prim's algorithm.\n",
                        "Enter 2 to perform a random walk.\n",
                        "Enter 3 to recursively divide."
                    ));
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    // User pressed enter without typing anything
                    input.trim().is_empty() && {
                        println!("No input detected.");
                        true
                    } || {
                        match input.trim().parse::<i32>() {
                            // User correctly input a value from 1 to 3
                            Ok(parsed) if 1 <= parsed && parsed <= 3 => {
                                algorithm_choice = parsed;
                                false
                            }
                            // User input an integer, but it wasn't from 1 to 3
                            Ok(_) => {
                                println!("Please enter an acceptable integer.");
                                true
                            }
                            // User typed something other than an integer
                            Err(_) => {
                                println!("Expected an integer.");
                                true
                            }
                        }
                    }
                } {}

                match algorithm_choice {
                    1 => {
                        maze = Maze::new_from((rows, cols), CreationAlgorithm::Prim);
                    }
                    2 => {
                        maze = Maze::new_from((rows, cols), CreationAlgorithm::RandomWalk);
                    }
                    3 => {
                        maze = Maze::new_from((rows, cols), CreationAlgorithm::RecursiveDivision);
                    }
                    _ => {
                        maze = Maze::new((rows, cols)); // unreachable
                    }
                }
                println!("{}", maze);

                // Time solving via recursive backtracking
                println!("Solving via recursive backtracking (press enter to continue).");
                let mut input = String::new();
                let _ = io::stdin().read_line(&mut input);

                let timer = Instant::now();
                maze.solve_from(SolvingAlgorithm::RecursiveBacktracking);
                let duration = timer.elapsed().as_micros();

                println!("{}", maze);
                println!(
                    "It took {:?} microseconds to solve via recursive backtracking.",
                    duration
                );

                maze.unsolve();

                // Time solving via dead-end filling
                println!("Solving via dead-end filling (press enter to continue).");
                let mut input = String::new();
                let _ = io::stdin().read_line(&mut input);

                let timer = Instant::now();
                maze.solve_from(SolvingAlgorithm::DeadEndFilling);
                let duration = timer.elapsed().as_micros();

                println!("{}", maze);
                println!(
                    "It took {:?} microseconds to solve via dead-end filling.",
                    duration
                );

                let mut input = String::new();
                println!("Press enter to continue.");
                let _ = io::stdin().read_line(&mut input);
            }
            // the only possible values of continue_choice by the point the match statement is
            // reached are 1 and 2, so this can't ever execute.
            _ => {
                panic!("Unexpected error while processing decision to continue");
            }
        }
    }
}
