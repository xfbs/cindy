use crate::{cli::ServeCommand, Cindy};
use anyhow::Result;
use axum::Server;

impl Cindy {
    pub async fn command_serve(&self, command: &ServeCommand) -> Result<()> {
        let server = Server::try_bind(&command.listen)?;
        println!("Listening on {}", command.listen);
        server.serve(self.router().into_make_service()).await?;
        Ok(())
    }
}
