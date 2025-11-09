use crate::sensor::{Actual, ReadingType, Target};
use crate::{Actuator,Actions};
use float_eq::float_eq;
use std::sync::mpsc::{RecvError, TryRecvError};
use std::sync::Arc;
use std::{sync::mpsc::Receiver, thread};
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

            if float_eq!(actual, target, abs<= 0.00_1){
                break;
            }
            thread::sleep(Duration::from_millis(elapse_mil));
        }
    }
    fn recieve_transmission(sensor_recv: Receiver<ReadingType>, counts: i32) {
        let singals_vector=Arc::new(Vec::new());
        let mut recv_counts: i32=0;
        loop{
            if recv_counts ==counts{
                println!("all data have been recieved");
                break;
            }
            let vector=Arc::clone(&singals_vector);
            match sensor_recv.recv(){
                Ok(value) => {
                    println!("Readings recieved...");
                    //no need to reference the value is the the enum implements copy
                    thread::spawn(move || {
                        let ReadingType::RoboticArm(arm, object)=value;
                        Self::process_singals(vector.to_vec(),&value, arm, object);
                    });
                    
                    recv_counts+=1;
                },
                Err(RecvError)=> {
                    println!("Error in channel receiption: channel is empty");
                    //TODO: Some logic should be made to avoid channel collapse
                },
                // Err(TryRecvError::Disconnected) => println!("channel reciever is disconnected"),
            }
        }
        // I prefer to use this method as there is an error introduced
        // for recv in sensor_recv.try_iter(){
        //     println!("Readings recieved...");
        //     println!("{:?}", recv);
        //     recv_counts+=1;
        // }
        println!("Sent: {counts}, recieved: {recv_counts}");
    }

    fn process_singals(mut signals_vector: Vec<ReadingType>,data: &ReadingType, mut current_arm_status: Actual, mut object_status: Target){
        signals_vector.push(*data); //storing the value into a vector for storage purposes and for
                                    //future use
        println!("Actuator is processing the target :{:?}",data);
    //TODO: processing Position
            println!("calculate_pid");
            Pid::calculate_pid(&mut current_arm_status.position,&mut object_status.position, 1);
    //TODO: processing Temparture
    //         println!("calculate_pid");
    //         Pid::calculate_pid(&mut arm.temperature,&mut object.temperature, 1);
    // //TODO: processing Force
    //         println!("calculate_pid");
    //         Pid::calculate_pid(&mut arm.force,&mut object.force, 1);
    // //TODO: processing Velocity
    //         println!("calculate_pid");
    //         Pid::calculate_pid(&mut arm.velocity,&mut object.velocity, 1);
    }
    // fn adjust(){

}
