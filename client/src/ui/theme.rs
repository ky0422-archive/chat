use cursive::{
    theme::{PaletteColor::*, *},
    *,
};

pub const BLACK: Color = Color::Rgb(30, 30, 30);
pub const GRAY: Color = Color::Rgb(60, 60, 60);
pub const WHITE: Color = Color::Rgb(255, 255, 255);

pub fn theme() -> Theme {
    cursive::theme::Theme {
        borders: BorderStyle::Simple,
        palette: Palette::default().with(|palette| {
            palette[Background] = BLACK;
            palette[View] = BLACK;
            palette[Primary] = WHITE;
            palette[Secondary] = GRAY;
            palette[Tertiary] = WHITE;
            palette[TitlePrimary] = WHITE;
            palette[TitleSecondary] = WHITE;
            palette[Highlight] = WHITE;
            palette[HighlightInactive] = WHITE;
            palette[HighlightText] = WHITE;
        }),
        ..cursive::theme::Theme::default()
    }
}
