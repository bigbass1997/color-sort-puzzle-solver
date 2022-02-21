
use clap::{AppSettings, Arg, Command};
use crate::puzzle::Puzzle;


pub mod puzzle;
pub mod recognition;
pub mod util;

fn main() {
    let matches = Command::new("Tube Color-sort Puzzle Solver")
        .arg(Arg::new("screenshot")
            .takes_value(true)
            .required(true))
        .global_setting(AppSettings::DeriveDisplayOrder)
        .arg_required_else_help(true)
        .next_line_help(true)
        .get_matches();
    
    let puzzle = Puzzle::new(matches.value_of("screenshot").unwrap());
    
    
}
