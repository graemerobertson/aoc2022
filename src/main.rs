extern crate lazy_static;
use structopt::StructOpt;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

#[derive(StructOpt)]
struct Cli {
    day: u16,
}

fn main() {
    let args = Cli::from_args();
    match args.day {
        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        4 => day04::day04(),
        5 => day05::day05(),
        6 => day06::day06(),
        411 => {
            day01::day01();
            day02::day02();
            day03::day03();
            day04::day04();
            day05::day05();
            day06::day06();
        }
        _ => println!("Unimplemented day: {}", args.day),
    }
}
