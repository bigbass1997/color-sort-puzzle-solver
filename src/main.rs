
use std::time::Instant;
use clap::{AppSettings, Arg, Command};
use crate::puzzle::PuzzleState;


pub mod puzzle;
pub mod recognition;


fn main() {
    let matches = Command::new("Tube Color-sort Puzzle Solver")
        .arg(Arg::new("screenshot")
            .takes_value(true)
            .required(true))
        .global_setting(AppSettings::DeriveDisplayOrder)
        .arg_required_else_help(true)
        .next_line_help(true)
        .get_matches();
    
    let start = Instant::now();
    let puzzle = recognition::parse_image(matches.value_of("screenshot").unwrap());
    let elapsed = start.elapsed();
    println!("Parsed in: {:.4}s", elapsed.as_secs_f64());
    
    let start = Instant::now();
    let node = puzzle.solve().unwrap();
    let elapsed = start.elapsed();
    for transfer in node.transfers {
        println!("{:?}", transfer);
    }
    
    println!("Solved in: {:.2}s", elapsed.as_secs_f64());
}
