use std::{sync::mpsc::{Receiver, Sender}};

use crate::sensor::{ReadingType, Target, Actual};
pub mod sensor;
pub mod actuator;
pub mod transmission_control;
pub trait Actions{
    fn new() -> Self;
}
pub trait Actuator{
    fn calculate_pid(actual: &mut f32, target: &mut f32, elapsed_mil: u64);
    fn recieve_transmission(sensor_recv: Receiver<ReadingType>, counts: i32);
    fn process_singals(signals_vector: &mut Vec<ReadingType>,data: &ReadingType, current_arm_status: Actual, object_status: Target);
    // fn adjust();            
    // fn avoid_obstacles();
    // fn filter_noise();
}
pub trait Sensing{
    const TOKEN: &'static str="This.@BoxIs!!V#ALid";
    fn assign_data(sample_boxes: i32) -> Self;
    fn generate_keys(index: i32)-> String;
    fn filter_noise(&self)-> Self;
    fn explore(&self);
    fn detect_noise();
    fn standardize_data();
    fn transmit_data(&self, sensor_send: Sender<ReadingType>);
}
pub trait Control{
    fn init() -> Self;
    fn clone(&self) -> Sender<ReadingType>;
    fn simulation_control(self);
} 
