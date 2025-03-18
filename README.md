# Mousefood

[embedded-graphics](https://crates.io/crates/embedded-graphics) backend for [Ratatui](https://crates.io/crates/ratatui)!

🚧 **Under construction** 🏗️

## Demo

![demo.jpg](demo.jpg)

## Installation

```shell
cargo add mousefood
```

## Usage

It is recommended to use `ibm437` font crate as it contains much more special characters
than `embedded-graphics`'s built-in fonts.
It is required to properly render widgets like charts and borders.

```rust
let backend = mousefood::EmbeddedBackend::new(&mut display, ibm437::IBM437_8X8_REGULAR);
let mut terminal = ratatui::Terminal::new(backend)?;

loop {
    terminal.draw(...)?;
}
```
