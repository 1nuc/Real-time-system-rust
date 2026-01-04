use std::{sync::{
    Arc,atomic::{AtomicI32, Ordering}}, time::{Instant} 
};
use tokio::{sync::{Mutex, MutexGuard}};
use manufacturer::{Actions,sensing_data::Readings, actuator_data::PID};
use crate::{Actuator, Shared,Target, Actual, Control, Sensing, sensor::{ReadingType}};
use flume::*;
pub struct TransmissionChannel{
    pub txes: Sender<ReadingType>,
    pub rxes: Receiver<ReadingType>,
}

impl Shared for TransmissionChannel{
    type SharedLock<'a>=MutexGuard<'a, (Actual,Vec<(Target,String, i32)>)>;
    type Type= Arc<Mutex<(Actual,Vec<(Target,String, i32)>)>>;
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
impl Control for TransmissionChannel{
    fn init()-> Self{
        let (tx, rx) = unbounded::<ReadingType>();
        Self{
            txes: tx,
            rxes: rx,
        }
    } 

    async fn transmit_data<'a>(data: ReadingType, sensor_send: Sender<ReadingType>, lock: Self::SharedLock<'a>){
       match sensor_send.send_async(data).await{
          Ok(_)=>{
            println!("Sending Target details...");
            drop(lock);
          }, 
          Err(_) =>{
              "error while sending, thread run into fail safe mode...";
              return;
          }
       } 
    }

    async fn recieve_transmission(sensor_recv: Receiver<ReadingType>)->Option<ReadingType> 
{
            match sensor_recv.recv_async().await{
                Ok(value) => {
                    println!("Readings recieved...");
                    Some(value)
                },
                Err(e)=> {
                   println!("Error in recieving the data: {e}");
                   None 
                },
            }
    }
    async fn recieve_transmission_deadline<'a>(now: Instant, mut data:MutexGuard<'a,Vec<(Target, String, i32)>>, mut arm_status: Arc<Actual>,feedback_recv: Receiver<ReadingType>){
        let token=<Readings as Actions>::TOKEN.to_string();
        // match feedback_recv.recv_deadline(now + Duration::from_millis(500)){
        match feedback_recv.recv_async().await{
            Ok(value) =>{
                let ReadingType::RoboticArm(arm, remaining_objects, id)=value;
                *Arc::make_mut(&mut arm_status)=arm.into();
                data.push((remaining_objects,token, id));
                drop(data);
            },
            Err(e)=> {
                println!("Error In receiving, {:?}", e);
            }
        }
        drop(feedback_recv);
    }

    async fn simulation_control(self, boxes_num: i32){
        let robotic_data=Readings::assign_data(boxes_num).filter_noise();
        let sensing_info= Arc::new(Mutex::new((robotic_data.current_state, robotic_data.objects.clone())));
        println!("objects :{}", robotic_data.objects_num);
        let feedback_channel=Self::init();
        let value=Arc::new(AtomicI32::new(robotic_data.objects_num));
        loop{
            let value_cloned=Arc::clone(&value);
            if value_cloned.load(Ordering::Acquire)==0{
                println!("All Boxes have been lifted");
                break;
            } 
            let robotic_data_cloned=robotic_data.clone();
            robotic_data.sensor_control(Arc::clone(&sensing_info),self.txes.clone(), feedback_channel.rxes.clone(), value_cloned).await;
            let value_cloned=Arc::clone(&value);
            PID::actuator_control(Arc::clone(&sensing_info),self.rxes.clone(), value_cloned, feedback_channel.txes.clone(), robotic_data_cloned).await;
        }
   }
}

