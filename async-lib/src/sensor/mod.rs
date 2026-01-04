use std::{
    sync::{Arc,atomic::{AtomicI32,Ordering}}};

use flume::*;
use tokio::{task, sync::{
    MutexGuard, Mutex}, time::Instant 
};
use manufacturer::{sensing_data::{Actual, Target, Readings}};

use crate::{Control, Sensing, Shared, transmission_control::{TransmissionChannel}};

#[derive(Clone, Copy, Debug)]
pub enum ReadingType{
    RoboticArm(Actual, Target, i32),
} 

impl Shared for Readings{
    type SharedLock<'a>=MutexGuard<'a, (Actual,Vec<(Target,String, i32)>)>;
    type Type= Arc<Mutex<(Actual,Vec<(Target,String, i32)>)>>;
}
#[allow(non_snake_case, unused_variables)]
impl Sensing for Readings{
    async fn sensor_control(&self, sensing_info: Self::Type,sensor_send: Sender<ReadingType>,feedback_recv: Receiver<ReadingType>, counts: Arc<AtomicI32>) {
        if feedback_recv.is_empty(){
            self.collect_data(sensing_info ,sensor_send, counts).await;
        }
        else{
            let now=Instant::now();
            let mut objects=Arc::new(Mutex::new(Vec::new()));
            let mut arm_status=Arc::new(self.current_state);
            let value=counts.load(Ordering::Acquire);
            let mut handler=vec![];
            for i in 0..value{
                let object_copy=Arc::clone(&mut objects);
                let feedback_recv_cloned=feedback_recv.clone();
                let arm_status_cloned=Arc::clone(&mut arm_status);
                let handle=task::spawn(async move{
                    let lock=object_copy.lock().await;
                    TransmissionChannel::recieve_transmission_deadline(now.into(), lock, arm_status_cloned, feedback_recv_cloned).await;
                    //TODO: Add timeout function
                });
                handler.push(handle);
            }
            for handle in handler{
                handle.await.unwrap();
            }
            match Arc::try_unwrap(objects){
                Ok(val)=>{
                    let object=val.into_inner();
                    let arm_unwrapped=Arc::try_unwrap(arm_status).unwrap();
                    let objects_lock=Arc::new(Mutex::new((arm_unwrapped, object)));
                    self.collect_data(objects_lock, sensor_send, counts).await;
                },
                Err(err) =>{
                    println!("objects cannot be unwrapped:{:?}", err);
                    return;
                }
            }
        }
    }

    // Collect data in the initial state
    async fn collect_data(&self, sensing_info: Self::Type, sensor_send: Sender<ReadingType>, counts: Arc<AtomicI32>) {
        let value=counts.load(Ordering::Acquire);
        let mut handler=vec![];
        for _ in 0..=value{
            let tx_copy=sensor_send.clone();
            let packets=Arc::clone(&sensing_info);
            let handle=task::spawn(async move {
                Self::sensor_workflow(packets, tx_copy).await;
            });
            handler.push(handle);
        }
        for handle in handler{
            handle.await.unwrap();
        }
        drop(sensor_send);
    }
    
    //start threads to send the data through packets
    async fn sensor_workflow(packets: Self::Type, tx_copy: Sender<ReadingType>){
        let mut data=packets.lock().await;
        match data.1.pop(){
            Some(value) =>{
                TransmissionChannel::transmit_data(ReadingType::RoboticArm(data.0, value.0, value.2), tx_copy, data).await;
            },
            None =>{
                drop(tx_copy);
                drop(data);
                return
            },
        }
    }

}


