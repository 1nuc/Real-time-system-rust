use std::sync::mpsc::{TryRecvError};

pub mod sensor;
pub mod actuator;
pub mod  transmission_control;
pub trait Actions{
    fn new() -> Self;
}
pub trait Actuator{
    fn calculate_pid(actual: &mut f32, target: &mut f32, elapsed_mil: u64);
    fn recieve_transmission();
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
    fn transmit_data(&self);
}
pub trait TransmissionControl<T, E>{
    fn new() ->Self;
    fn send_packets(&self, packet: T);
    fn receive_packets(&self) -> Result<T,TryRecvError>;
} 
