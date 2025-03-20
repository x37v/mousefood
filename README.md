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

```rust
let backend = mousefood::EmbeddedBackend::new(&mut display, None, None);
let mut terminal = ratatui::Terminal::new(backend)?;

loop {
    terminal.draw(...)?;
}
```
