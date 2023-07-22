use anyhow::Result;
use cindy::{Cindy, Options};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "ffmpeg")]
    cindy::ffmpeg_init()?;

    let options = Options::parse();
    let cindy = Cindy::new(&options).await?;
    cindy.command(&options.command).await
}
