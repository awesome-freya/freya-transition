use super::Animation;
use crate::{Curve, Tween, Value};

struct Frame {
    position: f32,
    value: Value,
    curve: Option<Curve>,
}

#[derive(Default)]
pub struct KeyFrameAnimation {
    frames: Vec<Frame>,
    current_frame: Option<usize>,
    duration: u64,
}

impl KeyFrameAnimation {
    pub fn keyframe_at<T: Into<Value>>(
        mut self,
        position: f32,
        value: T,
        curve: Option<Curve>,
    ) -> Self {
        self.frames.push(Frame {
            position,
            value: value.into(),
            curve,
        });

        self
    }

    pub fn duration(mut self, duration: u64) -> Self {
        self.duration = duration;

        self
    }

    fn update_current_keyframe(&mut self, time: f32) {
        // Common cases, reversing/wrapping
        if !self.frames.is_empty() && time == 0.0 {
            self.current_frame = Some(0);
            return;
        }
        if !self.frames.is_empty() && time == self.get_duration() as f32 {
            self.current_frame = Some(self.frames.len() - 1);
            return;
        }

        if let Some(k) = self.current_frame {
            if self.frames.len() <= k {
                self.current_frame = None;
            }

            if self.frames[k].position > time {
                for i in (0..self.current_frame.unwrap_or(0)).rev() {
                    if self.frames[i].position <= time {
                        self.current_frame = Some(i);
                        return;
                    }

                    self.current_frame = None;
                }
            } else {
                let copy = self.current_frame;
                self.current_frame = None;

                for i in copy.unwrap_or(0)..self.frames.len() {
                    if self.frames[i].position > time {
                        break;
                    } else {
                        self.current_frame = Some(i)
                    }
                }
            }
        } else if !self.frames.is_empty() {
            self.current_frame = Some(0);
            self.update_current_keyframe(time);
        }
    }
}

impl Animation for KeyFrameAnimation {
    fn init(&self, _: &mut Tween) {}

    fn advance(&mut self, tween: &mut Tween, index: u128) {
        let time = index as f32 / self.duration as f32;

        self.update_current_keyframe(time);

        if let Some((frame, curve)) = self.current_frame.map(|frame| {
            (
                &self.frames[frame],
                self.frames[frame].curve.clone().unwrap_or(Curve::None),
            )
        }) {
            if tween.curve != curve {
                tween.curve = curve.clone();
            }

            if tween.destination != frame.value {
                tween.to(frame.value.clone());
            }

            tween.advance(index as f32);
        }
    }

    fn get_duration(&self) -> u64 {
        self.duration
    }
}
