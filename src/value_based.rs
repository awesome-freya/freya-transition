use super::{Curve, Tween, Value};
use freya::{
    dioxus_core::Task,
    hooks::{use_platform, UsePlatform},
    prelude::{spawn, use_memo, use_signal, Memo, Readable, Signal, Writable},
};
use std::{collections::HashMap, hash::Hash, time::Instant};

#[derive(Default, PartialEq, Eq)]
pub struct Context {
    tweens: HashMap<String, Signal<Tween>>,
}

impl Context {
    pub fn add_tween<K: Into<String>, V: Into<Value>>(
        &mut self,
        key: K,
        value: V,
        curve: Curve,
        duration: u64,
    ) {
        let value = value.into();

        self.tweens.insert(
            key.into(),
            Signal::new(
                Tween::new(value.clone(), value)
                    .curve(curve)
                    .duration(duration),
            ),
        );
    }

    pub fn add_tween_delayed<K: Into<String>, V: Into<Value>>(
        &mut self,
        key: K,
        value: V,
        curve: Curve,
        duration: u64,
        delay: u64,
    ) {
        let value = value.into();

        self.tweens.insert(
            key.into(),
            Signal::new(
                Tween::new(value.clone(), value)
                    .curve(curve)
                    .duration(duration)
                    .delay(delay),
            ),
        );
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Transition {
    context: Memo<Context>,
    is_running: Signal<bool>,
    has_run_yet: Signal<bool>,
    platform: UsePlatform,
    task: Signal<Option<Task>>,
}

impl Transition {
    pub fn set<K: AsRef<str>, V: Into<Value>>(&self, key: K, value: V) {
        let context = self.context.peek();
        let mut tween = *context
            .tweens
            .get(key.as_ref())
            .unwrap_or_else(|| panic!("failed to get tween with {} name", key.as_ref()));

        tween.write().to(value.into());
    }

    pub fn set_duration<K: AsRef<str>>(&self, key: K, millis: u64) {
        let context = self.context.peek();
        let mut tween = *context.tweens.get(key.as_ref()).unwrap();

        tween.write().set_duration(millis);
    }

    pub fn forced_set<K: AsRef<str>, V: Into<Value>>(&self, key: K, value: V) {
        let context = self.context.peek();
        let mut tween = *context.tweens.get(key.as_ref()).unwrap();

        tween.write().set(value.into());
    }

    #[must_use]
    pub fn get<V: From<Value>>(&self, key: impl AsRef<str>) -> V {
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

    #[must_use]
    pub fn is_playing(&self) -> bool {
        *self.is_running.read()
    }

    #[must_use]
    pub fn peek_has_run_yet(&self) -> bool {
        *self.has_run_yet.peek()
    }

    pub fn play_all(&self) {
        self.run::<_, _, Vec<(&str, f32)>>(None)
    }

    pub fn play<K: Into<String> + Hash + Eq, V: Into<Value>, I: IntoIterator<Item = (K, V)>>(
        &self,
        filter: I,
    ) {
        self.run(Some(filter))
    }

    pub fn cancel(&self) {
        let mut task = self.task;

        if let Some(task) = task.write().take() {
            task.cancel();
        };
    }

    fn run<K: Into<String> + Hash + Eq, V: Into<Value>, I: IntoIterator<Item = (K, V)>>(
        &self,
        filter: Option<I>,
    ) {
        let ctx = self.context.peek();
        let platform = self.platform;
        let mut is_running = self.is_running;
        let mut ticker = platform.new_ticker();
        let mut values = ctx.tweens.clone();
        let mut has_run_yet = self.has_run_yet;
        let mut task = self.task;

        if let Some(filter) = filter {
            let mut filter: HashMap<_, _> = filter
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect();

            values.retain(|key, _| filter.contains_key(key));

            for (key, value) in &mut values {
                value.write().to(filter.remove(key).unwrap());
            }
        }

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

                // Advance the animations
                for value in values.values_mut() {
                    value.write().advance(index as f32);
                }

                prev_frame = Instant::now();

                let is_finished = values.values().all(|value| value.peek().is_done(index));

                if is_finished {
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

pub fn use_transition(run: impl Fn(&mut Context) + 'static) -> Transition {
    let platform = use_platform();
    let is_running = use_signal(|| false);
    let has_run_yet = use_signal(|| false);
    let task = use_signal(|| None);

    let context = use_memo(move || {
        let mut context = Context::default();

        run(&mut context);

        context
    });

    Transition {
        context,
        is_running,
        has_run_yet,
        platform,
        task,
    }
}
