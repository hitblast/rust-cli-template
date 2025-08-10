pub mod greet;
pub mod hello;

// command use for easy access throughout the entire codebase
pub use greet::GreetCmd;
pub use hello::world::WorldCmd;

use anyhow::Result;
use async_trait::async_trait;

/// Trait for all runnable commands.
#[async_trait]
pub trait Runnable {
    async fn run(&self) -> Result<()>;
}
