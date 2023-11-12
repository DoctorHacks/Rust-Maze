mod lib;
use lib::maze_operations;
use std::io;

fn main(){
    let usize mut rows = 0;
    let usize mut cols = 0;
    let mut input = String::new();
    loop{
        println!("Type 1 to Create and Solve a Maze.\n Type 2 to Quit");
        io::stdin().readline(&mut input).expect("No Line Given");
        let x: usize = input.trim().parse().expect("Please Input an Integer");
        if x == 1{
            println!("Give the Dimensions for the Maze in format: rows cols");
            loop{
                io::stdin().readline(&mut input).expect("No Line Given");
                let mut nums = input.trim().split_whitespace();
                let rows = numbers.next().expect("No Next").parse().expect("Not a Valid Integer");
                let cols = numbers.next().expect("No Next").parse().expect("Not a Valid Integer");

                let Maze
            }
            
            
        } else if x == 2 break;
        else println("Please Input an Acceptable Integer.");
    }
}