+++
title = "Quick Start"
description = "One page summary of how to start a new Cindy project."
date = 2021-05-01T08:20:00+00:00
updated = 2021-05-01T08:20:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "One page summary of how to start a new Cindy project."
toc = true
top = false
+++

## Requirements

Before using the theme, you need to install some dependencies. Cindy uses
`ffmpeg` to analyze media files and it uses `sqlite` to store metadata. Here
is how you can install the necessary packages on [Debian](https://debian.org)
or [Ubuntu](https://ubuntu.com):

```
apt install libavformat-dev libavutil-dev sqlite3-dev
```

## Installation

To install Cindy, you have two options. You can download a nightly release,
which is the easiest way to get started. You can also install it from source,
which may be required if you want to use it on a platform that does not have a
release build automatically.

### From Release

Go to the [Releases](/releases) page and download a release for your platform.
You may need to unpack it or mark it as executable, depending on your operating
system.

### From Source

Clone the repository and build it from source. You need to build the frontend
separately.

```
git clone https://gitlab.com/xfbs/cindy
cd cindy/ui
trunk build --release
cd ..
cargo build --release
```

### Using Docker

You can also build Cindy using Docker, which should be easier if you do not have
the build dependencies installed on your host system.

More on this soon.

## Setup

### Create Project

You do not need any configuration to use Cindy. All you need to do is
initialize a new Cindy project, which you can do like this. Cindy will create a
folder `myproject` for you.

```
cindy init myproject
```

You can also initialize an already existing folder as a Cindy project.

```
cindy init .
```

### Add Files

Next, you can add files (they will be scanned and indexed):

```
cindy add -r .
```

You can also add individual files, like this:

```
cindy add podcast.mp3 video.avi
```

### Launch Interface

Finally, you can start the server and explore the interface:

```
cindy serve
```

After you launched this command, you should be able to connect to Cindy at
<http://localhost:8000>.
