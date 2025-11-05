use crate::{Actions, Sensing};

#[derive(Clone, Copy)]
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
            temperature: rand::random::<f32>(),
        }
    }
} 

#[derive(Clone, Copy)]
pub struct Target{ 
    pub force: f32,
    pub velocity: f32,
    pub position: f32,
    pub temperature: f32,
}

impl Actions for Target{

    fn new() -> Self{
        Self{
            force: rand::random::<f32>(), 
            velocity: rand::random::<f32>(),
            position: rand::random::<f32>(),
            temperature: rand::random::<f32>(),
        }
    }
} 

pub struct Readings <'a>{
    pub objects: Vec<(Target,&'a str)>,
    pub current_state: Actual,
}

impl<'a> Sensing <'a> for Readings <'a>{

    fn assign_data(sample_data: i32 ) ->  Self{
        let mut arr= Vec::new();
        let charsets=random_string::charsets::ALPHANUMERIC;
        for _ in 0..sample_data{
            let defaulted_key_owned=random_string::generate_rng(0..40, &charsets);
            let index: i32= rand::random_range(0..=1);
            arr.push((Target::new(), Self::generate_keys(index, &defaulted_key_owned)));
        }
        Self{
            objects:arr,
            current_state: Actual::new(),
        }

    }
    fn generate_keys(index: i32, defaulted_key_owned: &'a String) -> &'a str{
        let defaulted_key: &'a str= defaulted_key_owned.as_str();
        match index{
            0 => defaulted_key,
            1 => Self::TOKEN,
            _=>"402ERROR",
        }
    }

    fn explore(&self) {
        println!()
    }

    fn detect_noise() {

    }
    fn standardize_data() {

    }
}

