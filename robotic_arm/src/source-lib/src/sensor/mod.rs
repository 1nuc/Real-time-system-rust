use std::{sync::{mpsc::Sender, Arc, Mutex}, thread, time::Duration};

use crate::{transmission_control::TransmissionChannel, Actions, Control, Sensing};

#[derive(Clone, Copy)]
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
}
impl Sensing for Readings{

    fn assign_data(sample_data: i32 ) ->  Self{
        let mut arr= Vec::new();
        for _ in 0..sample_data{
            let index: i32= rand::random_range(0..=1);
            arr.push((Target::new(), Self::generate_keys(index)));
        }
        Self{
            objects:arr,
            current_state: Actual::new(),
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
        let filtered_objects=self.objects.clone().into_iter().filter(|x|{
            x.1==Self::TOKEN
        }).collect();
        Self{
            objects: filtered_objects,
            current_state: self.current_state,
        }
    }

    fn detect_noise() {

    }
    fn standardize_data() {
    }
    fn transmit_data(&self, sensor_send: &Sender<ReadingType>) {
        let current_state_thread=thread::Builder::new();
        let res=sensor_send.clone();
        let current_state_arm=self.current_state.clone();
        //sending current states to the actuator
        current_state_thread.name("Current State Thread".to_string()).spawn(move || {
            println!("Sending current arm state.."); 
            res.send(ReadingType::RoboticArm(current_state_arm)).unwrap(); 
            drop(res);
            thread::sleep(Duration::from_secs(1));
        }).expect("failed to spawn thread");
        let objects= Arc::new(Mutex::new(self.objects.clone()));
        for _ in 0..=self.objects.len(){
            let tx_copy=sensor_send.clone();
            let packets=Arc::clone(&objects);
            thread::spawn(move ||{
                let mut data=packets.lock().unwrap().clone();
                //checking if the value will be empty
                if let Some(sensor_packet)=data.pop(){
                    tx_copy.send(ReadingType::ObjectBoxes(sensor_packet.0)).unwrap();
                }
                else {
                    println!("No more Targets..Sensor is closing");
                }
                println!("Sending Target details...");
                drop(tx_copy);
                drop(data);
                thread::sleep(Duration::from_secs(1));
            });
        }
    }
}


