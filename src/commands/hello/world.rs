use async_trait::async_trait;
use clap::Args;

use crate::{commands::Runnable, utils::confirm::confirm_action};
use anyhow::Result;

#[derive(Debug, Default, Args)]
pub struct WorldCmd;

#[async_trait]
impl Runnable for WorldCmd {
    async fn run(&self) -> Result<()> {
        loop {
            println!("Hello World!");

            if !confirm_action("Greet again?") {
                break;
            }
        }

        Ok(())
    }
}
