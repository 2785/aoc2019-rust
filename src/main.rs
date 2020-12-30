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
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Day1(d) => d.run(),
        SubCommand::Day2(d) => d.run(),
        SubCommand::Day3(d) => d.run(),
    }
}
