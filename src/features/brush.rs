pub struct Brush {
    pub size: f32,
}

impl Brush {
    pub fn enlarge(&mut self) {
        self.size += 1.5;
    }

    pub fn shrink(&mut self) {
        self.size = (self.size - 1.5).max(1.0);
    }
}

impl Default for Brush {
    fn default() -> Self {
        Self { size: 4.0 }
    }
}
