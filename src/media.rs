use crate::Tag;
use anyhow::{anyhow, Result};
use chrono::NaiveTime;
use ffmpeg_next::{
    self as ffmpeg,
    codec::context::Context,
    format::{context::Input, input},
    util::log::{set_level, Level},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::Path};
use strum::Display;

pub fn ffmpeg_init() -> Result<()> {
    ffmpeg::init()?;
    set_level(Level::Quiet);
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "media")]
pub enum MediaInfo {
    Image(ImageInfo),
    Video(VideoInfo),
    Audio(AudioInfo),
}

impl From<ImageInfo> for MediaInfo {
    fn from(info: ImageInfo) -> Self {
        MediaInfo::Image(info)
    }
}

impl From<VideoInfo> for MediaInfo {
    fn from(info: VideoInfo) -> Self {
        MediaInfo::Video(info)
    }
}

impl From<AudioInfo> for MediaInfo {
    fn from(info: AudioInfo) -> Self {
        MediaInfo::Audio(info)
    }
}

fn resolution(width: u64, height: u64) -> &'static str {
    let min: u64 = width.min(height);
    match min {
        min if min >= 4320 => "8k",
        min if min >= 2160 => "4k",
        min if min >= 1440 => "2k",
        min if min >= 1080 => "fullhd",
        min if min >= 720 => "hd",
        min if min >= 480 => "sd",
        _ => "low",
    }
}

fn durationgroup(duration: u64) -> &'static str {
    match duration {
        duration if duration <= 60 => "short",
        duration if duration <= (3 * 60) => "mediumshort",
        duration if duration <= (10 * 60) => "medium",
        duration if duration <= (30 * 60) => "mediumlong",
        duration if duration <= (60 * 60) => "long",
        _ => "extended",
    }
}

