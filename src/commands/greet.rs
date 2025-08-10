use async_trait::async_trait;
use clap::Args;

use crate::{commands::Runnable, utils::confirm::confirm_action};
use anyhow::Result;

#[derive(Debug, Default, Args)]
pub struct GreetCmd;

#[async_trait]
impl Runnable for GreetCmd {
    async fn run(&self) -> Result<()> {
        println!(
            "Hi user! This is a demo CLI which showcases how good Rust can be for building terminal apps."
        );

        if confirm_action("Close app?") {
            println!("Bye :<")
        } else {
            println!("Nah I'll close myself anyway.")
        }

        Ok(())
    }
}
