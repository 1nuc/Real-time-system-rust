use crate::Actions;

pub struct Actual{ 
    pub force: f32,
    pub velocity: f32,
    pub position: f32,
}
impl Actions for Actual{

    fn new() -> Self{
        Self {
            force: rand::random::<f32>(), 
            velocity: rand::random::<f32>(),
            position: rand::random::<f32>(),
        }
    }
} 

pub struct Target{ 
    pub force: f32,
    pub velocity: f32,
    pub position: f32,
}
impl Actions for Target{

    fn new() -> Self{
        Self{
            force: rand::random::<f32>(), 
            velocity: rand::random::<f32>(),
            position: rand::random::<f32>(),
        }
    }
} 
