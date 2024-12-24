use super::ParametricCurve;

#[derive(Clone, PartialEq)]
pub struct Linear;

impl ParametricCurve<f32> for Linear {
    fn transform_internal(&self, t: f32) -> f32 {
        t
    }
}
