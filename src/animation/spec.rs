use super::{vector::AnimationVector, vector_converters::TwoWayConverter};

pub trait AnimationSpec<T> {
    fn vectorize<V: AnimationVector>(
        self,
        converter: impl TwoWayConverter<T, V>,
    ) -> impl VectorizedAnimationSpec<V>;
}

pub trait VectorizedAnimationSpec<V: AnimationVector> {
    fn is_infinite(&self) -> bool;

    fn get_value_from_nanos(
        &mut self,
        play_time_nanos: i64,
        initial_value: V,
        target_value: V,
        initial_velocity: V,
    ) -> V;
    fn get_velocity_from_nanos(
        &mut self,
        play_time_nanos: i64,
        initial_value: V,
        target_value: V,
        initial_velocity: V,
    ) -> V;
    fn get_duration_nanos(&mut self, initial_value: V, target_value: V, initial_velocity: V)
        -> i64;
    fn get_end_velocity(
        &mut self,
        play_time_nanos: i64,
        initial_value: V,
        target_value: V,
        initial_velocity: V,
    ) -> V {
        self.get_velocity_from_nanos(
            play_time_nanos,
            initial_value,
            target_value,
            initial_velocity,
        )
    }
}

pub trait FloatAnimationSpec {
    fn get_value_from_nanos(
        &mut self,
        play_time_nanos: i64,
        initial_value: f32,
        target_value: f32,
        initial_velocity: f32,
    ) -> f32;
    fn get_velocity_from_nanos(
        &mut self,
        play_time_nanos: i64,
        initial_value: f32,
        target_value: f32,
        initial_velocity: f32,
    ) -> f32;
    fn get_duration_nanos(
        &mut self,
        initial_value: f32,
        target_value: f32,
        initial_velocity: f32,
    ) -> i64;
    
    fn get_end_velocity(
        &mut self,
        play_time_nanos: i64,
        initial_value: f32,
        target_value: f32,
        initial_velocity: f32,
    ) -> V {
        self.get_velocity_from_nanos(
            play_time_nanos,
            initial_value,
            target_value,
            initial_velocity,
        )
    }
}

impl<T: FloatAnimationSpec> AnimationSpec<f32> for T {
    fn vectorize<V: AnimationVector>(
        self,
        converter: impl TwoWayConverter<f32, V>,
    ) -> impl VectorizedAnimationSpec<V> {
        VectorizedFloatAnimationSpec::new(self)
    }
}

pub trait Animations {
    fn get(&self, index: usize) -> &impl FloatAnimationSpec;
}

struct SingleAnimation(Box<dyn FloatAnimationSpec>);

impl Animations for SingleAnimation {
    fn get(&self, index: usize) -> &impl FloatAnimationSpec {
        &self.0
    }
}

pub struct VectorizedFloatAnimationSpec<V: AnimationVector> {
    anims: Box<dyn Animations>,
    value_vector: Option<V>,
    velocity_vector: Option<V>,
    end_velocity_vector: Option<V>,
}

impl<V: AnimationVector> VectorizedFloatAnimationSpec<V> {
    fn new(anim: impl FloatAnimationSpec) -> Self {
        Self {
            anims: SingleAnimation(anim),
            value_vector: None,
            velocity_vector: None,
            end_velocity_vector: None,
        }
    }
}

impl<V: AnimationVector> VectorizedAnimationSpec<V> for VectorizedFloatAnimationSpec<V> {
    fn is_infinite(&self) -> bool {
        false
    }

    fn get_value_from_nanos(
        &mut self,
        play_time_nanos: i64,
        initial_value: V,
        target_value: V,
        initial_velocity: V,
    ) -> V {
        if self.value_vector.is_none() {
            self.value_vector.replace(V::default());
        }

        let Some(value_vector) = self.value_vector.as_mut() else {
            unreachable!()
        };

        for i in 0..value_vector.size() {
            value_vector.set(i, an)
        }
    }

    fn get_velocity_from_nanos(
        &mut self,
        play_time_nanos: i64,
        initial_value: V,
        target_value: V,
        initial_velocity: V,
    ) -> V {
        todo!()
    }

    fn get_duration_nanos(
        &mut self,
        initial_value: V,
        target_value: V,
        initial_velocity: V,
    ) -> i64 {
        todo!()
    }
}
