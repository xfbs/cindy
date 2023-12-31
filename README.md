# Cindy

[![ci status](https://gitlab.com/xfbs/cindy/badges/main/pipeline.svg)](https://gitlab.com/xfbs/cindy/-/pipelines)
[![test coverage](https://img.shields.io/badge/coverage-nightly-blue)](https://cindy.pics/coverage/)
[![docs nightly](https://img.shields.io/badge/docs-nightly-blue)](https://cindy.pics/rustdoc/cindy/)

<div align="center">
<img src="screenshot.png" width="50%" />
</div>

Cindy is your friendly librarian for media files. Once you create a new Cindy
project, she can scan all of the files inside it and automatically tag them.
She uses `ffmpeg` to decode media files, and as such she has the ability to
read a large variety of file types.

Once files are tagged, you can use her to query them. She has a GUI mode
written using [Yew](https://yew.rs) that you can use to explore and modify
your library of files.

Currently in development is a feature to add labels to media files to be able
to quickly retrieve specific parts of them (offsets in an audio file or video
file or coordinates in an image).

## Requirements

Cindy should be able to run on any modern Linux, Windows or MacOS system,
provided that you can get it compiled.

Run-time dependencies:

- sqlite3
- ffmpeg

## Download

You can get the latest build of Cindy from here. These releases might be
somewhat unstable but always represents the latest status (whatever is on the
`main` branch).

| Platform | Link |
| --- | --- |
| Linux (AMD64) | [cindy-linux-amd64](https://xfbs.gitlab.io/cindy/cindy-linux-amd64) |

## Development

To be able to build Cindy, you need some dependencies. To install them on a
recent Debian system, you can run this:

```
apt install libsqlite3-dev libavcodec-dev clang pkg-config libavformat-dev
```

You will also need `cargo`, which you can install using [rustup](https://rustup.rs):

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you do not like to install things on your system, you can also use the
provided `Dockerfile` to get an environment set up where you can build and run
tests. To do this, for build an image using the provided `Dockerfile` and then
run a container with the repository mapped into it.

```
docker build . -t cindy-builder
docker run -it --rm -v $(pwd):/code --workdir /code --user $(id -u):$(id -g) cindy-builder
```

Finally, use `cargo` to test and build Cindy:

```
cargo test
cargo build --release
```

## License

MIT.
