mod animations;
pub mod curves;
mod trait_based;
mod segmented_animation;
mod tween;
mod transition;

pub use self::{
    animations::{Animation, KeyFrameAnimation, PathAnimation},
    curves::Curve,
    trait_based::use_trait_animation,
    segmented_animation::use_segmented_animation,
    tween::{Gradient, Lerp, Tween, Value},
    transition::use_transition,
};
