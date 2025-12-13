use std::{sync::{mpsc::{Sender, Receiver}, Arc, Mutex}, thread::{self}, time::Duration, result::Result};

use crate::{Actions, Sensing, Shared};

#[derive(Clone, Copy, Debug)]
pub enum ReadingType{
    RoboticArm(Actual, Target),
} 
#[derive(Clone, Copy, Debug)]
pub struct Actual{ 
    pub force: f32,
    pub velocity: f32,
    pub position: f32,
    pub temperature: f32,
}
//define the implmnetation of the Action crate to the struct
impl Actions for Actual{

    fn new() -> Self{
        Self {
            force: rand::random::<f32>(), 
            velocity: rand::random::<f32>(),
            position: rand::random::<f32>(),
            temperature: rand::random_range(0.0..=100.0),
        }
    }
} 

#[derive(Clone, Copy, Debug)]
pub struct Target{ 
    pub force: f32,
    pub velocity: f32,
    pub position: f32,
    pub temperature: f32,
}
//define implementation for the target
impl Actions for Target{

    fn new() -> Self{
        Self{
            force: rand::random::<f32>(), 
            velocity: rand::random::<f32>(),
            position: rand::random::<f32>(),
            temperature: rand::random_range(0.0..=100.0),
        }
    }
} 
#[derive(Debug, Clone)]
pub struct Readings {
    pub objects: Vec<(Target,String)>,
    pub current_state: Actual,
    pub objects_num: i32,
}
impl Shared for Readings{
    type SharedLock= Arc<Mutex<(Actual,Vec<(Target,String)>)>>;
}
impl Sensing for Readings{
    fn assign_data(sample_data: i32 ) ->  Self{
        let mut arr= Vec::new();
        let mut count:i32=0;
        for _ in 0..sample_data{
            let index: i32= rand::random_range(0..=1);
            arr.push((Target::new(), Self::generate_keys(index)));
            count+=1;
        }
        Self{
            objects:arr,
            current_state: Actual::new(),
            objects_num: count,
        }

    }
    fn generate_keys(index: i32) -> String{
        let charsets=random_string::charsets::ALPHANUMERIC;
        let defaulted_key=random_string::generate_rng(0..40, charsets);
        match index{
            0 => defaulted_key,
            1 => String::from(Self::TOKEN),
            _=>"402ERROR".to_string(),
        }
    }

    fn explore(&self) {
       let header= "=".repeat(30); 
       let title="Robotic Arm Picker Readings";
       println!("{} \n{},\n{}\n current_state:{:#?},\n Target Boxes:{:#?}", header, title,header,self.current_state,self.objects);
    }

    fn filter_noise(&self)-> Self {
        let filtered_objects: Vec<(Target, String)>=self.objects.clone().into_iter().filter(|x|{
            x.1==Self::TOKEN
        }).collect();
        let up_count: i32=filtered_objects.len().try_into().unwrap();
        Self{
            objects: filtered_objects,
            current_state: self.current_state,
            objects_num: up_count,
        }
    }

    fn sensor_control(&self, sensing_info: Self::SharedLock,sensor_send: Sender<ReadingType>, feedback_recv: Receiver<ReadingType>) {
        self.collect_data(sensing_info ,sensor_send);
    }

    fn collect_data(&self, sensing_info: Self::SharedLock, sensor_send: Sender<ReadingType>) {
        for _ in 0..=self.objects.len(){
            let tx_copy=sensor_send.clone();
            let packets=Arc::clone(&sensing_info);
            thread::spawn(move ||{
                let mut data=packets.lock().expect("error while locking");
                match data.1.pop(){
                    Some(value) =>{
                        Self::transmit_data(ReadingType::RoboticArm(data.0, value.0), tx_copy);
                    },
                    None =>{
                        drop(tx_copy);
                        drop(data);
                        println!("No more Targets..Sensor is closing");
                    },
                }
            });
        }
        drop(sensor_send);
        println!("All data has been sent");
    }
    fn transmit_data(data: ReadingType, sensor_send: Sender<ReadingType>){
       match sensor_send.send(data){
          Ok(_)=>{
            println!("Sending Target details...");
            drop(data);
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
}


