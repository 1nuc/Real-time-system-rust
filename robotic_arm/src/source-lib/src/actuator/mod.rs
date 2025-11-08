use crate::sensor::{ReadingType};
use crate::{Actuator,Actions};
use float_eq::float_eq;
use std::sync::mpsc::TryRecvError;
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
        let mut recv_counts=0;
        loop{
            if recv_counts -counts ==1{
                println!("all data have been recieved");
                break;
            }
            match sensor_recv.try_recv(){
                Ok(value) => {
                    println!("Readings recieved...");
                    Self::process_singals(&mut singals_vector,&value);
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
            // println!("Readings recieved...");
            // println!("{:?}", recv);
        // }
        println!("Sent: {counts}, recieved: {recv_counts}");
    }

    fn process_singals(signals_vector: &mut Vec<ReadingType>,data: &ReadingType){
        signals_vector.push(*data); //storing the value into a vector for storage purposes and for
                                    //future use
        println!("Actuator is processing the target :{:?}",data);
    }
    // fn adjust(){

}
