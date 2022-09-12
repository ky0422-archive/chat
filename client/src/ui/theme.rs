use cursive::{
    theme::{PaletteColor::*, *},
    *,
};

pub fn theme() -> Theme {
    cursive::theme::Theme {
        borders: BorderStyle::Simple,
        palette: Palette::default().with(|palette| {
            palette[Background] = Color::Rgb(30, 30, 30);
        }),
        ..cursive::theme::Theme::default()
    }
}
