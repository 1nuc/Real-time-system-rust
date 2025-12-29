use tokio::*;
use manufacturer::{PIDSetup, actuator_data::PID, sensing_data::{Actual, Target}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ReadingType{
    RoboticArm(Actual, Target, i32),
} 
type Lock<'a>=tokio::sync::MutexGuard<'a, Vec<(Actual, Target, i32)>>;

// //TODO: Add another function to update the indices without the need for the object data to be
// //available
pub async fn process_singals<'a>(robotic_data: Lock<'a>,mut current_arm_status: Actual, mut object_status: Target,
    id: i32){
//TODO: processing Position
    let position=task::spawn(async move{
        PID::calculate_pid(&mut current_arm_status.position,&mut object_status.position, "Position")
    }).await.unwrap();
//TODO: processing Temparture
    let temparture=task::spawn(async move{
        PID::calculate_pid(&mut current_arm_status.temperature,&mut object_status.temperature, "Temprature")
    }).await.unwrap();
// //TODO: processing Force
    let force=task::spawn(async move{
        PID::calculate_pid(&mut current_arm_status.force,&mut object_status.force,  "Force")
    }).await.unwrap();
//TODO: processing Velocity
    let velocity=task::spawn(async move{
        PID::calculate_pid(&mut current_arm_status.velocity,&mut object_status.velocity,  "Velocity")
    }).await.unwrap();

    process_feedback(position, temparture, force, velocity, id, robotic_data).await; 
}

async fn process_feedback<'a>(pos: f32, temparture: f32, force: f32, vel: f32, id_deleted: i32,
    robotic_data: Lock<'a>) {
   // let updated_arm_status=manufacturer::Initiation::init(temparture, force, vel, pos);
   // println!("Object with ID: {:?} is lifted", id_deleted);
   // println!("Updated Arm stats: {:?}", updated_arm_status);
   println!("robot data number: {:?}",robotic_data.len());
   drop(robotic_data);
   // let updated_readings= robotic_data.update_indices(id_deleted, updated_arm_status);
   // let sensing_info= Arc::new(Mutex::new((updated_readings.current_state, updated_readings.objects.clone())));
   // counts.fetch_sub(1, Ordering::Release);
   // let value= counts.load(Ordering::Acquire);
   // if value ==0{
   //      return;
   // }
   // println!("remaining objects: {:?}", counts);
   // robotic_data.collect_data(sensing_info, feedback_send, counts).await;
}
