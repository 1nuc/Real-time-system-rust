use crate::sensor::{Actual, ReadingType, Target};
use crate::{Actuator,Actions};
use float_eq::float_eq;
use std::io::Read;
use std::sync::mpsc::TryRecvError;
use std::thread::current;
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

            if float_eq!(actual, target, abs<= 0.00000_1){
                break;
            }
            thread::sleep(Duration::from_millis(elapse_mil));
        }
    }
    fn recieve_transmission(sensor_recv: Receiver<ReadingType>, counts: i32) {
        let mut singals_vector: Vec<ReadingType>=Vec::new();
        let mut recv_counts: i32=0;
        loop{
            if recv_counts -counts ==1{
                println!("all data have been recieved");
                break;
            }
            match sensor_recv.try_recv(){
                Ok(value) => {
                    println!("Readings recieved...");
                    Self::process_singals(&mut singals_vector,&value, recv_counts);
                    recv_counts+=1;
                },
                Err(TryRecvError::Empty)=> {
                    println!("Error in channel receiption: channel is empty");
                },
                Err(TryRecvError::Disconnected) => println!("channel reciever is disconnected"),
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

    fn process_singals(signals_vector: &mut Vec<ReadingType>,data: &ReadingType, recv_counts: i32){
        signals_vector.push(*data); //storing the value into a vector for storage purposes and for
                                    //future use
        let current_arm_status:Option<ReadingType>;
        let object_status: Option<ReadingType>;
        if let ReadingType::RoboticArm(actual)=*data{
            current_arm_status=Some(ReadingType::RoboticArm(actual));
        }else{
            current_arm_status=None;
        }

        if let ReadingType::ObjectBoxes(target)=*data{
            object_status=Some(ReadingType::ObjectBoxes(target));
       }
        else{
            object_status=None;
        }
        println!("Actuator is processing the target :{:?}",data);
        //TODO: processing Position
        if let(Some(ReadingType::RoboticArm(mut arm)), Some(ReadingType::ObjectBoxes(mut object))) = (current_arm_status, object_status){
                Pid::calculate_pid(&mut arm.position,&mut object.position, 100);
        //TODO: processing Temparture
                Pid::calculate_pid(&mut arm.temperature,&mut object.temperature, 100);
        //TODO: processing Force
                Pid::calculate_pid(&mut arm.force,&mut object.force, 100);
        //TODO: processing Velocity
                Pid::calculate_pid(&mut arm.velocity,&mut object.velocity, 100);
        }
    }
    // fn adjust(){

}
