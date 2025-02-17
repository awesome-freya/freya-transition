use freya::prelude::*;
use freya_motion::{use_trait_animation, Curve, PathAnimation};

fn main() {
    launch(app);
}

fn app() -> Element {
    let animation = use_trait_animation(|context| {
        context.add_tween("offset", 0.0);
        context.add_tween("opacity", 0.0);
    });

    let [offset, opacity] = [
        animation.get::<f32>("offset"),
        animation.get::<f32>("opacity"),
    ];

    use_hook(move || {
        animation.play(
            "opacity",
            PathAnimation::default()
                .insert(1.0, Curve::FAST_OUT_SLOW_IN, 1000)
                .insert_delayed(0.0, Curve::FAST_OUT_SLOW_IN, 1000, 500),
        );

        animation.play(
            "offset",
            PathAnimation::default().insert_delayed(256.0, Curve::EASE_IN_OUT_CIRC, 500, 1000),
        );
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
