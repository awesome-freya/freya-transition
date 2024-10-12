use super::{Curve, Tween, Value};
use freya::{
    dioxus_core::Task,
    hooks::{use_platform, UsePlatform},
    prelude::{spawn, use_signal, Readable, Signal, Writable},
};
use indexmap::IndexMap;
use std::{hash::Hash, time::Instant};

#[derive(PartialEq)]
pub struct Context<State: PartialEq + Eq + Hash> {
    init: State,
    states: IndexMap<State, IndexMap<String, Value>>,
    tweens: IndexMap<String, Signal<Tween>>,
}

impl<State: PartialEq + Eq + Hash> Context<State> {
    pub fn new(init: State, states: IndexMap<State, IndexMap<String, Value>>) -> Self {
        Self {
            init,
            states,
            tweens: IndexMap::default(),
        }
    }

    pub fn add_tween<K: Into<String>>(&mut self, key: K, curve: Curve, duration: u64) {
        let key = key.into();
        let value = self
            .states
            .get(&self.init)
            .and_then(|values| values.get(&key))
            .unwrap();

        self.tweens.insert(
            key,
            Signal::new(
                Tween::new(value.clone(), value.clone())
                    .curve(curve)
                    .duration(duration),
            ),
        );
    }
}

#[derive(Default)]
pub struct State {
    properties: IndexMap<String, Value>,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn property(mut self, name: impl Into<String>, value: impl Into<Value>) -> Self {
        self.properties.insert(name.into(), value.into());

        self
    }
}

pub struct TransitionBuilder<S: PartialEq + Eq + Hash> {
    init: S,
    properties: IndexMap<String, (u64, Curve)>,
    states: IndexMap<S, IndexMap<String, Value>>,
}

impl<S: PartialEq + Eq + Hash> TransitionBuilder<S> {
    pub fn new(init: S) -> Self {
        Self {
            init,
            properties: IndexMap::default(),
            states: IndexMap::default(),
        }
    }

    pub fn property(mut self, name: impl Into<String>, duration: u64, curve: Curve) -> Self {
        self.properties.insert(name.into(), (duration, curve));

        self
    }

    pub fn state(mut self, name: S, state: State) -> Self {
        self.states.insert(name, state.properties);

        self
    }

    pub fn build(self) -> Transition<S> {
        let platform = use_platform();
        let is_running = use_signal(|| false);
        let has_run_yet = use_signal(|| false);
        let task = use_signal(|| None);
        let tweens = self
            .properties
            .into_iter()
            .map(|(key, (duration, curve))| {
                let value = self
                    .states
                    .get(&self.init)
                    .and_then(|values| values.get(&key))
                    .unwrap();

                (
                    key,
                    Signal::new(
                        Tween::new(value.clone(), value.clone())
                            .duration(duration)
                            .curve(curve),
                    ),
                )
            })
            .collect();

        let context = Context {
            init: self.init,
            states: self.states,
            tweens,
        };

        Transition {
            context: use_signal(|| context),
            is_running,
            has_run_yet,
            platform,
            task,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Transition<S: PartialEq + Eq + Hash + 'static> {
    context: Signal<Context<S>>,
    is_running: Signal<bool>,
    has_run_yet: Signal<bool>,
    platform: UsePlatform,
    task: Signal<Option<Task>>,
}

impl<S: PartialEq + Eq + Hash> Transition<S> {
    pub fn builder(init: S) -> TransitionBuilder<S> {
        TransitionBuilder::new(init)
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

    pub fn run<T: Into<S>>(&self, state: T) {
        let state = state.into();
        let context = self.context.peek();
        let platform = self.platform;
        let mut is_running = self.is_running;
        let mut ticker = platform.new_ticker();
        let mut tweens = context.tweens.clone();
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

        if let Some(values) = context.states.get(&state) {
            for (key, value) in values {
                if let Some(tween) = tweens.get_mut(key) {
                    tween.write().to(value.clone());
                }
            }
        }

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
                for value in tweens.values_mut() {
                    value.write().advance(index as f32);
                }

                prev_frame = Instant::now();

                let is_finished = tweens.values().all(|value| value.peek().is_done(index));

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

pub fn use_transition<
    State: PartialEq + Eq + Hash + 'static,
    K: Into<String>,
    V: Into<Value>,
    Map: IntoIterator<Item = (K, V)>,
    States: IntoIterator<Item = (State, Map)> + 'static,
>(
    init: State,
    states: States,
    run: impl FnOnce(&mut Context<State>) + 'static,
) -> Transition<State> {
    let platform = use_platform();
    let is_running = use_signal(|| false);
    let has_run_yet = use_signal(|| false);
    let task = use_signal(|| None);
    let mut context = Context::new(
        init,
        states
            .into_iter()
            .map(|(key, map)| {
                (
                    key,
                    map.into_iter()
                        .map(|(key, value)| (key.into(), value.into()))
                        .collect(),
                )
            })
            .collect(),
    );

    run(&mut context);

    Transition {
        context: use_signal(|| context),
        is_running,
        has_run_yet,
        platform,
        task,
    }
}
