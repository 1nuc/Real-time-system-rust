use source_lib::{actuator::*, sensor::*, Actions, PidExtended};
use advanced_pid::Pid;
fn main() {
    let mut sensor_data=Actual::new(); 
    let mut target_data=Target::new();
    Pid::calculate_pid(&mut sensor_data.force,&mut target_data.force, 20);
    
}


