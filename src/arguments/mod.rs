use argparse::{ArgumentParser, Store, StoreOption};

#[derive(Debug, Default)]
pub struct Arguments {
    pub day: u32,
    pub part: u32,
    pub src_file: Option<String>,
}

impl Arguments {
    pub fn parse_args() -> Arguments {
        let mut args = Arguments::default();

        {
            let mut parser = ArgumentParser::new();
            parser.set_description("Advent of Code 2020");

            parser
                .refer(&mut args.day)
                .add_option(
                    &["-n", "--day"],
                    Store,
                    "Number of the Advent of Code challenge/day",
                )
                .required();

            parser
                .refer(&mut args.part)
                .add_option(
                    &["-p", "--part"],
                    Store,
                    "Each problem has two parts -- specific the part with this option",
                )
                .required();

            parser.refer(&mut args.src_file).add_option(
                &["-i", "--input-file"],
                StoreOption,
                "Input file. Defaults to 'input/day<number>.input",
            );

            parser.parse_args_or_exit();
        }

        if args.src_file.is_none() {
            args.src_file = Some(format!("input/day{}.input", args.day));
        }

        args
    }
}
