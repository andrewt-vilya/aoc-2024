#![feature(gen_blocks, array_windows, let_chains)]

use std::{env, fmt, time::Instant};

mod util;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

const USAGE: &str = "Usage: aoc-2024 <day>";

struct HumanTime(std::time::Duration);

impl fmt::Display for HumanTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elapsed = self.0;
        match elapsed.as_millis() {
               0..10   => write!(f, "{}us", elapsed.as_micros()),
              10..1000 => write!(f, "{}ms", elapsed.as_millis()),
            1000..     => write!(f, "{:.2}s", elapsed.as_secs_f32()),
        }
    }
}

fn run_timed(day: u8, task: impl FnOnce()) {
    eprintln!("\n# Day {}", day);
    let start = Instant::now();
    task();
    let elapsed = HumanTime(start.elapsed());

    eprintln!("Completed in {elapsed}");
}

fn main() {
    let arg = env::args().nth(1).expect(USAGE);
    if arg == "--help" || arg == "-h" { return eprintln!("{USAGE}"); }

    if arg == "all" {
        let tasks = [
            day_1::main,
            day_2::main,
            day_3::main,
            day_4::main,
            day_5::main,
        ];

        let start = Instant::now();
        for (i, task) in tasks.into_iter().enumerate() {
            run_timed(i as u8 + 1, task);
        }
        let elapsed = HumanTime(start.elapsed());
        eprintln!("\nTotal time: {elapsed}");
    } else {
        let day: u8 = arg.parse().expect("Invalid day");

        let task = match day {
            1 => day_1::main,
            2 => day_2::main,
            3 => day_3::main,
            4 => day_4::main,
            5 => day_5::main,
            _ => unimplemented!("day {day} challenge"),
        };

        run_timed(day, task);
    }
}
