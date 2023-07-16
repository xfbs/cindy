use anyhow::Result;
use cindy::{ffmpeg_init, Cindy, Options};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    ffmpeg_init()?;
    let options = Options::parse();
    let cindy = Cindy::new(&options).await?;
    cindy.command(&options.command).await
}
