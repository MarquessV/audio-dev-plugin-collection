# Audio Development Utility Plugins

A collection of single-purpose VST3 and CLAP plugins that are useful for audio plugin development.

## Plugins

The package for each plugin can be found in the `plugins` directory. Here's a brief overview of each.

### Tone Generator

A simple tone generator with configurable frequency, gain, and four waveforms to choose from.

## Building

After installing [Rust](https://rustup.rs/), you can build these plugins using [cargo-make](https://github.com/sagiegurari/cargo-make#env-declaration):

```shell
cargo make build-plugins
```

After a successful build, you can find the VST3 and CLAP plugins in `./target/bundled/`.
