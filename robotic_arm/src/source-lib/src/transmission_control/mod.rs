use std::{time::{Duration, Instant},sync::{
    Arc, Mutex,MutexGuard}, thread,
};
use crate::{Actuator,Shared,Target, Actual, Control, Sensing, sensor::{ReadingType, Readings}};
use crossbeam::channel::*;
use advanced_pid::{Pid};

pub struct TransmissionChannel{
    pub txes: Sender<ReadingType>,
    pub rxes: Receiver<ReadingType>,
}

impl Shared for TransmissionChannel{
    type SharedLock<'a>=MutexGuard<'a, (Actual,Vec<(Target,String, i32)>)>;
    type Type= Arc<Mutex<(Actual,Vec<(Target,String, i32)>)>>;
}

impl Control for TransmissionChannel{
    fn init()-> Self{
        let (tx, rx) = unbounded::<ReadingType>();
        Self{
            txes: tx,
            rxes: rx,
        }
    } 

    fn transmit_data<'a>(data: ReadingType, sensor_send: Sender<ReadingType>, lock: Self::SharedLock<'a>){
       match sensor_send.send(data){
          Ok(_)=>{
            println!("Sending Target details...");
            drop(lock);
            thread::sleep(Duration::from_millis(100));
          }, 
          Err(_) =>{
              "error while sending, thread run into fail safe mode...";
              //TODO: Implementing fault tolerance mechanism Options:
              //1. Checkpoints
              //2. more concrete fail safe mode function 
          }
       } 
    }

    fn recieve_transmission(sensor_recv: Receiver<ReadingType>)->Option<ReadingType> 
{
            match sensor_recv.recv(){
                Ok(value) => {
                    println!("Readings recieved...");
                    Some(value)
                   // one issue detected is that the counts should increment only when the boxes are lifted not when they recieved, or the logic of the loop should change 
                },
                Err(e)=> {
                   println!("Error in recieving the data: {e}");
                   None 
                    //TODO: Some logic should be made to avoid channel collapse
                },
            }
    }
    fn recieve_transmission_deadline(now: Instant,object_lock:Arc<Mutex<Vec<(Target, String, i32)>>>, mut arm_status: Arc<Actual>,feedback_recv: Receiver<ReadingType>){
        let token=<Readings as Sensing>::TOKEN.to_string();
        match feedback_recv.recv_deadline(now + Duration::from_millis(500)){
            Ok(value) =>{
                let mut data=object_lock.lock().expect("cannot lock");
                let ReadingType::RoboticArm(arm, remaining_objects, id)=value;
                *Arc::make_mut(&mut arm_status)=arm.into();
                data.push((remaining_objects,token, id));
                drop(data);
            },
            Err(RecvTimeoutError)=> println!("Time out"),
        }
    }

    fn simulation_control(self){
        let robotic_data=Readings::assign_data(30).filter_noise();
        let sensing_info= Arc::new(Mutex::new((robotic_data.current_state, robotic_data.objects.clone())));
        println!("objects :{}", robotic_data.objects_num);
        let feedback_channel=Self::init();
        robotic_data.sensor_control(Arc::clone(&sensing_info),self.txes.clone(), feedback_channel.rxes);
        Pid::actuator_control(Arc::clone(&sensing_info),self.rxes, robotic_data.objects_num, feedback_channel.txes);
    }
}

