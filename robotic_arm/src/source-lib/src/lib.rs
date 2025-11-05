pub mod sensor;
pub mod actuator;
pub trait Actions{
    fn new() -> Self;
}
pub trait PidExtended{
    fn calculate_pid(actual: &mut f32, target: &mut f32, elapsed_mil: u64);
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
    fn send_packets();
}
pub trait TransmissionControl {
    fn simulation_control ();
    // fn send_packets(packet: Readings);
    // fn receive_packets();
} 
