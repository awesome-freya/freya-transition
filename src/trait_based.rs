use crate::{Animation, Tween, Value};
use freya::{
    dioxus_core::Task,
    hooks::{use_platform, UsePlatform},
    prelude::{spawn, use_memo, use_signal, Memo, Readable, Signal, Writable},
};
use indexmap::IndexMap;
use std::time::Instant;

#[derive(Default, PartialEq, Eq)]
pub struct Context {
    tweens: IndexMap<String, Signal<Tween>>,
}

impl Context {
    pub fn add_tween<K: Into<String>, V: Into<Value>>(&mut self, key: K, value: V) {
        let value = value.into();

        self.tweens
            .insert(key.into(), Signal::new(Tween::new(value.clone(), value)));
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct TraitBasedAnimation {
    context: Memo<Context>,
    is_running: Signal<bool>,
    has_run_yet: Signal<bool>,
    platform: UsePlatform,
    tasks: Signal<IndexMap<String, Option<Task>>>,
}

impl TraitBasedAnimation {
    #[must_use]
    pub fn is_playing(&self) -> bool {
        *self.is_running.read()
    }

    #[must_use]
    pub fn peek_has_run_yet(&self) -> bool {
        *self.has_run_yet.peek()
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

    pub fn set<V: Into<Value>>(&mut self, key: impl AsRef<str>, value: V) {
        let context = self.context.read();
        let mut tween = *context.tweens.get(key.as_ref()).unwrap();

        tween.write().set(value.into());
    }

    pub fn play<T: Animation + 'static>(
        &self,
        key: impl AsRef<str> + Copy + 'static,
        animation: T,
    ) {
        self.run(key, animation)
    }

    pub fn cancel(&self, key: impl AsRef<str>) {
        let mut task = self.tasks;

        if let Some(task) = task
            .write()
            .get_mut(key.as_ref())
            .and_then(|task| task.take())
        {
            task.cancel();
        };
    }

    pub fn cancel_all(&self) {
        let mut tasks = self.tasks;

        for task in tasks.write().values_mut() {
            if let Some(task) = task.take() {
                task.cancel()
            }
        }
    }

    fn run<T: Animation + 'static>(&self, key: impl AsRef<str> + Copy + 'static, mut animation: T) {
        let ctx = self.context.peek();
        let platform = self.platform;
        let mut is_running = self.is_running;
        let mut ticker = platform.new_ticker();
        let Some(mut tween) = ctx.tweens.get(key.as_ref()).cloned() else {
            return;
        };
        let mut has_run_yet = self.has_run_yet;
        let mut task = self.tasks;

        // Cancel previous animations
        if let Some(task) = task
            .write()
            .get_mut(key.as_ref())
            .and_then(|task| task.take())
        {
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

            animation.init(&mut tween.write());

            tween.write().set_duration(animation.get_duration());

            loop {
                // Wait for the event loop to tick
                ticker.tick().await;

                platform.request_animation_frame();

                index += prev_frame.elapsed().as_millis();

                // Advance the current segment
                animation.advance(&mut tween.write(), index);

                // println!("{:?}", tween.read().value);

                prev_frame = Instant::now();

                if index >= animation.get_duration() as u128 {
                    break;
                }
            }

            tween.write().advance(animation.get_duration() as f32);

            task.write()
                .get_mut(key.as_ref())
                .and_then(|task| task.take());

            if task.read().is_empty() {
                is_running.set(false);
            }
        });

        // Cancel previous animations
        if let Some(task) = task.write().get_mut(key.as_ref()) {
            task.replace(animation_task);
        };
    }
}

pub fn use_trait_animation(run: impl Fn(&mut Context) + 'static) -> TraitBasedAnimation {
    let platform = use_platform();
    let is_running = use_signal(|| false);
    let has_run_yet = use_signal(|| false);

    let context = use_memo(move || {
        let mut context = Context::default();

        run(&mut context);

        context
    });

    let tasks = use_signal(|| {
        context
            .read()
            .tweens
            .keys()
            .cloned()
            .map(|key| (key, None))
            .collect()
    });

    TraitBasedAnimation {
        context,
        is_running,
        has_run_yet,
        platform,
        tasks,
    }
}
