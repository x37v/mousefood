//! A prelude for conveniently writing applications using this library.
//!
//! # Examples
//!
//! ```rust,no_run
//! use mousefood::prelude::*;
//! ```

pub use crate::EmbeddedBackend;
pub use embedded_graphics::pixelcolor::{
    Bgr555, Bgr565, Bgr666, Bgr888, Rgb555, Rgb565, Rgb666, Rgb888,
};
pub use ratatui::prelude::*;
