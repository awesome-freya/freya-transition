use freya::prelude::Point2D;
use freya_node_state::Parse;
use material_colors::color::{Argb, Rgb};
use skia_safe::Color;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Color(Color),
    Number(f32),
    Gradient(String),
    Point(Point2D),
}

#[derive(Default)]
enum GradientType {
    #[default]
    Linear,
    Radial,
    Conic,
}

#[derive(Default)]
pub struct Gradient {
    ty: GradientType,
    stops: Vec<(f32, Color)>,
}

impl Gradient {
    pub fn linear() -> Self {
        Self::default()
    }

    pub fn radial() -> Self {
        Self {
            ty: GradientType::Radial,
            ..Default::default()
        }
    }

    pub fn conic() -> Self {
        Self {
            ty: GradientType::Conic,
            ..Default::default()
        }
    }

    pub fn stop(mut self, at: f32, color: impl Into<Value>) -> Self {
        let Value::Color(color) = color.into() else {
            panic!("expected valid color")
        };

        self.stops.push((at, color));

        self
    }

    pub fn build(self) -> Value {
        Value::Gradient(format!(
            "{}-gradient({})",
            match self.ty {
                GradientType::Linear => "linear",
                GradientType::Radial => "radial",
                GradientType::Conic => "conic",
            },
            self.stops
                .into_iter()
                .map(|(at, color)| format!(
                    "rgb({}, {}, {}, {}) {}%",
                    color.r(),
                    color.g(),
                    color.b(),
                    color.a(),
                    at * 100.0
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        match value {
            Value::Color(color) => format!(
                "rgb({}, {}, {}, {})",
                color.r(),
                color.g(),
                color.b(),
                color.a()
            ),
            Value::Gradient(gradient) => gradient,
            _ => unimplemented!(),
        }
    }
}

impl From<Value> for Color {
    fn from(value: Value) -> Self {
        match value {
            Value::Color(color) => color,
            _ => unimplemented!(),
        }
    }
}

impl From<Value> for f32 {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(number) => number,
            _ => unimplemented!(),
        }
    }
}

impl From<Value> for (f32, f32) {
    fn from(value: Value) -> Self {
        match value {
            Value::Point(point) => point.to_tuple(),
            _ => unimplemented!(),
        }
    }
}

impl From<Value> for Point2D {
    fn from(value: Value) -> Self {
        match value {
            Value::Point(point) => point,
            _ => unimplemented!(),
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        if value.starts_with("linear") {
            Self::Gradient(value.to_string())
        } else {
            Self::Color(Color::parse(value).unwrap())
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<String> for Value {
    fn from(value: String) -> Self {
        if value.starts_with("linear") {
            Self::Gradient(value)
        } else {
            Self::Color(Color::parse(value.as_str()).unwrap())
        }
    }
}

impl From<Color> for Value {
    fn from(value: Color) -> Self {
        Self::Color(value)
    }
}

impl From<Point2D> for Value {
    fn from(value: Point2D) -> Self {
        Self::Point(value)
    }
}

impl From<(f32, f32)> for Value {
    fn from((x, y): (f32, f32)) -> Self {
        Self::Point(Point2D::new(x, y))
    }
}

impl From<Argb> for Value {
    fn from(value: Argb) -> Self {
        Self::Color(Color::from_argb(
            value.alpha,
            value.red,
            value.green,
            value.blue,
        ))
    }
}

impl From<Rgb> for Value {
    fn from(value: Rgb) -> Self {
        Self::Color(Color::from_rgb(value.red, value.green, value.blue))
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Number(value)
    }
}
