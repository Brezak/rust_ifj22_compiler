use std::ops::{Deref, DerefMut, Range};

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
            ReportKind::Custom(_, color) => *color,
        }
    }
}

pub type Span = Range<usize>;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct WithSpan<T>(pub Span, pub T);

// For ergonomics
impl<T> Deref for WithSpan<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<T> DerefMut for WithSpan<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}
