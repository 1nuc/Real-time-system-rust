use crate::{Initiation, Actions};

#[derive(Clone, Copy, Debug)]
pub struct Actual{ 
    pub force: f32,
    pub velocity: f32,
    pub position: f32,
    pub temperature: f32,
}
//define the implmnetation of the Action crate to the struct
impl Initiation for Actual{
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
impl Initiation for Target{
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

impl Actions for Readings{
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
}
