pub mod greet;
pub mod hello;

pub use greet::GreetCmd;
pub use hello::world::WorldCmd;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Runnable {
    async fn run(&self) -> Result<()>;
}
