# Cask

## Quick Start

### pack your site

```bash
cask --pack ./dist --output ./release/output.bin

```

### run your site

```bash
./release/output.bin
```

## How It Works

Cask embeds your static files into a single executable.
On first run it extracts the files, starts a local HTTP server,
and opens the browser automatically. Nothing to install.

## Features

- Gzip compression for HTML, CSS and JS out of the box
- Smart cache with configurable memory limit
- Hash based change detection on restart
- Works with any framework — React, Vue, SvelteKit, plain HTML

## Installation

```bash
cargo install cask
```

## Documentation

Full documentation at [https://daviddanielng.github.io/cask](https://daviddanielng.github.io/cask)
