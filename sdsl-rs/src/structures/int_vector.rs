#[derive(Clone)]
pub struct IntVector<const WIDTH: u32> {
    _default_value: u32,
}

impl<const WIDTH: u32> IntVector<WIDTH> {
    pub fn new(default_value: u32) -> Self {
        Self {
            _default_value: default_value,
        }
    }
}
