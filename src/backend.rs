use std::io;
use std::marker::PhantomData;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{self, Dimensions};
use embedded_graphics::mono_font::{MonoFont, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{
    Bgr555, Bgr565, Bgr666, Bgr888, Rgb555, Rgb565, Rgb666, Rgb888,
};
use embedded_graphics::pixelcolor::{PixelColor, RgbColor};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
#[cfg(feature = "simulator")]
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use ratatui::backend::Backend;
use ratatui::layout;
use ratatui::style;

use crate::colors::*;
use crate::default_font;
use crate::error::DrawError;
use crate::framebuffer;

/// Embedded backend for Ratatui
///
/// # Examples
///
/// ```rust,no_run
/// let backend = EmbeddedBackend::new(&mut display);
/// let mut terminal = Terminal::new(backend).unwrap();
/// ```
pub struct EmbeddedBackend<'display, D, C>
where
    D: DrawTarget<Color = C> + 'display,
    C: PixelColor + 'display,
{
    #[cfg(not(feature = "simulator"))]
    display: &'display mut D,
    #[cfg(feature = "simulator")]
    display: &'display mut SimulatorDisplay<C>,
    display_type: PhantomData<D>,

    buffer: framebuffer::HeapBuffer<C>,

    font_regular: MonoFont<'static>,
    font_bold: MonoFont<'static>,

    char_offset: geometry::Point,

    columns_rows: layout::Size,
    pixels: layout::Size,

    #[cfg(feature = "simulator")]
    simulator_window: Window,
}

impl<'display, D, C> EmbeddedBackend<'display, D, C>
where
    D: DrawTarget<Color = C> + Dimensions,
    C: RgbColor + Into<Rgb888> + From<Rgb888>,
{
    fn init(
        #[cfg(not(feature = "simulator"))] display: &'display mut D,
        #[cfg(feature = "simulator")] display: &'display mut SimulatorDisplay<C>,
        font_regular: MonoFont<'static>,
        font_bold: MonoFont<'static>,
    ) -> EmbeddedBackend<'display, D, C> {
        let pixels = layout::Size {
            width: display.bounding_box().size.width as u16,
            height: display.bounding_box().size.height as u16,
        };
        Self {
            buffer: framebuffer::HeapBuffer::new(display.bounding_box()),
            display,
            display_type: PhantomData,
            font_regular,
            font_bold,
            char_offset: geometry::Point::new(0, font_regular.character_size.height as i32),
            columns_rows: layout::Size {
                height: pixels.height / font_regular.character_size.height as u16,
                width: pixels.width / font_regular.character_size.width as u16,
            },
            pixels,
            #[cfg(feature = "simulator")]
            simulator_window: Window::new(
                "mousefood emulator",
                &OutputSettings {
                    scale: 4,
                    max_fps: 30,
                    ..Default::default()
                },
            ),
        }
    }

    /// Creates a new `EmbeddedBackend` using default fonts.
    pub fn new(
        #[cfg(not(feature = "simulator"))] display: &'display mut D,
        #[cfg(feature = "simulator")] display: &'display mut SimulatorDisplay<C>,
    ) -> EmbeddedBackend<'display, D, C> {
        Self::with_font(display, None, None)
    }

    /// Creates a new `EmbeddedBackend` using `font_regular` and `font_bold`.
    pub fn with_font(
        #[cfg(not(feature = "simulator"))] display: &'display mut D,
        #[cfg(feature = "simulator")] display: &'display mut SimulatorDisplay<C>,
        font_regular: Option<MonoFont<'static>>,
        font_bold: Option<MonoFont<'static>>,
    ) -> EmbeddedBackend<'display, D, C> {
        let font_regular = font_regular.unwrap_or(default_font::regular);
        let font_bold = font_bold.unwrap_or(default_font::bold);
        Self::init(display, font_regular, font_bold)
    }

    #[cfg(feature = "simulator")]
    fn update_simulation(&mut self) -> io::Result<()> {
        self.simulator_window.update(self.display);
        if self
            .simulator_window
            .events()
            .any(|e| e == SimulatorEvent::Quit)
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "simulator window closed",
            ));
        }
        Ok(())
    }
}

macro_rules! impl_for_color {
    (
        $color_type:ident
    ) => {
        impl<D> Backend for EmbeddedBackend<'_, D, $color_type>
        where
            D: DrawTarget<Color = $color_type>,
        {
            fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
            where
                I: Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>,
            {
                for (x, y, cell) in content {
                    let position = geometry::Point::new(
                        x as i32 * self.font_regular.character_size.width as i32,
                        y as i32 * self.font_regular.character_size.height as i32,
                    );
                    let mut style_builder = MonoTextStyleBuilder::new()
                        .font(
                            if cell.style().add_modifier.contains(style::Modifier::BOLD) {
                                &self.font_bold
                            } else {
                                &self.font_regular
                            },
                        )
                        .text_color(TermColor(cell.fg, TermColorType::Foreground).into())
                        .background_color(TermColor(cell.bg, TermColorType::Background).into());

                    if cell
                        .style()
                        .add_modifier
                        .contains(style::Modifier::UNDERLINED)
                    {
                        style_builder = style_builder.underline();
                    }
                    if cell
                        .style()
                        .add_modifier
                        .contains(style::Modifier::CROSSED_OUT)
                    {
                        style_builder = style_builder.strikethrough();
                    }
                    let style = style_builder.build();

                    Text::new(cell.symbol(), position + self.char_offset, style)
                        .draw(&mut self.buffer)
                        .map_err(|_| io::Error::new(io::ErrorKind::Other, DrawError))?;
                }
                Ok(())
            }

            fn hide_cursor(&mut self) -> io::Result<()> {
                // TODO
                Ok(())
            }

            fn show_cursor(&mut self) -> io::Result<()> {
                // TODO
                Ok(())
            }

            fn get_cursor_position(&mut self) -> io::Result<layout::Position> {
                // TODO
                Ok(layout::Position::new(0, 0))
            }

            fn set_cursor_position<P: Into<layout::Position>>(
                &mut self,
                #[allow(unused_variables)] position: P,
            ) -> io::Result<()> {
                // TODO
                Ok(())
            }

            fn clear(&mut self) -> io::Result<()> {
                self.buffer
                    .clear($color_type::BLACK)
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, DrawError))
            }

            fn size(&self) -> io::Result<layout::Size> {
                Ok(self.columns_rows)
            }

            fn window_size(&mut self) -> io::Result<ratatui::backend::WindowSize> {
                Ok(ratatui::backend::WindowSize {
                    columns_rows: self.columns_rows,
                    pixels: self.pixels,
                })
            }

            fn flush(&mut self) -> io::Result<()> {
                self.display
                    .fill_contiguous(&self.display.bounding_box(), self.buffer.data.clone())
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, DrawError))?;
                #[cfg(feature = "simulator")]
                self.update_simulation()?;
                Ok(())
            }
        }
    };
}

impl_for_color!(Rgb555);
impl_for_color!(Bgr555);
impl_for_color!(Rgb565);
impl_for_color!(Bgr565);
impl_for_color!(Rgb666);
impl_for_color!(Bgr666);
impl_for_color!(Rgb888);
impl_for_color!(Bgr888);
