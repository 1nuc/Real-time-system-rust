use tokio::*;
use manufacturer::{sensing_data::{Actual, Target}, *};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ReadingType{
    RoboticArm(Actual,Target, i32),
} 

async fn prepare_data(){
    //assign 50 boxes of data
    let robotic_data=manufacturer::sensing_data::Readings::assign_data(50);
}
