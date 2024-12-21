pub trait AnimationVector: Default {
    fn reset(&mut self);
    fn get(&self, index: usize) -> f32;
    fn set(&mut self, index: usize, value: f32);
    fn size(&self) -> usize;
}

#[derive(Default)]
pub struct AnimationVector1D(f32);

impl AnimationVector1D {
    pub fn new(v1: f32) -> Self {
        Self(v1)
    }
}

impl AnimationVector for AnimationVector1D {
    fn reset(&mut self) {
        self.0 = 0.0;
    }

    fn get(&self, index: usize) -> f32 {
        if index == 0 {
            self.0
        } else {
            0.0
        }
    }

    fn set(&mut self, index: usize, value: f32) {
        if index == 0 {
            self.0 = value;
        }
    }

    fn size(&self) -> usize {
        1
    }
}

#[derive(Default)]
pub struct AnimationVector2D([f32; 2]);

impl AnimationVector2D {
    pub fn new(v1: f32, v2: f32) -> Self {
        Self([v1, v2])
    }
}

impl AnimationVector for AnimationVector2D {
    fn reset(&mut self) {
        self.0.fill(0.0);
    }

    fn get(&self, index: usize) -> f32 {
        self.0.get(index).copied().unwrap_or_default()
    }

    fn set(&mut self, index: usize, value: f32) {
        if let Some(val) = self.0.get_mut(index) {
            *val = value;
        }
    }

    fn size(&self) -> usize {
        2
    }
}

#[derive(Default)]
pub struct AnimationVector3D([f32; 3]);

impl AnimationVector3D {
    pub fn new(v1: f32, v2: f32, v3: f32) -> Self {
        Self([v1, v2, v3])
    }
}

impl AnimationVector for AnimationVector3D {
    fn reset(&mut self) {
        self.0.fill(0.0);
    }

    fn get(&self, index: usize) -> f32 {
        self.0.get(index).copied().unwrap_or_default()
    }

    fn set(&mut self, index: usize, value: f32) {
        if let Some(val) = self.0.get_mut(index) {
            *val = value;
        }
    }

    fn size(&self) -> usize {
        3
    }
}

#[derive(Default)]
pub struct AnimationVector4D([f32; 4]);

impl AnimationVector4D {
    pub fn new(v1: f32, v2: f32, v3: f32, v4: f32) -> Self {
        Self([v1, v2, v3, v4])
    }
}

impl AnimationVector for AnimationVector4D {
    fn reset(&mut self) {
        self.0.fill(0.0);
    }

    fn get(&self, index: usize) -> f32 {
        self.0.get(index).copied().unwrap_or_default()
    }

    fn set(&mut self, index: usize, value: f32) {
        if let Some(val) = self.0.get_mut(index) {
            *val = value;
        }
    }

    fn size(&self) -> usize {
        4
    }
}
