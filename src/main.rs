use advent_of_code_2021::arguments::Arguments;
use advent_of_code_2021::day1::day1;
use anyhow::Result;
use dotenv::dotenv;
use env_logger::Builder;
use log::LevelFilter;

fn main() -> Result<()> {
    dotenv().ok();

    // Initialize the env_logger crate
    Builder::from_default_env()
        .filter_level(LevelFilter::Debug)
        .init();

    let args = Arguments::parse_args();

    match args.day {
        1 => day1(&args)?,
        _ => {
            return Err(anyhow::Error::msg(format!(
                "Unrecognized day number: {}",
                args.day
            )))
        }
    }

    Ok(())
}
