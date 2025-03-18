use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::{MonoFont, MonoTextStyle};
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle, StyledDrawable};
use embedded_graphics::text::Text;
use ratatui::backend::{Backend, WindowSize};
use ratatui::buffer::Cell;
use ratatui::layout::{Position, Size};
use ratatui::prelude::Color;

enum TermColorType {
    Foreground,
    Background,
}

struct TermColor(Color, TermColorType);

pub struct EmbeddedBackend<'display, D: 'display, C>
where
    D: DrawTarget<Color = C>,
{
    display: &'display mut D,
    font: MonoFont<'static>,

    bg_offset: Point,
    fg_offset: Point,

    columns_rows: Size,
    pixels: Size,
}

impl<'display, D, C> EmbeddedBackend<'display, D, C>
where
    D: DrawTarget<Color = C> + Dimensions,
{
    pub fn new(
        display: &'display mut D,
        font: MonoFont<'static>,
    ) -> EmbeddedBackend<'display, D, C> {
        let pixels = Size {
            width: display.bounding_box().size.width as u16,
            height: display.bounding_box().size.height as u16,
        };
        Self {
            display,
            font,
            bg_offset: Point::new(0, (font.character_size.height - font.baseline) as i32),
            fg_offset: Point::new(0, font.character_size.height as i32),
            columns_rows: Size {
                height: pixels.height / font.character_size.height as u16,
                width: pixels.width / font.character_size.width as u16,
            },
            pixels,
        }
    }
}

macro_rules! impl_for_color {
    (
        $color_type:ident
    ) => {
        impl From<TermColor> for $color_type {
            fn from(color: TermColor) -> Self {
                const R_RATIO: f32 = $color_type::MAX_R as f32 / 256.0;
                const G_RATIO: f32 = $color_type::MAX_G as f32 / 256.0;
                const B_RATIO: f32 = $color_type::MAX_B as f32 / 256.0;

                const LIGHT_RED: $color_type = $color_type::new(
                    $color_type::MAX_R,
                    $color_type::MAX_G / 2,
                    $color_type::MAX_B / 2,
                );
                const LIGHT_GREEN: $color_type = $color_type::new(
                    $color_type::MAX_R / 2,
                    $color_type::MAX_G,
                    $color_type::MAX_B / 2,
                );
                const LIGHT_YELLOW: $color_type = $color_type::new(
                    $color_type::MAX_R,
                    $color_type::MAX_G,
                    $color_type::MAX_B / 2,
                );
                const LIGHT_BLUE: $color_type = $color_type::new(
                    $color_type::MAX_R / 2,
                    $color_type::MAX_G / 2,
                    $color_type::MAX_B,
                );
                const LIGHT_MAGENTA: $color_type = $color_type::new(
                    $color_type::MAX_R,
                    $color_type::MAX_G / 2,
                    $color_type::MAX_B,
                );
                const LIGHT_CYAN: $color_type = $color_type::new(
                    $color_type::MAX_R / 2,
                    $color_type::MAX_G,
                    $color_type::MAX_B,
                );
                const GRAY: $color_type = $color_type::new(
                    $color_type::MAX_R / 2,
                    $color_type::MAX_G / 2,
                    $color_type::MAX_B / 2,
                );
                const DARK_GRAY: $color_type = $color_type::new(
                    (2.0 / 3.0 * ($color_type::MAX_R as f32)) as u8,
                    (2.0 / 3.0 * ($color_type::MAX_G as f32)) as u8,
                    (2.0 / 3.0 * ($color_type::MAX_B as f32)) as u8,
                );

                match color.0 {
                    Color::Reset => match color.1 {
                        TermColorType::Foreground => $color_type::WHITE,
                        TermColorType::Background => $color_type::BLACK,
                    },
                    Color::White => $color_type::WHITE,
                    Color::Black => $color_type::BLACK,
                    Color::Red => $color_type::RED,
                    Color::Green => $color_type::GREEN,
                    Color::Yellow => $color_type::YELLOW,
                    Color::Blue => $color_type::BLUE,
                    Color::Magenta => $color_type::MAGENTA,
                    Color::Cyan => $color_type::CYAN,

                    Color::LightRed => LIGHT_RED,
                    Color::LightGreen => LIGHT_GREEN,
                    Color::LightYellow => LIGHT_YELLOW,
                    Color::LightBlue => LIGHT_BLUE,
                    Color::LightMagenta => LIGHT_MAGENTA,
                    Color::LightCyan => LIGHT_CYAN,
                    Color::Gray => GRAY,
                    Color::DarkGray => DARK_GRAY,

                    Color::Rgb(r, g, b) => $color_type::new(
                        (r as f32 * R_RATIO) as u8,
                        (g as f32 * G_RATIO) as u8,
                        (b as f32 * B_RATIO) as u8,
                    ),
                    Color::Indexed(_) => todo!("Color::Indexed not implemented yet!"),
                }
            }
        }

        impl<D> Backend for EmbeddedBackend<'_, D, $color_type>
        where
            D: DrawTarget<Color = $color_type>,
        {
            fn draw<'a, I>(&mut self, content: I) -> std::io::Result<()>
            where
                I: Iterator<Item = (u16, u16, &'a Cell)>,
            {
                for (x, y, cell) in content {
                    let position = Point::new(
                        x as i32 * self.font.character_size.width as i32,
                        y as i32 * self.font.character_size.height as i32,
                    );

                    // Background
                    let bg: $color_type = TermColor(cell.bg, TermColorType::Background).into();
                    Rectangle::new(position + self.bg_offset, self.font.character_size)
                        .draw_styled(&PrimitiveStyle::with_fill(bg), self.display)
                        .ok();

                    // Foreground
                    let fg: $color_type = TermColor(cell.fg, TermColorType::Foreground).into();
                    let style = MonoTextStyle::new(&self.font, fg);
                    Text::new(cell.symbol(), position + self.fg_offset, style)
                        .draw(self.display)
                        .ok();
                }
                Ok(())
            }

            fn hide_cursor(&mut self) -> std::io::Result<()> {
                // TODO
                Ok(())
            }

            fn show_cursor(&mut self) -> std::io::Result<()> {
                // TODO
                Ok(())
            }

            fn get_cursor_position(&mut self) -> std::io::Result<Position> {
                // TODO
                Ok(Position::new(0, 0))
            }

            fn set_cursor_position<P: Into<Position>>(
                &mut self,
                #[allow(unused_variables)] position: P,
            ) -> std::io::Result<()> {
                // TODO
                Ok(())
            }

            fn clear(&mut self) -> std::io::Result<()> {
                self.display.clear($color_type::BLACK).ok();
                Ok(())
            }

            fn size(&self) -> std::io::Result<Size> {
                Ok(self.columns_rows)
            }

            fn window_size(&mut self) -> std::io::Result<WindowSize> {
                Ok(WindowSize {
                    columns_rows: self.columns_rows,
                    pixels: self.pixels,
                })
            }

            fn flush(&mut self) -> std::io::Result<()> {
                // buffer is flushed after each character draw
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

pub use embedded_graphics;
pub use ratatui;

#[cfg(test)]
mod tests {
    // TODO: tests
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