impl MediaInfo {
    pub fn tags(&self) -> BTreeSet<Tag> {
        match self {
            MediaInfo::Image(media) => media.tags(),
            MediaInfo::Video(media) => media.tags(),
            MediaInfo::Audio(media) => media.tags(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageInfo {
    format: ImageFormat,
    width: u64,
    height: u64,
}

impl ImageInfo {
    fn tags(&self) -> BTreeSet<Tag> {
        [
            Tag::new("media".into(), "image".into()),
            Tag::new("width".into(), self.width.to_string()),
            Tag::new("height".into(), self.height.to_string()),
            Tag::new(
                "resolution".into(),
                resolution(self.width, self.height).into(),
            ),
            self.format.tag(),
        ]
        .into()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VideoInfo {
    format: VideoFormat,
    width: u64,
    height: u64,
    duration: u64,
}

impl VideoInfo {
    fn tags(&self) -> BTreeSet<Tag> {
        [
            Tag::new("media".into(), "video".into()),
            Tag::new("width".into(), self.width.to_string()),
            Tag::new("height".into(), self.height.to_string()),
            Tag::new("duration".into(), self.duration.to_string()),
            Tag::new("durationgroup".into(), durationgroup(self.duration).into()),
            Tag::new(
                "resolution".into(),
                resolution(self.width, self.height).into(),
            ),
            self.format.tag(),
        ]
        .into()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioInfo {
    format: AudioFormat,
    duration: u64,
}

impl AudioInfo {
    fn tags(&self) -> BTreeSet<Tag> {
        [
            Tag::new("media".into(), "audio".into()),
            Tag::new("duration".into(), self.duration.to_string()),
            Tag::new("durationgroup".into(), durationgroup(self.duration).into()),
            self.format.tag(),
        ]
        .into()
    }
}

#[derive(Display, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ImageFormat {
    Jpg,
    Png,
    Webp,
}

#[derive(Display, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VideoFormat {
    Gif,
    Mp4,
    Avi,
    Mpeg,
    Ts,
    Mkv,
    Mov,
    Ogg,
    Wmv,
}

#[derive(Display, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AudioFormat {
    Mp3,
    M4a,
}

trait FormatTag: std::fmt::Display {
    fn tag(&self) -> Tag {
        Tag::new("format".into(), self.to_string())
    }
}

impl FormatTag for ImageFormat {}
impl FormatTag for AudioFormat {}
impl FormatTag for VideoFormat {}

pub fn media_info(path: &Path) -> Result<MediaInfo> {
    let file = input(&path)?;
    match file.format().name() {
        "image2" | "jpeg_pipe" | "webp_pipe" | "png_pipe" => {
            image_info(&file).map(MediaInfo::Image)
        }
        "asf"
        | "ogg"
        | "mpeg"
        | "matroska,webm"
        | "avi"
        | "mov,mp4,m4a,3gp,3g2,mj2"
        | "gif"
        | "mpegts" => video_info(&file).map(MediaInfo::Video),
        format => Err(anyhow!("Unknown format {format}")),
    }
}

fn video_info(input: &Input) -> Result<VideoInfo> {
    let format = match input.format().name() {
        "gif" => VideoFormat::Gif,
        "mpegts" => VideoFormat::Ts,
        "mpeg" => VideoFormat::Mpeg,
        "mov,mp4,m4a,3gp,3g2,mj2" => VideoFormat::Mov,
        "avi" => VideoFormat::Avi,
        "matroska,webm" => VideoFormat::Mkv,
        "asf" => VideoFormat::Wmv,
        "ogg" => VideoFormat::Ogg,
        format => return Err(anyhow!("Unknown format {format}")),
    };

    let mut info = VideoInfo {
        format,
        width: 0,
        height: 0,
        duration: 0,
    };

    for stream in input.streams() {
        let codec = Context::from_parameters(stream.parameters())?;
        if codec.medium() == ffmpeg::media::Type::Video {
            if stream.duration() < 0 {
                for (name, value) in &stream.metadata() {
                    if name == "DURATION" {
                        if let Ok(time) = NaiveTime::parse_from_str(value, "%H:%M:%S%.f") {
                            info.duration =
                                time.signed_duration_since(NaiveTime::MIN).num_seconds() as u64;
                        }
                    }
                }
            } else {
                info.duration =
                    (stream.duration() as f64 * f64::from(stream.time_base())).ceil() as u64;
            }
            if let Ok(video) = codec.decoder().video() {
                info.width = video.width() as u64;
                info.height = video.height() as u64;
            }
        }
    }

    Ok(info)
}

fn image_info(input: &Input) -> Result<ImageInfo> {
    let format = match input.format().name() {
        "image2" => ImageFormat::Jpg,
        "jpeg_pipe" => ImageFormat::Jpg,
        "webp_pipe" => ImageFormat::Webp,
        "png_pipe" => ImageFormat::Png,
        format => return Err(anyhow!("Unknown format {format}")),
    };

    let mut info = ImageInfo {
        format,
        width: 0,
        height: 0,
    };

    for stream in input.streams() {
        let codec = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
        if codec.medium() == ffmpeg::media::Type::Video {
            if let Ok(video) = codec.decoder().video() {
                info.width = video.width() as u64;
                info.height = video.height() as u64;
            }
        }
    }

    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs::read_to_string;

    #[derive(Deserialize)]
    struct Samples {
        sample: Vec<Sample>,
    }

    #[derive(Deserialize)]
    struct Sample {
        file: String,
        #[serde(flatten)]
        info: MediaInfo,
    }

    #[test]
    fn test_resolution() {
        assert_eq!(resolution(480, 240), "low");
        assert_eq!(resolution(240, 480), "low");

        assert_eq!(resolution(650, 480), "sd");
        assert_eq!(resolution(480, 640), "sd");

        assert_eq!(resolution(1024, 720), "hd");
        assert_eq!(resolution(720, 1024), "hd");

        assert_eq!(resolution(1920, 1080), "fullhd");
        assert_eq!(resolution(1080, 1920), "fullhd");

        assert_eq!(resolution(2560, 1440), "2k");
        assert_eq!(resolution(1440, 2560), "2k");

        assert_eq!(resolution(3840, 2160), "4k");
        assert_eq!(resolution(2160, 3840), "4k");

        assert_eq!(resolution(7680, 4320), "8k");
        assert_eq!(resolution(4320, 7680), "8k");
    }

    #[test]
    fn image_info_tags() {
        let info: MediaInfo = ImageInfo {
            format: ImageFormat::Jpg,
            width: 1920,
            height: 1080,
        }
        .into();
        let tags = info.tags();
        assert!(tags.contains(&Tag::new("media".into(), "image".into())));
        assert!(tags.contains(&Tag::new("format".into(), "jpg".into())));
        assert!(tags.contains(&Tag::new("width".into(), "1920".into())));
        assert!(tags.contains(&Tag::new("height".into(), "1080".into())));
        assert!(tags.contains(&Tag::new("resolution".into(), "fullhd".into())));
    }

    #[test]
    fn video_info_tags() {
        let info: MediaInfo = VideoInfo {
            format: VideoFormat::Gif,
            width: 1920,
            height: 1080,
            duration: 60,
        }
        .into();
        let tags = info.tags();
        assert!(tags.contains(&Tag::new("media".into(), "video".into())));
        assert!(tags.contains(&Tag::new("format".into(), "gif".into())));
        assert!(tags.contains(&Tag::new("width".into(), "1920".into())));
        assert!(tags.contains(&Tag::new("height".into(), "1080".into())));
        assert!(tags.contains(&Tag::new("duration".into(), "60".into())));
        assert!(tags.contains(&Tag::new("resolution".into(), "fullhd".into())));
        assert!(tags.contains(&Tag::new("durationgroup".into(), "short".into())));
    }

    #[test]
    fn audio_info_tags() {
        let info: MediaInfo = AudioInfo {
            format: AudioFormat::Mp3,
            duration: 60,
        }
        .into();
        let tags = info.tags();
        assert!(tags.contains(&Tag::new("media".into(), "audio".into())));
        assert!(tags.contains(&Tag::new("format".into(), "mp3".into())));
        assert!(tags.contains(&Tag::new("duration".into(), "60".into())));
    }

    #[test]
    fn media_info_samples() {
        let samples: Samples =
            toml::from_str(&read_to_string("samples/samples.toml").unwrap()).unwrap();
        for sample in &samples.sample {
            let path = Path::new("samples").join(&sample.file);
            assert_eq!(
                &media_info(&path).unwrap(),
                &sample.info,
                "{path:?} media info"
            );
            let _clone = sample.info.clone();
            let _debug = format!("{:?}", sample.info);
        }
    }
}
