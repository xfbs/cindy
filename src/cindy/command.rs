use crate::{cli::Command, Cindy};
use anyhow::Result;
use std::time::Duration;
use tokio::task::block_in_place;
pub const UPDATE_INTERVAL: Duration = Duration::from_millis(30);

mod add;
mod query;
mod tags;

impl Cindy {
    pub async fn command(&self, command: &Command) -> Result<()> {
        match command {
            Command::Initialize(_) => Ok(()),
            Command::Add(command) => self.command_add(&command).await,
            Command::Query(command) => self.command_query(&command).await,
            Command::Tags(command) => self.command_tags(&command).await,
            #[cfg(feature = "gtk4")]
            Command::Ui => {
                block_in_place(|| crate::gtk::main(self.clone()));
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
