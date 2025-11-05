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
pub trait Sensing<'a>{
    const TOKEN: &'a str="This.@BoxIs!!V#ALid";
    fn assign_data(sample_boxes: i32) -> Self;
    fn generate_keys(index: i32, defaulted_key_owned: &'a String)-> &'a str;
    fn explore(&self);
    fn detect_noise();
    fn standardize_data();
}

