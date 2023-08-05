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

pub use crate::{
    cindy::Cindy,
    cli::{Command, Options},
    config::Config,
    database::Database,
};
pub use cindy_common::{
    self as common,
    tag::{self, Tag, TagFilter, TagPredicate},
};
#[cfg(feature = "ffmpeg")]
pub use media::{
    ffmpeg_init, AudioFormat, AudioInfo, ImageFormat, ImageInfo, MediaInfo, VideoFormat, VideoInfo,
};
