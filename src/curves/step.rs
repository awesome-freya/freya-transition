use super::ParametricCurve;

#[derive(Clone, PartialEq)]
pub struct Stepped {
    pub(super) is_initial_step_single_frame: bool,
    pub(super) is_final_step_single_frame: bool,
    pub(super) step_count: usize,
}

impl Stepped {
    #[must_use]
    pub const fn new(step_count: usize) -> Self {
        Self {
            is_initial_step_single_frame: false,
            is_final_step_single_frame: false,
            step_count,
        }
    }

    pub const fn initial_step_single_frame(&mut self) {
        self.is_initial_step_single_frame = true;
    }

    pub const fn final_step_single_frame(&mut self) {
        self.is_final_step_single_frame = true;
    }
}

impl ParametricCurve<f32> for Stepped {
    fn transform_internal(&self, t: f32) -> f32 {
        let mut step_time = t * self.step_count as f32;

        if self.is_initial_step_single_frame && t > 0.0 {
            step_time = step_time.ceil();
        } else if self.is_final_step_single_frame && t < 1.0 {
            step_time = step_time.floor();
        } else {
            step_time = step_time.round();
        }

        step_time / self.step_count as f32
    }
}
