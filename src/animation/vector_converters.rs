use super::vector::AnimationVector;

pub trait TwoWayConverter<T, V: AnimationVector> {
    fn convert_to_vector(&self, value: T) -> V;
    fn convert_from_vector(&self, vector: V) -> T;
}

pub struct TwoWayConverterImpl<T, V: AnimationVector> {
    convert_to_vector: fn(T) -> V,
    convert_from_vector: fn(V) -> T,
}

impl<T, V: AnimationVector> TwoWayConverter<T, V> for TwoWayConverterImpl<T, V> {
    fn convert_to_vector(&self, value: T) -> V {
        (self.convert_to_vector)(value)
    }

    fn convert_from_vector(&self, vector: V) -> T {
        (self.convert_from_vector)(vector)
    }
}
