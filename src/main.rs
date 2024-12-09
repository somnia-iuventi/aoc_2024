use clap::Parser;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: usize,

    #[arg(short, long)]
    part: u8,
}

fn main() {
    let cli = Args::parse();
    match &cli.day {
        1 => day1::run(cli.part),
        2 => day2::run(cli.part),
        3 => day3::run(cli.part),
        4 => day4::run(cli.part),
        5 => day5::run(cli.part),
        6 => day6::run(cli.part),
        // 7 => {}
        // 8 => {}
        // 9 => {}
        // 10 => {}
        // 11 => {}
        // 12 => {}
        // 13 => {}
        // 14 => {}
        // 15 => {}
        // 16 => {}
        // 17 => {}
        // 18 => {}
        // 19 => {}
        // 20 => {}
        // 21 => {}
        // 22 => {}
        // 23 => {}
        // 24 => {}
        // 25 => {}
        _ => return,
    };
}
