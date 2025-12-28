use tokio::*;
use manufacturer::{sensing_data::{Actual, Target}, actuator_data::PID};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ReadingType{
    RoboticArm(Actual, Target, i32),
} 

// pub async fn process_singals(lock: Self::SharedLock<'a>, mut current_arm_status: Actual, mut object_status: Target,
//     id: i32, feedback_send: Sender<ReadingType>, robotic_data: Readings, counts: Arc<AtomicI32>){
// //TODO: processing Position
//     let position=task::spawn(async move{
//         PID::calculate_pid(&mut current_arm_status.position,&mut object_status.position, "Position")
//     }).await.unwrap();
// //TODO: processing Temparture
//     let temparture=task::spawn(async move{
//         PID::calculate_pid(&mut current_arm_status.temperature,&mut object_status.temperature, "Temprature")
//     }).await.unwrap();
// // //TODO: processing Force
//     let force=task::spawn(async move{
//         PID::calculate_pid(&mut current_arm_status.force,&mut object_status.force,  "Force")
//     }).await.unwrap();
// //TODO: processing Velocity
//     let velocity=task::spawn(async move{
//         PID::calculate_pid(&mut current_arm_status.velocity,&mut object_status.velocity,  "Velocity")
//     }).await.unwrap();
//
//     Self::process_feedback(position, temparture, force, velocity, id, robotic_data,feedback_send, counts).await; 
//
//     drop(lock);
// }
//
// async fn process_feedback(pos: f32, temparture: f32, force: f32, vel: f32, id_deleted: i32,
//     mut robotic_data: Readings, feedback_send: Sender<ReadingType>, counts: Arc<AtomicI32>) {
//    let updated_arm_status=Initiation::init(temparture, force, vel, pos);
//    println!("Object with ID: {:?} is lifted", id_deleted);
//    println!("Updated Arm stats: {:?}", updated_arm_status);
//    let updated_readings= robotic_data.update_indices(id_deleted, updated_arm_status);
//    let sensing_info= Arc::new(Mutex::new((updated_readings.current_state, updated_readings.objects.clone())));
//    counts.fetch_sub(1, Ordering::Release);
//    let value= counts.load(Ordering::Acquire);
//    if value ==0{
//         return;
//    }
//    println!("remaining objects: {:?}", counts);
//    robotic_data.collect_data(sensing_info, feedback_send, counts).await;
// }
