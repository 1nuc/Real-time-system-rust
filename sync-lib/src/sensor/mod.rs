use std::{
    sync::{Arc, Mutex, MutexGuard,atomic::{AtomicI32,Ordering}},
    thread, time::Instant};
use crossbeam::{channel::*};
use manufacturer::{sensing_data::{Actual, Target, Readings}};

use crate::{ControlSync, SensingSync, Shared, transmission_control::{TransmissionChannel}};

#[derive(Clone, Copy, Debug)]
pub enum ReadingType{
    RoboticArm(Actual, Target, i32),
} 

impl Shared for Readings{
    type SharedLock<'a>=MutexGuard<'a, (Actual,Vec<(Target,String, i32)>)>;
    type Type= Arc<Mutex<(Actual,Vec<(Target,String, i32)>)>>;
}
#[allow(non_snake_case)]
impl SensingSync for Readings{
    fn sensor_control(&self, sensing_info: Self::Type,sensor_send: Sender<ReadingType>,feedback_recv: Receiver<ReadingType>, counts: Arc<AtomicI32>) {
        if feedback_recv.is_empty(){
            self.collect_data(sensing_info ,sensor_send, counts, 100);
        }
        else{
            let now=Instant::now();
            let objects=Arc::new(Mutex::new(Vec::new()));
            let mut arm_status=self.current_state;
            while !feedback_recv.is_empty(){
                let object_copy=Arc::clone(&objects);
                let feedback_recv_cloned=feedback_recv.clone();
                let arm=thread::spawn(move||{
                    let lock=object_copy.lock().expect("unable to lock");
                    TransmissionChannel::recieve_transmission_deadline(now, lock, feedback_recv_cloned)
                }).join().expect("unable to get the value");
                if let Some(val)=arm{
                    arm_status=val;
                }
            }
            match Arc::try_unwrap(objects){
                Ok(val)=>{
                    println!("Objects are safe");
                    let object=val.into_inner().unwrap();
                    let objects_lock=Arc::new(Mutex::new((arm_status, object)));
                    self.collect_data(objects_lock, sensor_send, counts, 100);
                },
                Err(err) =>println!("objects cannot be unwrapped:{:?}", err),
            }
        }
    }

    // Collect data in the initial state
    fn collect_data(&self, sensing_info: Self::Type, sensor_send: Sender<ReadingType>, counts: Arc<AtomicI32>, time: u64) {
        let value=counts.load(Ordering::Acquire);
        for _ in 0..value{
            let tx_copy=sensor_send.clone();
            let packets=Arc::clone(&sensing_info);
            thread::spawn(move ||{
                Self::sensor_workflow(packets, tx_copy, time);
            });
        }
        drop(sensor_send);
    }
    
    //start threads to send the data through packets
    fn sensor_workflow(packets: Self::Type, tx_copy: Sender<ReadingType>, time: u64){
        let mut data=packets.lock().expect("error while locking");
        if let Some(value)=data.1.pop(){
            TransmissionChannel::transmit_data(ReadingType::RoboticArm(data.0, value.0, value.2), tx_copy, data, time);
        }else{
            println!("all values have been submitted");
            drop(tx_copy);
            drop(data);
        }
    }

}


