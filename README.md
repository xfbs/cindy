# Cindy

Cindy is your friendly librarian for media files. Once you create a new Cindy
project, she can scan all of the files inside it and automatically tag them.
She uses `ffmpeg` to decode media files, and as such she has the ability to
read a large variety of file types.

Once files are tagged, you can use her to query them. She has a GUI mode
written using GTK4 that you can use to explore your library of files.

Currently in development is a feature to add labels to media files to be able
to quickly retrieve specific parts of them (offsets in an audio file or video
file or coordinates in an image).

## Requirements

Cindy should be able to run on any modern Linux, Windows or MacOS system,
provided that you can get it compiled.

Run-time dependencies:

- sqlite3
- ffmpeg
- gtk4

## Development

To be able to build Cindy, you need some dependencies. To install them on a
recent Debian system, you can run this:

```
apt install libsqlite3-dev gtk-4-dev libavcodec-dev libav
```

You will also need `cargo`, which you can install using rustup.

Use `cargo` to test and build Cindy.

```
cargo test
cargo build --release
```

## License

MIT.
