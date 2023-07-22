pub mod cindy;
pub mod cli;
pub mod config;
mod database;
#[cfg(feature = "gtk4")]
pub mod gtk;
pub mod hash;
#[cfg(feature = "ffmpeg")]
mod media;
mod plugins;
mod tag;

pub use crate::cindy::Cindy;
pub use cli::{Command, Options};
pub use config::Config;
#[cfg(feature = "ffmpeg")]
pub use media::{
    ffmpeg_init, AudioFormat, AudioInfo, ImageFormat, ImageInfo, MediaInfo, VideoFormat, VideoInfo,
};
pub use tag::{Tag, TagFilter, TagPredicate};
