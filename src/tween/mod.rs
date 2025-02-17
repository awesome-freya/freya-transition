pub use self::{
    lerp::Lerp,
    value::{Gradient, Value},
};
use super::{curves::ICurve, Curve};

pub mod lerp;
pub mod value;

pub struct Tween {
    pub origin: Value,
    pub destination: Value,
    pub value: Value,
    pub duration: f32,
    pub delay: f32,
    pub curve: Curve,
}

impl Tween {
    #[must_use]
    pub fn new(origin: Value, destination: Value) -> Self {
        Self {
            origin: origin.clone(),
            destination,
            value: origin,
            duration: 0.0,
            delay: 0.0,
            curve: Curve::LINEAR,
        }
    }

    pub fn set_duration(&mut self, millis: u64) {
        self.duration = millis as f32;
    }

    pub fn set_curve(&mut self, curve: Curve) {
        self.curve = curve;
    }

    pub fn set_delay(&mut self, millis: u64) {
        self.delay = millis as f32;
    }

    pub fn set(&mut self, value: Value) {
        self.origin = value.clone();
        self.value = value;
    }

    pub fn to(&mut self, value: Value) {
        self.origin = self.value.clone();
        self.destination = value;
    }

    #[must_use]
    pub const fn curve(mut self, curve: Curve) -> Self {
        self.curve = curve;

        self
    }

    #[must_use]
    pub const fn duration(mut self, millis: u64) -> Self {
        self.duration = millis as f32;

        self
    }

    #[must_use]
    pub const fn delay(mut self, millis: u64) -> Self {
        self.delay = millis as f32;

        self
    }

    #[must_use]
    pub const fn is_done(&self, time: u128) -> bool {
        time >= (self.delay + self.duration) as u128
    }

    pub fn advance(&mut self, time: f32) {
        if time > self.delay {
            if matches!(self.curve, Curve::None) {
                self.value = self.destination.clone();
            } else {
                self.value = self.origin.lerp(
                    &self.destination,
                    self.curve
                        .transform((time - self.delay).min(self.duration) / self.duration),
                );
            }
        }
    }
}
