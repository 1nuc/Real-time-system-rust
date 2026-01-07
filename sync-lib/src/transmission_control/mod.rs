use std::{time::{Duration, Instant},sync::{
    Arc, Mutex,MutexGuard, atomic::{AtomicI32, Ordering}}, 
};
use manufacturer::{Actions,sensing_data::Readings, actuator_data::PID};
use crate::{ActuatorSync, Shared,Target, Actual, ControlSync, SensingSync, sensor::{ReadingType}};
use crossbeam::channel::*;
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
impl ControlSync for TransmissionChannel{
    fn init()-> Self{
        let (tx, rx) = unbounded::<ReadingType>();
        Self{
            txes: tx,
            rxes: rx,
        }
    } 

    fn transmit_data<'a>(data: ReadingType, sensor_send: Sender<ReadingType>, lock: Self::SharedLock<'a>, time: u64){
       match sensor_send.send_timeout(data, Duration::from_micros(time)){
          Ok(_)=>{
            println!("Sending Target details...");
            drop(lock);
          }, 
          Err(_) =>{
              "Deadline violated";
              return
          }
       } 
    }

    fn recieve_transmission(sensor_recv: Receiver<ReadingType>)->Option<ReadingType> 
{
            match sensor_recv.recv(){
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
    fn recieve_transmission_deadline(now: Instant, mut data:MutexGuard<Vec<(Target, String, i32)>>, feedback_recv: Receiver<ReadingType>) -> Option<Actual>{
        let token=<Readings as Actions>::TOKEN.to_string();
        match feedback_recv.recv_deadline(now + Duration::from_micros(500)){
            Ok(value) =>{
                let ReadingType::RoboticArm(arm, remaining_objects, id)=value;
                data.push((remaining_objects,token, id));
                drop(data);
                Some(arm)
            },
            Err(RecvTimeoutError)=>{
                 println!("deadline passed for thread..recovery mode is On");
                 None
            }
        }
    }

    fn simulation_control(self, boxes_num: i32){
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
            robotic_data.sensor_control(Arc::clone(&sensing_info),self.txes.clone(), feedback_channel.rxes.clone(), value_cloned);
            let value_cloned=Arc::clone(&value);
            PID::actuator_control(Arc::clone(&sensing_info),self.rxes.clone(), value_cloned, feedback_channel.txes.clone(), robotic_data_cloned);
        }
   }
}

