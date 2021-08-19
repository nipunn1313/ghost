use clap::Clap;
use futures::executor::block_on;
use reqwest;

#[derive(Clap, Debug)]
pub struct CloneCmd {
    url: String,
}

pub fn clone_cmd(cmd: CloneCmd) {
    println!("Hello, world! {:?}", cmd);
    let x = block_on(reqwest::get(cmd.url));
    println!("{:?}", x)
}
