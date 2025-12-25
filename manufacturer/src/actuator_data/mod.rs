use advanced_pid::{prelude::*, PidGain, Pid};
use crate::PIDSetup;
use float_eq::float_eq;
use std::{thread, time::Duration};

pub struct PID{
    pid_control: PidGain,
}

impl PIDSetup for PID{
    fn new()-> Self{
        let gain=PidGain{
            kp: 1.0,
            ki: 0.8,
            kd: 0.1,
        };
        Self{
            pid_control:gain,
        }
    }
    fn calculate_pid<'a>(actual: &mut f32, target: &mut f32, elapse_mil: u64, measurement: &'a str) ->f32 {   
        let gain = PID::new(); 
        let mut pid = Pid::new(gain.pid_control.into());
        let dt = 0.1;
        loop {
            let output = pid.update(*target, *actual, dt);

            *actual += (output - *actual) / 4.0;

            if float_eq!(actual, target, abs<= 0.0_1){
                break;
            }
            thread::sleep(Duration::from_millis(elapse_mil));
        }
        println!("Arm: {}, changed to: {}", measurement, actual);
        *actual
        
    }
}

