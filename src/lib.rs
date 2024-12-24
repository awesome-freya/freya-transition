pub mod curves;
pub mod segmented_animation;
pub mod state_based;
pub mod tween;
pub mod value_based;

pub use curves::Curve;
pub use segmented_animation::use_segmented_animation;
pub use state_based::use_transition as use_state_transition;
pub use tween::Tween;
pub use tween::Value;
pub use value_based::use_transition;
