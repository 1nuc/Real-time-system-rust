use std::{sync::{Arc, Mutex}, thread, time::Duration};

use crate::{transmission_control::TransmissionChannel, Actions, Control, Sensing};

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
    fn transmit_data(&self) {
        let control=TransmissionChannel::init();
        let current_state_thread=thread::Builder::new();
        let res=control.clone();
        //sending current states to the actuator
        current_state_thread.name("Current State Thread".to_string()).spawn(move || {
            println!("Sending current arm state.."); 
            res.send(self.current_state).unwrap(); 
            thread::sleep(Duration::from_secs(1));
        }).expect("failed to spawn thread");
        drop(res);
        let objects= Arc::new(Mutex::new(self.objects.clone()));
        for _ in 0..=self.objects.len(){
            let tx_copy=control.clone();
            let packets=Arc::clone(&objects);
            thread::spawn(move ||{
                let data=packets.lock().unwrap().clone();
                if let Some(sensor_packet)=data.pop(){
                    tx_copy.send(sensor_packet.0).unwrap();
                }
                else {
                    println!("No more Targets..Sensor is closing");
                    drop(data);
                }
                println!("Sending Target details...");
                drop(data);
                thread::sleep(Duration::from_secs(1));
            });
            drop(tx_copy);
        }
    }
}


