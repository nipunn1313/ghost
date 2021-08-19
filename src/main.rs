use clap::Clap;

use ghost::clone::{clone_cmd, CloneCmd};

#[derive(Clap, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    Clone(CloneCmd),
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Clone(cmd) => clone_cmd(cmd),
    }
}
