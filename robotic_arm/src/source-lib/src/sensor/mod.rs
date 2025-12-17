use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread, time::{Duration, Instant}};
use crossbeam::{channel::*};

use crate::{Actions, Control, Sensing, Shared, transmission_control::{self, TransmissionChannel}};

#[derive(Clone, Copy, Debug)]
pub enum ReadingType{
    RoboticArm(Actual, Target, i32),
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
    fn new() -> Self where Self: Sized{
        Self {
            force: rand::random::<f32>(), 
            velocity: rand::random::<f32>(),
            position: rand::random::<f32>(),
            temperature: rand::random_range(0.0..=100.0),
        }
    }
    fn init(temp: f32, force: f32, vel: f32, pos: f32)->Self where Self: Sized{
        Self{
            force: force,
            velocity: vel,
            position: pos,
            temperature: temp,
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
    fn new() -> Self where Self: Sized{
        Self {
            force: rand::random::<f32>(), 
            velocity: rand::random::<f32>(),
            position: rand::random::<f32>(),
            temperature: rand::random_range(0.0..=100.0),
        }
    }
    fn init(temp: f32, force: f32, vel: f32, pos: f32)->Self where Self: Sized{
        Self{
            force: force,
            velocity: vel,
            position: pos,
            temperature: temp,
        }
    }
} 
#[derive(Debug, Clone)]
pub struct Readings {
    pub objects: Vec<(Target,String,i32)>, //each object contains the required info to be lifted 
    pub current_state: Actual, //as well as the token and ID
    pub objects_num: i32,
}
impl Shared for Readings{
    type SharedLock<'a>=MutexGuard<'a, (Actual,Vec<(Target,String, i32)>)>;
    type Type= Arc<Mutex<(Actual,Vec<(Target,String, i32)>)>>;
}
impl Sensing for Readings{
    fn assign_data(sample_data: i32 ) ->  Self{
        let mut arr= Vec::new();
        let mut count:i32=0;
        for i in 0..sample_data{
            let index: i32= rand::random_range(0..=1);
            arr.push((Target::new(), Self::generate_keys(index), i));
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
        let filtered_objects: Vec<(Target, String, i32)>=self.objects.clone().into_iter().filter(|x|{
            x.1==Self::TOKEN
        }).collect();
        let up_count: i32=filtered_objects.len().try_into().unwrap();
        Self{
            objects: filtered_objects,
            current_state: self.current_state,
            objects_num: up_count,
        }
    }

    fn update_indices(&mut self, id: i32, new_current_state: Actual)->Self {
        let updated_data=self.objects.clone().into_iter().filter(|x| x.2 !=id).collect();
        self.objects_num-=1;
        Self{
            objects: updated_data,
            current_state:new_current_state,
            objects_num: self.objects_num,
        }
    }

    fn sensor_control(&self, sensing_info: Self::Type,sensor_send: Sender<ReadingType>, feedback_recv: Receiver<ReadingType>) {
        if feedback_recv.is_empty(){
            self.collect_data(sensing_info ,sensor_send);
        }
        else{
            let now=Instant::now();
            let mut objects=Arc::new(Mutex::new(Vec::new()));
            let mut arm_status=Arc::new(self.current_state);
            while feedback_recv.is_full(){
                let object_copy=Arc::clone(&mut objects);
                let feedback_recv_cloned=feedback_recv.clone();
                let arm_status_cloned=Arc::clone(&mut arm_status);
                thread::spawn(move||{
                    TransmissionChannel::recieve_transmission_deadline(now, object_copy, arm_status_cloned, feedback_recv_cloned);
                });
            }
            match Arc::try_unwrap(objects){
                Ok(val)=>{
                    let object=val.into_inner().unwrap();
                    let arm_unwrapped=Arc::try_unwrap(arm_status).unwrap();
                    let objects_lock=Arc::new(Mutex::new((arm_unwrapped, object)));
                    self.collect_data(objects_lock, sensor_send);
                },
                Err(err)=>println!("values cannot be unwrapped: {:?}", err),
            }
        }
    }

    // Collect data in the initial state
    fn collect_data(&self, sensing_info: Self::Type, sensor_send: Sender<ReadingType>) {
        for _ in 0..=self.objects.len(){
            let tx_copy=sensor_send.clone();
            let packets=Arc::clone(&sensing_info);
            thread::spawn(move ||{
                Self::sensor_workflow(packets, tx_copy);
            });
        }
        drop(sensor_send);
    }
    
    //start threads to send the data through packets
    fn sensor_workflow(packets: Self::Type, tx_copy: Sender<ReadingType>){
        let mut data=packets.lock().expect("error while locking");
        match data.1.pop(){
            Some(value) =>{
                TransmissionChannel::transmit_data(ReadingType::RoboticArm(data.0, value.0, value.2), tx_copy, data);
            },
            None =>{
                drop(tx_copy);
                drop(data);
                println!("No more Targets..Sensor is closing");
            },
        }
    }

    //send the data through locks
    // The Aim of this infrustrcture is to easliy track the errors in the code 
}


