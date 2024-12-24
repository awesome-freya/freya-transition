use freya::prelude::*;
use freya_transition::{use_segmented_animation, Curve};

fn main() {
    launch(app);
}

fn app() -> Element {
    let animation = use_segmented_animation(|context| {
        context.add_tween("offset", 0.0);
        context.add_tween("opacity", 0.0);

        context.add_segment("opacity", 1.0, Curve::FAST_OUT_SLOW_IN, 1000);
        context.add_segment("offset", 128.0, Curve::EASE_IN_OUT_CIRC, 500);
        context.add_segment("opacity", 0.0, Curve::FAST_OUT_SLOW_IN, 1000);
    });

    let [offset, opacity] = [
        animation.get::<f32>("offset"),
        animation.get::<f32>("opacity"),
    ];

    use_hook(move || {
        animation.play();
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
