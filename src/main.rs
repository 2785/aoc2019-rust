mod days;
mod intcode;

use clap::Clap;

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Day1(days::Day1),
    Day2(days::Day2),
    Day3(days::Day3),
    Day4(days::Day4),
    Day5(days::Day5),
    Day6(days::Day6),
    Day7(days::Day7),
    Day8(days::Day8),
    Day9(days::Day9),
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Day1(d) => d.run(),
        SubCommand::Day2(d) => d.run(),
        SubCommand::Day3(d) => d.run(),
        SubCommand::Day4(d) => d.run(),
        SubCommand::Day5(d) => d.run(),
        SubCommand::Day6(d) => d.run(),
        SubCommand::Day7(d) => d.run(),
        SubCommand::Day8(d) => d.run(),
        SubCommand::Day9(d) => d.run(),
    }
}
