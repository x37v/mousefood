use crate::macros::for_all_colors;
use embedded_graphics::pixelcolor::{
    Bgr555, Bgr565, Bgr666, Bgr888, Rgb555, Rgb565, Rgb666, Rgb888, RgbColor,
};
use ratatui::style::Color;

pub enum TermColorType {
    Foreground,
    Background,
}

pub struct TermColor(pub Color, pub TermColorType);

macro_rules! impl_from_term_color {
    (
        $color_type:ident
    ) => {
        impl From<TermColor> for $color_type {
            fn from(color: TermColor) -> Self {
                const LIGHT_RED: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R, Rgb888::MAX_G / 2, Rgb888::MAX_B / 2);
                const LIGHT_GREEN: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G, Rgb888::MAX_B / 2);
                const LIGHT_YELLOW: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R, Rgb888::MAX_G, Rgb888::MAX_B / 2);
                const LIGHT_BLUE: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G / 2, Rgb888::MAX_B);
                const LIGHT_MAGENTA: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R, Rgb888::MAX_G / 2, Rgb888::MAX_B);
                const LIGHT_CYAN: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G, Rgb888::MAX_B);
                const GRAY: Rgb888 =
                    Rgb888::new(Rgb888::MAX_R / 2, Rgb888::MAX_G / 2, Rgb888::MAX_B / 2);
                const DARK_GRAY: Rgb888 = Rgb888::new(
                    (2.0 / 3.0 * (Rgb888::MAX_R as f32)) as u8,
                    (2.0 / 3.0 * (Rgb888::MAX_G as f32)) as u8,
                    (2.0 / 3.0 * (Rgb888::MAX_B as f32)) as u8,
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

                    Color::LightRed => LIGHT_RED.into(),
                    Color::LightGreen => LIGHT_GREEN.into(),
                    Color::LightYellow => LIGHT_YELLOW.into(),
                    Color::LightBlue => LIGHT_BLUE.into(),
                    Color::LightMagenta => LIGHT_MAGENTA.into(),
                    Color::LightCyan => LIGHT_CYAN.into(),
                    Color::Gray => GRAY.into(),
                    Color::DarkGray => DARK_GRAY.into(),

                    Color::Rgb(r, g, b) => Rgb888::new(r, g, b).into(),
                    Color::Indexed(_) => todo!("Color::Indexed not implemented yet!"),
                }
            }
        }
    };
}

for_all_colors!(impl_from_term_color);
