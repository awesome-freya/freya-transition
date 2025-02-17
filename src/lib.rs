mod animations;
pub mod curves;
mod segmented_animation;
mod trait_based;
mod transition;
mod tween;

pub use self::{
    animations::{Animation, KeyFrameAnimation, PathAnimation},
    curves::Curve,
    segmented_animation::use_segmented_animation,
    trait_based::use_trait_animation,
    transition::use_transition,
    tween::{Gradient, Lerp, Tween, Value},
};
