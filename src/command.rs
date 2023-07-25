use crate::{cli::Command, Cindy};
use anyhow::Result;
use std::time::Duration;
pub const UPDATE_INTERVAL: Duration = Duration::from_millis(30);

mod add;
mod query;
mod serve;
mod tags;

impl Cindy {
    // TODO: use global options (for thread count)
    pub async fn command(&self, command: &Command) -> Result<()> {
        match command {
            Command::Init(_) => Ok(()),
            Command::Add(command) => self.command_add(&command).await,
            Command::Query(command) => self.command_query(&command).await,
            Command::Tags(command) => self.command_tags(&command).await,
            Command::Serve(command) => self.command_serve(&command).await,
            _ => Ok(()),
        }
    }
}
