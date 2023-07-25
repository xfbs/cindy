mod cindy;
pub mod cli;
mod command;
pub mod config;
mod database;
pub mod hash;
#[cfg(feature = "ffmpeg")]
mod media;
mod plugins;
#[cfg(feature = "server")]
mod server;
mod tag;

pub use crate::cindy::Cindy;
pub use cindy_common as common;
pub use cli::{Command, Options};
pub use config::Config;
#[cfg(feature = "ffmpeg")]
pub use media::{
    ffmpeg_init, AudioFormat, AudioInfo, ImageFormat, ImageInfo, MediaInfo, VideoFormat, VideoInfo,
};
pub use tag::{Tag, TagFilter, TagPredicate};
