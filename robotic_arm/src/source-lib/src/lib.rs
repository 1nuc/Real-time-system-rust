use std::sync::{mpsc::{Receiver, Sender},MutexGuard,Arc, atomic::{AtomicI32}};

use crate::sensor::{ReadingType, Target, Actual};
pub mod sensor;
pub mod actuator;
pub mod transmission_control;
pub trait Actions{
    fn new() -> Self;
}
pub trait Shared{
    type SharedLock;
}
pub trait Actuator <'a>: Shared{
    fn calculate_pid(actual: &mut f32, target: &mut f32, elapsed_mil: u64, measurment: &'a str);
    fn recieve_transmission(sensing_info: Self::SharedLock,sensor_recv: Receiver<ReadingType>, counts: i32, feedback_send: Sender<ReadingType>);
    fn process_singals<T>(lock: MutexGuard<T>,data: &ReadingType, current_arm_status: Actual, object_status: Target, recv_counts: Arc<AtomicI32>);

    // fn adjust();            
    // fn avoid_obstacles();
    // fn filter_noise();
}
pub trait Sensing: Shared{
    const TOKEN: &'static str="This.@BoxIs!!V#ALid";
    fn assign_data(sample_boxes: i32) -> Self;
    fn generate_keys(index: i32)-> String;
    fn filter_noise(&self)-> Self;
    fn explore(&self);
    fn collect_data(&self, sensing_info: Self::SharedLock,sensor_send: Sender<ReadingType>);
    fn transmit_data(sensing_info: ReadingType, sensor_send: Sender<ReadingType>);
    fn sensor_control(&self, sensing_info: Self::SharedLock,sensor_send: Sender<ReadingType>, feedback_recv: Receiver<ReadingType>);
}
pub trait Control{
    fn init() -> Self;
    fn clone(&self) -> Sender<ReadingType>;
    fn simulation_control(self);
} 
