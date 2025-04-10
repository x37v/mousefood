# ![Mousefood](https://github.com/j-g00da/mousefood/blob/599f1026d37c8d6308a6df64a234dbefaedc0c6f/assets/logo/mousefood.svg?raw=true)

[![Crate](https://img.shields.io/crates/v/mousefood?logo=rust&style=flat-square&color=ebe94f)](https://crates.io/crates/mousefood)
[![Docs](https://img.shields.io/docsrs/mousefood?logo=rust&style=flat-square)](https://docs.rs/mousefood)
[![CI](https://img.shields.io/github/actions/workflow/status/j-g00da/mousefood/ci.yml?style=flat-square&logo=github)](https://github.com/j-g00da/mousefood/blob/main/.github/workflows/ci.yml)
[![Deps](https://deps.rs/crate/mousefood/latest/status.svg?style=flat-square)](https://deps.rs/crate/mousefood)

**Mousefood** - [embedded-graphics](https://crates.io/crates/embedded-graphics) backend
for [Ratatui](https://crates.io/crates/ratatui)!

> [!IMPORTANT]  
> Currently works only with `std`-enabled targets,
> such as Espressif's ESP32 MCU series.
> Support for "bare-metal" (`no_std`) targets is planned,
> but this would require upstream changes - discussed [here](https://github.com/ratatui/ratatui/discussions/1746).

<div align="center">

![demo](https://github.com/j-g00da/mousefood/blob/599f1026d37c8d6308a6df64a234dbefaedc0c6f/assets/demo.jpg?raw=true)
![animated demo](https://github.com/j-g00da/mousefood/blob/599f1026d37c8d6308a6df64a234dbefaedc0c6f/assets/demo.gif?raw=true)

</div>

## Quickstart

Add mousefood as a dependency:

```shell
cargo add mousefood
```

Exemplary setup:

```rust
use mousefood::prelude::*;

fn main() -> Result<(), std::io::Error> {
    // Any embedded_graphics DrawTarget
    let mut display = MyDrawTarget::new();
    
    let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(...)?;
    }
}
```

### Special characters

Embedded-graphics includes bitmap fonts that have a very limited
set of characters to save space (ASCII, ISO 8859 or JIS X0201).
This makes it impossible to draw most of Ratatui's widgets,
which heavily use box-drawing glyphs, Braille,
and other special characters.

Mousefood by default uses [`embedded-graphics-unicodefonts`](https://crates.io/crates/embedded-graphics-unicodefonts),
which provides embedded-graphics fonts with a much larger set of characters.

#### Alternatives

In order to save space and [speed up rendering](#performance-and-hardware-support),
the `fonts` feature can be disabled by turning off the default crate features.
[`ibm437`](https://crates.io/crates/ibm437) is a good alternative that includes
some drawing characters, but is not as large as embedded-graphics-unicodefonts.

### Simulator

Mousefood can be run in a simulator
(requires [SDL2](https://wiki.libsdl.org/SDL2/Installation) to be installed).
The simulator mode can be enabled using the `simulator` feature and utilizes the
[embedded-graphics-simulator](https://crates.io/crates/embedded-graphics-simulator)
crate.

Run simulator example:

```shell
git clone https://github.com/j-g00da/mousefood.git
cd mousefood
cargo run --example=simulator --features=simulator
```

Exemplary setup using simulator:

```rust
use mousefood::prelude::*;
use mousefood::embedded_graphics::geometry;
use mousefood::simulator::SimulatorDisplay;

fn main() -> Result<(), std::io::Error> {
    let mut display = SimulatorDisplay::<Bgr565>::new(geometry::Size::new(128, 64));
    let backend: EmbeddedBackend<SimulatorDisplay<_>, _>
        = EmbeddedBackend::new(&mut display);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(...)?;
    }
}
```

## Performance and hardware support

Flash memory on most embedded devices is very limited. Additionally,
to achieve high frame rate when using the `fonts` feature,
it is recommended to use `opt-level = 3`,
which can make the resulting binary even larger.

Mousefood is hardware-agnostic, but requires a `std`-enabled target.
Successfully tested on:

- esp32 (base model, 4MB flash)
- esp32c6 (16MB flash)

## Docs

Full API docs are available on [docs.rs](https://docs.rs/mousefood).

## License

[![License MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square&color=8d97b3)](LICENSE-MIT)
[![License Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat-square&color=8d97b3)](LICENSE-APACHE)

Mousefood is dual-licensed under
[Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) terms.
