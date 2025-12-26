use crate::sensing_data::{Actual, Target};

pub mod sensing_data;
pub mod actuator_data;

pub trait Initiation{
    fn new() -> Self;
    fn init(temp: f32, force: f32, vel: f32, pos: f32)-> Self;
}

pub trait Actions {
    const TOKEN: &'static str="This.@BoxIs!!V#ALid";
    fn assign_data(sample_boxes: i32) -> Self;
    fn generate_keys(index: i32)-> String;
    fn filter_noise(&self)-> Self;
    fn explore(&self);
    fn update_indices(&mut self, id: i32, new_current_state: Actual)->Self;
}
pub trait PIDSetup{
    fn new() -> Self;
    fn calculate_pid<'a>(actual: &mut f32, target: &mut f32, measurment: &'a str) -> f32;
}
