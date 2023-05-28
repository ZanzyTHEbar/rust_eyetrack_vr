# EyeTrackVR Rust

This is a Rust library for interfacing with the EyeTrackVR eye tracker.

## Usage

## Utils

Setup Cargo Watch:

> This is for automatically recompiling on file change - Live Reloading

Install it:

```bash
cargo add cargo-watch
```

Trigger it:

```bash
cargo watch -q -c -w src/ -x 'run -q'
```

Add Cargo Edit:

> This is for managing dependencies
> `cargo upgrade` to upgrade all dependencies

```bash
cargo add cargo-edit
```

Generate library documentation:

> This is for generating documentation for the library

```bash
cargo doc --open
```

Generate rlib file:

> This is for generating an rlib file for use in other projects

```bash
rustc --crate-type=lib main.rs --create-name "eye_track_vr" --out-dir ./target/eyetrackvr
```
