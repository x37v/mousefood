use embedded_graphics::pixelcolor::{
    Bgr555, Bgr565, Bgr666, Bgr888, Rgb555, Rgb565, Rgb666, Rgb888, RgbColor,
};
use ratatui::style::Color;

pub enum TermColorType {
    Foreground,
    Background,
}

pub struct TermColor(pub Color, pub TermColorType);

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
