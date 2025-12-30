use std::sync::Arc;
use tokio::{*, time::{sleep, timeout}, sync::Mutex};
use futures_lite::stream::StreamExt;
use lapin::{types::FieldTable, *, options::*};
use serde_json;
mod actuator;
use manufacturer::{sensing_data:: *};
use std::time::Duration;

use crate::actuator::ReadingType;
async fn create_connection()-> Connection{
    let addr="amqp://guest:guest@localhost:5672";
    let mut res=Connection::connect(addr,ConnectionProperties::default()).await;
    while res.is_err(){
        println!("Failure in Connecting");
        res=Connection::connect(addr,ConnectionProperties::default()).await;
        sleep(Duration::from_secs(1)).await;
    }
    println!("Successful connection");
    let connection= res.unwrap();
    connection
    
}
async fn receive(data_vec: Vec<(Actual, Target, i32)>){
    let (arm, object, id)=find_smallest(data_vec.clone());
    let handle=task::spawn(async move {
        println!("Processing the Nearset Object with ID:{:?}", id );
        actuator::process_singals(data_vec, arm, object, id,).await;
    });
    handle.await.unwrap();
}

fn find_smallest(vector: Vec<(Actual, Target, i32)>)-> (Actual, Target, i32){
    let extracted_vals: Vec<_>=vector.clone().into_iter().map(|x|{
        let y=x.0.position.abs() -x.1.position.abs();
        (y, x.2)
    }).collect();
    let smallest_val=extracted_vals.into_iter().min_by(|a,b|a.0.partial_cmp(&b.0).unwrap()).unwrap();
    let mut filtered_vec: Vec<(Actual, Target, i32)>=vector.into_iter().filter(|x| x.2 ==smallest_val.1).collect();
    filtered_vec.pop().unwrap()
}
#[allow(unused_variables)]
#[tokio::main]
async fn main() {
    let connection=create_connection().await;
    let channel=connection.create_channel().await.expect("error in creating a channel");
    let queue_options=QueueDeclareOptions{
        passive: false,
        ..QueueDeclareOptions::default()
    };
    let _=channel.queue_delete("sensing_data", QueueDeleteOptions::default()).await.expect("unable to delete the queue");
    let queue=channel.queue_declare("sensing_data",queue_options, FieldTable::default()).await.expect("unable to read from the queue");
    let mut consumer= channel.basic_consume("sensing_data", "Actuator", BasicConsumeOptions::default(), FieldTable::default()).await;
    while consumer.is_err(){
         println!("Waiting for a message to recieve");
         consumer= channel.basic_consume("sensing_data", "consumer", BasicConsumeOptions::default(), FieldTable::default()).await;
         sleep(Duration::from_secs(2)).await;
    }
    let mut data_vec=vec![];
    loop{
        match timeout(Duration::from_secs(2), consumer.clone().expect("Error retreiving the data").next()).await{
            Ok(Some(msg))=>{
                if let Ok(msg)=msg{
                    let actuator::ReadingType::RoboticArm(arm,object,id)=serde_json::from_slice::<actuator::ReadingType>(&(msg.data)).expect("Unable to serialize the data");
                    data_vec.push((arm, object, id));
                    println!("Message recieved, Arm current position:{:?}, Objcet with ID:{:?}, stats:{:?}",arm, id, object);
                    let _=msg.acker.ack(BasicAckOptions::default()).await;
                }
            },
            Ok(None) =>{
                println!("messages have been received");
                break;
            },
            Err(_)=>{
                println!("Timeout");
                break;
            },


        }
    }
    receive(data_vec).await;
// there should be a function to calculate the nearset position
// this function should receive everything all at once pick the nearset object from the arm hold
// it and send back the remaining objcets
// requirements-> function to calculat the distance
// function to delete from the objects based on the id and return the remaining objects
// don't forget to clean the code after you are done with the logic
}
