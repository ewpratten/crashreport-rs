<img src="https://github.com/Ewpratten/crashreport-rs/raw/master/crashreport_screenshot.png" style="margin:auto">

# Automatic Crash Reporting for Rust

[![Crates.io](https://img.shields.io/crates/v/crashreport)](https://crates.io/crates/crashreport) 
[![Docs.rs](https://docs.rs/crashreport/badge.svg)](https://docs.rs/crashreport) 
[![Build](https://github.com/Ewpratten/crashreport-rs/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/crashreport-rs/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/crashreport-rs/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/crashreport-rs/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/crashreport-rs/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/crashreport-rs/actions/workflows/audit.yml)


`crashreport` is a *set & forget* crate that appends a button to your application's panic messages allowing users to quickly report basic diagnostic information back to you.

## Usage

Firstly, this crate relies on you setting the [`repository`](https://doc.rust-lang.org/cargo/reference/manifest.html#the-repository-field) key in your `Cargo.toml` file. When using `crashreport`, your `Cargo.toml` could end up looking something like this:

```toml
[package]
name = "my-awesome-crate"
version = "0.1.0"
repository = "https://github.com/ewpratten/my-awesome-crate"

[dependencies]
crashreport = "^1.0.0"

# If you never want terminal colors, import the crate like this instead:
crashreport = { version = "^1.0.0", default-features = false }
```

On the code side, simply use our `crashreport!` macro to add a button to your panic messages globally:

```rust
#[macro_use]
extern crate crashreport;

pub fn main() {
    // The important bit :)
    crashreport!();

    // ... do stuff

    // Your panics are now a little fancier!
    panic!("This is a panic!");
}
```

## A bit about the internals

`crashreport` works by *appending* a new function to the end of whatever your existing panic handler is. Don't worry, you can keep your fancy custom handlers if you want. Just initialize them *before* calling `crashreport!`.

### Git provider resolution

As of now, we support the following hosted Git services:

- GitHub

If you happen to be using a self-hosted version of any of these services, `crashreport` will *not* pick it up automatically. Instead, enable one of the following features to force override provider resolution:

- `assume_github`

### Terminal URLs

Whenever possible, `crashreport` will try to make clickable buttons in your terminal. The work of deciding weather the terminal supports this is carried out by the [`supports-hyperlinks`](https://github.com/zkat/supports-hyperlinks) crate. If issues arise, please open an issue over there to add support. 

Worst case, you can force-disable button creation (falling back on printing out a URL) by setting `FORCE_HYPERLINK=0` in your environment.