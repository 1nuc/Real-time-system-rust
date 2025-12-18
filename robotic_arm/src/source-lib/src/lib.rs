use std::{time::Instant,sync::{Arc, atomic::{AtomicI32}, Mutex}};
use crossbeam::channel::*;
use crate::sensor::{ReadingType, Target, Actual};
pub mod sensor;
pub mod actuator;
pub mod transmission_control;
pub trait Actions{
    fn new() -> Self;
    fn init(temp: f32, force: f32, vel: f32, pos: f32)-> Self;
}
pub trait Shared{
    type SharedLock<'a>;
    type Type;
}
pub trait Actuator <'a>: Shared{
    fn calculate_pid(actual: &mut f32, target: &mut f32, elapsed_mil: u64, measurment: &'a str) -> f32;
    fn actuator_control(sensing_info: Self::Type,sensor_recv: Receiver<ReadingType>, counts: i32, feedback_send: Sender<ReadingType>);
    fn process_singals(lock: Self::SharedLock<'a>, current_arm_status: Actual, object_status: Target, recv_counts: Arc<AtomicI32>, id: i32, feedback_send: Sender<ReadingType>);
    fn process_feedback(pos: f32, temparture: f32, force: f32, vel: f32);
}
pub trait Sensing: Shared{
    const TOKEN: &'static str="This.@BoxIs!!V#ALid";
    fn assign_data(sample_boxes: i32) -> Self;
    fn generate_keys(index: i32)-> String;
    fn filter_noise(&self)-> Self;
    fn explore(&self);
    fn update_indices(&mut self, id: i32, new_current_state: Actual)->Self;
    fn collect_data(&self, sensing_info: Self::Type,sensor_send: Sender<ReadingType>);
    fn sensor_control(&self, sensing_info: Self::Type,sensor_send: Sender<ReadingType>, feedback_recv: Receiver<ReadingType>);
    fn sensor_workflow(packets: Self::Type, tx_copy: Sender<ReadingType>);
}
pub trait Control: Shared{
    fn init()-> Self;
    fn simulation_control(self);
    fn transmit_data<'a>(data: ReadingType, sensor_send: Sender<ReadingType>, lock: Self::SharedLock<'a>);
    fn recieve_transmission(sensor_recv: Receiver<ReadingType>)->Option<ReadingType>;
    fn recieve_transmission_deadline(now: Instant,object_lock:Arc<Mutex<Vec<(Target, String, i32)>>>, arm_status: Arc<Actual>,feedback_recv: Receiver<ReadingType>);
} 
