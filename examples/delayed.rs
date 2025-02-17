use freya::prelude::*;
use freya_motion::{use_transition, Curve};

fn main() {
    launch(app);
}

fn app() -> Element {
    let animation = use_transition(|context| {
        context.add_tween_delayed("offset", 0.0, Curve::EASE_IN_OUT_CIRC, 500, 500);
        context.add_tween_delayed("opacity", 1.0, Curve::FAST_OUT_SLOW_IN, 1000, 1000);
    });

    let [offset, opacity] = [
        animation.get::<f32>("offset"),
        animation.get::<f32>("opacity"),
    ];

    use_hook(move || {
        animation.play([("offset", 256.0), ("opacity", 0.0)]);
    });

    rsx! {
        rect {
            background: "red",
            width: "128",
            height: "128",
            corner_radius: "12",
            opacity: "{opacity}",
            margin: "0 0 0 {offset}",
        }
    }
}
