# notion
[![Build](https://github.com/fundament-software/notion/actions/workflows/build.yml/badge.svg)](https://github.com/fundament-software/notion/actions/workflows/build.yml)

Internal Fundament Software fork of the Notion API client library for rust, which incorporates several pending PRs and additional fixes from the original repo. To use this fork in your project, add the following to `cargo.toml`:

```toml
notion = { git = "https://github.com/Fundament-Software/notion" }
```

## Docs

The (outdated) generated documentation site is available here: https://docs.rs/notion/

## Building

```bash
cargo build
```

## Testing

This fork may be unstable and may break randomly. Fundament Software makes no attempt to support configurations we don't need.