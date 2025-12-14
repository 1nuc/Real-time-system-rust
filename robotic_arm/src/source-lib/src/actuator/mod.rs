use crate::sensor::{Actual, ReadingType, Target};
use crate::{Actions, Actuator, Shared};
use float_eq::float_eq;
use crossbeam::channel::*;
use std::{
    thread, time::Duration, sync::{
        Arc, Mutex, MutexGuard, atomic::{
            AtomicI32, Ordering}}};
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

impl Shared for Pid{
    // defining the type of the lock for the implementation of this struct
    type Type= Arc<Mutex<(Actual,Vec<(Target,String)>)>>;
    type SharedLock<'a>=MutexGuard<'a, (Actual,Vec<(Target,String)>)>;
}
impl<'a> Actuator<'a> for Pid{
    fn calculate_pid(actual: &mut f32, target: &mut f32, elapse_mil: u64, measurement: &'a str) {   
        let gain = PidGain::new(); 
        let mut pid = Pid::new(gain.into());
        let dt = 0.1;
        loop {
            // Calculate control output
            let output = pid.update(*target, *actual, dt);

            *actual += (output - *actual) / 4.0;
            // println!("Arm: {}, changed to: {}", measurement, actual);
            // Sleep 100ms

            if float_eq!(actual, target, abs<= 0.0_1){
                break;
            }
            thread::sleep(Duration::from_millis(elapse_mil));
        }
        
    }
    fn actuator_control(sensing_info: Self::Type,sensor_recv: Receiver<ReadingType>, counts: i32, feedback_send: Sender<ReadingType>) {
        let recv_counts=Arc::new(AtomicI32::new(0));
        loop{
            if recv_counts.load(Ordering::Acquire)==counts{
                println!("all data have been recieved");
                break;
            }
            else if sensor_recv.is_empty(){
                println!("Channel is empty no messaging recieved");
                thread::sleep(Duration::from_millis(200));
            }
            else {
                for _ in 0..counts{
                    let receiver_lock=Arc::clone(&sensing_info);
                    // let recv_counts_cloned=Arc::clone(&recv_counts);
                    let sensor_recv_cloned=sensor_recv.clone();
                    Self::recieve_transmission(receiver_lock, sensor_recv_cloned,recv_counts.clone(), feedback_send.clone());
                }
                println!("Sent: {counts}, recieved: {:?}", recv_counts);
            }
        }
    }
    fn recieve_transmission(sensing_lock: Self::Type,sensor_recv: Receiver<ReadingType>,ref_value:Arc<AtomicI32>,feedbakc_send: Sender<ReadingType>) {
        thread::spawn(move || {
            match sensor_recv.recv(){
                Ok(value) => {
                    println!("Readings recieved...");
                    let lock=sensing_lock.lock().unwrap();
                    let ReadingType::RoboticArm(arm, object)=value;
                    println!("object {:?}: {:?}", ref_value, object);
                    Self::process_singals(lock, &value, arm, object,ref_value);
                   // one issue detected is that the counts should increment only when the boxes are lifted not when they recieved, or the logic of the loop should change 
                },
                Err(RecvError)=> {
                    println!("Error in channel receiption: channel is empty");
                    //TODO: Some logic should be made to avoid channel collapse
                },
            }
        });
    }

    fn process_singals<T>(lock: MutexGuard<T>,_data: &ReadingType, mut current_arm_status: Actual, mut object_status: Target, recv_counts: Arc<AtomicI32>){
//TODO: processing Position
        println!("altering position");
        Pid::calculate_pid(&mut current_arm_status.position,&mut object_status.position,1, "Position");
        thread::sleep(Duration::from_millis(10));
//TODO: processing Temparture
        println!("altering temprature");
        Pid::calculate_pid(&mut current_arm_status.temperature,&mut object_status.temperature, 1, "Temprature");
        thread::sleep(Duration::from_millis(10));
// //TODO: processing Force
        println!("altering force");
        Pid::calculate_pid(&mut current_arm_status.force,&mut object_status.force, 1, "Force");
        thread::sleep(Duration::from_millis(10));
//TODO: processing Velocity
        println!("altering velocity");
        Pid::calculate_pid(&mut current_arm_status.velocity,&mut object_status.velocity, 1, "Velocity");
        thread::sleep(Duration::from_millis(10));
        recv_counts.fetch_add(1, Ordering::Release);
        drop(lock);
    }
    // fn adjust(){

}
