#![feature(gen_blocks, array_windows, let_chains)]

use std::{env, time::Instant};

mod day_1;
mod day_2;
mod day_3;

const USAGE: &str = "Usage: aoc-2024 <day>";

fn main() {
    let arg = env::args().nth(1).expect(USAGE);
    if arg == "--help" || arg == "-h" { return eprintln!("{USAGE}"); }

    let day: u8 = arg.parse().expect("Invalid day");

    let task = match day {
        1 => day_1::main,
        2 => day_2::main,
        3 => day_3::main,
        _ => unimplemented!("day {day} challenge"),
    };

    eprintln!("# Day {day}\n");
    let start = Instant::now();
    task();
    let elapsed = start.elapsed();

    eprint!("\n# Completed in ");
    match elapsed.as_millis() {
           0..10   => eprintln!("{}us", elapsed.as_micros()),
          10..1000 => eprintln!("{}ms", elapsed.as_millis()),
        1000..     => eprintln!("{:.2}s", elapsed.as_secs_f32()),
    }
}
