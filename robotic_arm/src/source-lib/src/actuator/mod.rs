use crate::{Actuator,transmission_control::TransmissionChannel, Actions};
use float_eq::float_eq;
use std::thread;
use std::time::Duration;
use advanced_pid::{prelude::*, PidGain, Pid};

impl Actions for PidGain{
    fn new()-> Self{
        Self{
            kp: 1.0,
            ki: 0.8,
            kd: 0.1,
        }
    }
}

impl Actuator for Pid{
    fn calculate_pid(actual: &mut f32, target: &mut f32, elapse_mil: u64) {   
        let gain = PidGain::new(); 
        let mut pid = Pid::new(gain.into());
        let dt = 0.1;
        loop {
            // Calculate control output
            let output = pid.update(*target, *actual, dt);

            *actual += (output - *actual) / 4.0;
            println!("Current Readings Measurement: {actual}, : {target}");
            // Sleep 100ms

            if float_eq!(actual, target, abs<= 0.00000_1){
                break;
            }
            thread::sleep(Duration::from_millis(elapse_mil));
        }
    }
    fn recieve_transmission() {
        
    }

    // fn adjust(){

}
