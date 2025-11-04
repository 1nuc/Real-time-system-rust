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
    fn explore();
    fn detect_noise();
    fn standardize_data();
}

