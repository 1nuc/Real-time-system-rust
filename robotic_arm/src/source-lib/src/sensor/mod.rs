use std::{sync::{mpsc::Sender, Arc, Mutex}, thread::{self}, time::Duration};

use crate::{Actions, Sensing};

#[derive(Clone, Copy, Debug)]
pub enum ReadingType{
    RoboticArm(Actual),
    ObjectBoxes(Target),
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

    fn detect_noise() {

    }
    fn standardize_data() {
    }
    fn transmit_data(&self, sensor_send: Sender<ReadingType>) {
        let current_state=self.current_state;
        let res=sensor_send.clone();
        //sending current states to the actuator
        thread::Builder::new().name("Current State Thread".to_string()).spawn(move || {
            println!("Sending current arm state.."); 
            res.send(ReadingType::RoboticArm(current_state)).unwrap(); 
            drop(res);
            thread::sleep(Duration::from_secs(1));
        }).expect("failed to spawn thread");

        let objects= Arc::new(Mutex::new(self.objects.clone()));
        for _ in 0..=self.objects.len(){
            let tx_copy=sensor_send.clone();
            let packets=Arc::clone(&objects);
            thread::spawn(move ||{
                let mut data=packets.lock().expect("error while locking");
                //checking if the value will be empty
                match data.pop(){
                    Some(value) =>{
                        match tx_copy.send(ReadingType::ObjectBoxes(value.0)){
                          Ok(_)=>{
                            println!("Sending Target details...");
                            drop(data);
                            thread::sleep(Duration::from_millis(1));
                          }, 
                          Err(_) =>{
                              println!("error while sending, thread run into fail safe mode...");
                          }
                        }
                    },
                    None =>{
                        drop(tx_copy);
                        drop(data);
                        println!("No more Targets..Sensor is closing");
                    },
                }
            });
        }
        println!("All data has been sent");
    }
}


