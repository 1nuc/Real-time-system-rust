use rand::prelude::*;
use float_eq::float_eq;
use std::thread;
use std::time::Duration;
use advanced_pid::{prelude::*, PidGain, Pid};
use source_lib::library::Actions;
#[derive(Debug, Default)]
struct Actual{ 
    force: f32,
    velocity: f32,
    position: f32,
}
impl Actions for Actual{

    fn new() -> Self{
        Self {
            force: rand::random::<f32>(), 
            velocity: rand::random::<f32>(),
            position: rand::random::<f32>(),
        }
    }
} 

#[derive(Default)]
struct Target{ 
    force: f32,
    velocity: f32,
    position: f32,
}
impl Actions for Target{

    fn new() -> Self{
        Self{
            force: rand::random::<f32>(), 
            velocity: rand::random::<f32>(),
            position: rand::random::<f32>(),
        }
    }
} 

fn calculate_pid(actual: &mut f32, target: &mut f32){   
    let gain = PidGain {
        kp: 1.0,
        ki: 0.8,
        kd: 0.1,
    };
    let mut pid = Pid::new(gain.into());
    let dt = 0.1;
    loop {
        // Calculate control output
        let output = pid.update(*target, *actual, dt);

        // Simulate the system respons
        *actual += (output - *actual) / 4.0;
        println!("Current Readings Measurement: {actual}, : {target}");
        // Sleep 100ms

        if float_eq!(actual, target, abs<= 0.00000_1){
           println!() 
        }
        thread::sleep(Duration::from_millis(100));
    }
}
fn main() {
    let mut sensor_data=Actual::new(); 
    let mut target_data=Target::new();
    calculate_pid(&mut sensor_data.force,&mut target_data.force);
    
}


