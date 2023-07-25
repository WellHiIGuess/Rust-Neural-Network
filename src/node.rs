#[derive(Clone)]
pub struct Node {
    value: f32,
}

impl Node {
    pub fn new(value: f32) -> Node {
        Self { 
            value,
        }
    }

    pub fn get_value(self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) -> Result<(), String> {
        self.value = value;
        Ok(())
    }
}
