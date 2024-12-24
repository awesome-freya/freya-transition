use super::{Curve, Tween, Value};
use freya::{
    dioxus_core::Task,
    hooks::{use_platform, UsePlatform},
    prelude::{spawn, use_memo, use_signal, Memo, Readable, Signal, Writable},
};
use indexmap::IndexMap;
use std::{ops::Range, time::Instant};

#[derive(Default, PartialEq, Eq)]
pub struct Context {
    tweens: IndexMap<String, Signal<Tween>>,
    segments: IndexMap<Range<u64>, Signal<Segment>>,
    duration: u64,
}

struct Segment {
    tween: String,
    value: Value,
    curve: Curve,
    duration: u64,
}

impl Context {
    pub fn add_tween<K: Into<String>, V: Into<Value>>(&mut self, key: K, value: V) {
        let value = value.into();

        self.tweens
            .insert(key.into(), Signal::new(Tween::new(value.clone(), value)));
    }

    pub fn add_segment<K: Into<String>, V: Into<Value>>(
        &mut self,
        tween: K,
        value: V,
        curve: Curve,
        duration: u64,
    ) {
        let last = self
            .segments
            .last()
            .map(|(key, _)| key.end)
            .unwrap_or_default();

        self.duration += duration;
        self.segments.insert(
            last..(last + duration),
            Signal::new(Segment {
                tween: tween.into(),
                value: value.into(),
                curve,
                duration,
            }),
        );
    }
}

/// So basic idea is:
/// segments: 0:[0.0 -> 1.0; linear 200ms], 1:[20px -> 40px; linear 300ms]
/// and then
/// run(): segment0-[0ms..200ms] -> segment1-[200ms..300ms]
#[derive(PartialEq, Clone, Copy)]
pub struct SegmentedAnimation {
    context: Memo<Context>,
    is_running: Signal<bool>,
    has_run_yet: Signal<bool>,
    platform: UsePlatform,
    task: Signal<Option<Task>>,
}

impl SegmentedAnimation {
    #[must_use]
    pub fn is_playing(&self) -> bool {
        *self.is_running.read()
    }

    #[must_use]
    pub fn peek_has_run_yet(&self) -> bool {
        *self.has_run_yet.peek()
    }

    #[must_use]
    pub fn get<T: From<Value>>(&self, key: impl AsRef<str>) -> T {
        self.context
            .read()
            .tweens
            .get(key.as_ref())
            .unwrap()
            .read()
            .value
            .clone()
            .into()
    }

    pub fn play(&self) {
        self.run()
    }

    pub fn cancel(&self) {
        let mut task = self.task;

        if let Some(task) = task.write().take() {
            task.cancel();
        };
    }

    fn run(&self) {
        let ctx = self.context.peek();
        let platform = self.platform;
        let mut is_running = self.is_running;
        let mut ticker = platform.new_ticker();
        let segments = ctx.segments.clone();
        let mut tweens = ctx.tweens.clone();
        let duration = ctx.duration;
        let mut has_run_yet = self.has_run_yet;
        let mut task = self.task;

        // Cancel previous animations
        if let Some(task) = task.write().take() {
            task.cancel();
        }

        if !self.peek_has_run_yet() {
            *has_run_yet.write() = true;
        }

        is_running.set(true);

        let animation_task = spawn(async move {
            platform.request_animation_frame();

            let mut index = 0;
            let mut prev_frame = Instant::now();

            loop {
                // Wait for the event loop to tick
                ticker.tick().await;

                platform.request_animation_frame();

                index += prev_frame.elapsed().as_millis();

                // Advance the current segment
                if let Some((key, current_segment)) = segments
                    .iter()
                    .find(|(key, _)| key.contains(&(index as u64)))
                {
                    let current_segment = current_segment.read();

                    if let Some(tween) = tweens.get_mut(&current_segment.tween) {
                        let mut tween = tween.write();

                        if tween.curve != current_segment.curve {
                            tween.curve = current_segment.curve.clone();
                        }

                        if tween.destination != current_segment.value {
                            tween.to(current_segment.value.clone());
                        }

                        if tween.duration != current_segment.duration as f32 {
                            tween.set_duration(current_segment.duration);
                        }

                        tween.advance((index as u64 - key.start) as f32);
                    }
                }

                prev_frame = Instant::now();

                if index >= duration as u128 {
                    break;
                }
            }

            is_running.set(false);

            task.write().take();
        });

        // Cancel previous animations
        task.write().replace(animation_task);
    }
}

pub fn use_segmented_animation(run: impl Fn(&mut Context) + 'static) -> SegmentedAnimation {
    let platform = use_platform();
    let is_running = use_signal(|| false);
    let has_run_yet = use_signal(|| false);
    let task = use_signal(|| None);

    let context = use_memo(move || {
        let mut context = Context::default();

        run(&mut context);

        context
    });

    SegmentedAnimation {
        context,
        is_running,
        has_run_yet,
        platform,
        task,
    }
}
