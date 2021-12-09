use advent_of_code_2021::arguments::Arguments;
use advent_of_code_2021::day1::day1;
use advent_of_code_2021::day2::day2;
use advent_of_code_2021::day3::day3;
use advent_of_code_2021::day4::day4;
use advent_of_code_2021::day5::day5;
use advent_of_code_2021::day6::day6;
use advent_of_code_2021::day7::day7;
use advent_of_code_2021::day8::day8;
use advent_of_code_2021::day9::day9;
use anyhow::Result;
use dotenv::dotenv;
use env_logger::Builder;
use log::LevelFilter;
#[macro_use]
extern crate log;

fn main() -> Result<()> {
    dotenv().ok();

    // Initialize the env_logger crate
    Builder::from_default_env()
        .filter_level(LevelFilter::Debug)
        .init();

    let args = Arguments::parse_args();

    info!("args: {:?}", args);

    match args.day {
        1 => day1(&args)?,
        2 => day2(&args)?,
        3 => day3(&args)?,
        4 => day4(&args)?,
        5 => day5(&args)?,
        6 => day6(&args)?,
        7 => day7(&args)?,
        8 => day8(&args)?,
        9 => day9(&args)?,
        _ => {
            return Err(anyhow::Error::msg(format!(
                "Unrecognized day number: {}",
                args.day
            )))
        }
    }

    Ok(())
}
