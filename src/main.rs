mod lib;
use lib::maze_operations;
use std::io;

fn main(){
    let usize mut rows = 0;
    let usize mut cols = 0;
    
    loop{
        println!("Type 1 to Create and Solve a Maze.\n Type 2 to Quit");
        let mut input = String::new();
        io::stdin().readline(&mut input).expect("No Line Given");
        let x: usize = input.trim().parse().expect("Please Input an Integer");
        if x == 1{
            //Maze Dimension Input
            println!("Give the Dimensions for the Maze in format: rows cols");
            loop{
                let mut input = String::new();
                io::stdin().readline(&mut input).expect("No Line Given");
                let mut nums = input.trim().split_whitespace();
                let rows = numbers.next().expect("No Next").parse().expect("Not a Valid Integer");
                let cols = numbers.next().expect("No Next").parse().expect("Not a Valid Integer");
                
                if rows>=3 && cols>=3 {
                    break;
                } else {println!("Rows and Cols Must be 3 or Greater")}
            }
            
            //Maze Generation Algorithm Used Input
            println!("Type 1 for Prim Generation\n
                        Type 2 for Random Walk Generation\n
                        Type 3 for Recursive Division Generation");
            loop{
                let mut input = String::new();
                io::stdin().readline(&mut input).expect("No Line Given");
                let x: usize = input.trim().parse().expect("Please Input an Integer");
                
                match x{
                    1 => println!("1")
                    2 => println!("2")
                    3 => println!("3")
                    _ => println!("Please Input a Valid Answer")
                }
            }
            
        } else if x == 2 break;
        else println("Please Input an Acceptable Integer.");
    }
}