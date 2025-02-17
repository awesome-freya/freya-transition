use super::Animation;
use crate::{Curve, Tween, Value};
use indexmap::IndexMap;
use std::ops::Range;

struct Segment {
    value: Value,
    curve: Curve,
    duration: u64,
}

#[derive(Default)]
pub struct PathAnimation {
    initial_value: Option<Value>,
    segments: IndexMap<Range<u64>, Segment>,
    duration: u64,
}

impl PathAnimation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initial<V: Into<Value>>(mut self, value: V) -> Self {
        self.initial_value = Some(value.into());

        self
    }

    pub fn insert<V: Into<Value>>(mut self, value: V, curve: Curve, duration: u64) -> Self {
        let last = self
            .segments
            .last()
            .map(|(key, _)| key.end)
            .unwrap_or_default();

        self.duration += duration;
        self.segments.insert(
            last..(last + duration),
            Segment {
                value: value.into(),
                curve,
                duration,
            },
        );

        self
    }

    pub fn insert_delayed<V: Into<Value>>(
        mut self,
        value: V,
        curve: Curve,
        duration: u64,
        delay: u64,
    ) -> Self {
        let last = self
            .segments
            .last()
            .map(|(key, _)| key.end)
            .unwrap_or_default();

        self.duration += duration + delay;
        self.segments.insert(
            (last + delay)..(last + delay + duration),
            Segment {
                value: value.into(),
                curve,
                duration,
            },
        );

        self
    }
}

impl Animation for PathAnimation {
    fn init(&self, tween: &mut Tween) {
        if let Some(value) = &self.initial_value {
            tween.set(value.clone());
        }
    }

    fn advance(&mut self, tween: &mut Tween, index: u128) {
        if let Some((key, current_segment)) = self
            .segments
            .iter()
            .find(|(key, _)| key.contains(&(index as u64)))
        {
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

    fn get_duration(&self) -> u64 {
        self.duration
    }
}
