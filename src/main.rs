use clap::Clap;
use tokio;

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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Clone(cmd) => clone_cmd(cmd),
    }
    .await
    .unwrap();
}
