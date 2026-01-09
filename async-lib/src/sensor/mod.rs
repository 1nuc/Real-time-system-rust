use std::{
    sync::{Arc,atomic::{AtomicI32,Ordering}}};

use flume::*;
use tokio::{task, sync::{
    MutexGuard, Mutex}, time::{Instant, timeout, Duration} 
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
            match timeout(Duration::from_micros(500), self.handle_feedback(sensing_info, sensor_send.clone(), feedback_recv.clone(), counts.clone())).await{
                Ok(ok)=> println!("Feedback is sent in the allocated time"),
                Err(err) =>{
                     println!("Error timeout for the feedback to be sent.. entering fail safe mode ..");
                     drop(sensor_send);
                     while ! feedback_recv.is_empty(){
                         println!("draining channel");
                         let _=feedback_recv.recv_async().await;
                     }
                     counts.store(0, Ordering::Release);
                }
            }
        }
    }
    //handle the feedback back to the actuator
    async fn handle_feedback(&self, sensing_info: Self::Type,sensor_send: Sender<ReadingType>,feedback_recv: Receiver<ReadingType>, counts: Arc<AtomicI32>) {
        let now=Instant::now();
        let objects=Arc::new(Mutex::new(Vec::new()));
        let value=counts.load(Ordering::Acquire);
        let mut handler=vec![];
        for i in 0..value{
            let object_copy=Arc::clone(&objects);
            let feedback_recv_cloned=feedback_recv.clone();
            let handle=task::spawn(async move{
                let lock=object_copy.lock().await;
                TransmissionChannel::recieve_transmission_feedback(now.into(), lock, feedback_recv_cloned).await
            });
            handler.push(handle);
        }
        let mut arm_status=self.current_state;
        for handle in handler{
            if let Some(arm)=handle.await.unwrap(){
                arm_status=arm;
            }
        }
        match Arc::try_unwrap(objects){
            Ok(val)=>{
                let object=val.into_inner();
                let objects_lock=Arc::new(Mutex::new((arm_status, object)));
                self.collect_data(objects_lock, sensor_send, counts).await;
            },
            Err(err) =>{
                println!("objects cannot be unwrapped:{:?}", err);
            }
        }
    }

    // Collect data in the initial state
    async fn collect_data(&self, sensing_info: Self::Type, sensor_send: Sender<ReadingType>, counts: Arc<AtomicI32>) {
        let value=counts.load(Ordering::Acquire);
        let mut handler=vec![];
        for _ in 0..value{
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
        if let Some(value)= data.1.pop(){
                match timeout(Duration::from_micros(100),TransmissionChannel::transmit_data(ReadingType::RoboticArm(data.0, value.0, value.2),tx_copy, data)).await{
                    Ok(ok) => println!("Data is transmitted in the allocated time bound"),
                    Err(err)=>{
                        println!("Deadline is violated.. recovery mechanism is ON");
                    }
                }
        }else{
            println!("all values have been submitted")
        }
    }

}


