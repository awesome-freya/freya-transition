use freya::prelude::Point2D;
use freya_core::values::{Fill, Shadow};
use skia_safe::Color;

use super::Value;

pub trait Lerp {
    #[must_use]
    fn lerp(&self, end: &Self, x: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(&self, end: &Self, x: f32) -> Self {
        self * (1.0 - x) + end * x
    }
}

impl Lerp for Point2D {
    fn lerp(&self, end: &Self, x: f32) -> Self {
        Self::new(self.x.lerp(&end.x, x), self.y.lerp(&end.y, x))
    }
}

impl Lerp for Color {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn lerp(&self, end: &Self, x: f32) -> Self {
        Self::from_argb(
            f32::from(self.a()).lerp(&f32::from(end.a()), x) as u8,
            f32::from(self.r()).lerp(&f32::from(end.r()), x) as u8,
            f32::from(self.g()).lerp(&f32::from(end.g()), x) as u8,
            f32::from(self.b()).lerp(&f32::from(end.b()), x) as u8,
        )
    }
}

impl Lerp for Shadow {
    fn lerp(&self, end: &Self, x: f32) -> Self {
        let (Fill::Color(a), Fill::Color(b)) = (&self.fill, &end.fill) else {
            unimplemented!()
        };

        Self {
            position: self.position.clone(),
            x: self.x.lerp(&end.x, x),
            y: self.y.lerp(&end.y, x),
            blur: self.blur.lerp(&end.blur, x),
            spread: self.spread.lerp(&end.spread, x),
            fill: Fill::Color(a.lerp(b, x)),
        }
    }
}

impl Lerp for Value {
    fn lerp(&self, end: &Self, x: f32) -> Self {
        match (self, end) {
            (Self::Color(start), Self::Color(end)) => Self::Color(start.lerp(end, x)),
            (Self::Number(start), Self::Number(end)) => Self::Number(start.lerp(end, x)),
            (Self::Point(start), Self::Point(end)) => Self::Point(start.lerp(end, x)),
            (Self::Shadow(start), Self::Shadow(end)) => Self::Shadow(start.lerp(end, x)),
            _ => unimplemented!("haha"),
        }
    }
}
