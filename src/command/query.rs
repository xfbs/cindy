use crate::{cli::QueryCommand, Cindy};
use anyhow::Result;

impl Cindy {
    pub async fn command_query(&self, command: &QueryCommand) -> Result<()> {
        let database = self.database().await;
        let command = command.clone();
        tokio::task::spawn_blocking(move || {
            let hashes = database.hash_query(&mut command.filters.iter())?;
            for hash in &hashes {
                println!("{}", hex::encode(hash));
            }
            Ok(()) as Result<()>
        })
        .await??;
        Ok(())
    }
}
