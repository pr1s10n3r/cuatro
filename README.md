# Cuatro

_The last 4chan downloader tool you will ever need._

## Build

```shell
$ cargo build --release
```

## Usage

```shell
# Basic usage:
$ cuatro https://boards.4chan.org/po/thread/570368

# Exclude file extensions
$ cuatro https://boards.4chan.org/po/thread/570368 --exclude webm,png,gif

# Set how many threads for downloading
$ cuatro https://boards.4chan.org/po/thread/570368 --threads 4

# Help
$ cuatro --help
```
