use anyhow::Result;
use clap::Clap;

use crate::protocol::GitProtocol;

#[derive(Clap, Debug)]
pub struct CloneCmd {
    url: String,
}

pub async fn clone_cmd(cmd: CloneCmd) -> Result<()> {
    println!("Hello, world! {:?}", cmd);

    let protocol = GitProtocol::new(cmd.url);
    let x = protocol.refs().await?;
    println!("x = {}", x);
    Ok(())
}
