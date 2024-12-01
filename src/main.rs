use clap::Parser;

use day1::run_day1;

mod day1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The day to run; if 0 is given, all days are run.
    #[arg(short, long, default_value_t = 0)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    match args.day {
        0 => run_all(),
        1 => run_day1(),
        _ => todo!("Day {} is not upon us yet!", args.day),
    }
}

fn run_all() {
    run_day1();
}
