use ariadne::{Color, ReportKind};

pub trait ToColor {
    fn color(&self) -> Color;
}

impl ToColor for ReportKind {
    fn color(&self) -> Color {
        match self {
            ReportKind::Error => Color::Red,
            ReportKind::Warning => Color::Yellow,
            ReportKind::Advice => Color::White,
            ReportKind::Custom(_, color) => color.clone(),
        }
    }
}
