pub use self::{keyframe::KeyFrameAnimation, path::PathAnimation};
use crate::Tween;

mod keyframe;
mod path;

pub trait Animation {
    fn init(&self, tween: &mut Tween);
    fn advance(&mut self, tween: &mut Tween, index: u128);
    fn get_duration(&self) -> u64;
}
