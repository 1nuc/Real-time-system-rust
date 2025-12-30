use tokio::{*, time::{sleep, timeout}};
use futures_lite::stream::StreamExt;
use lapin::{types::FieldTable, *, options::*};
use serde_json;
mod actuator;
use manufacturer::{sensing_data:: *};
use std::time::Duration;


#[allow(unused_variables)]
#[allow(non_snake_case)]
#[tokio::main]
async fn main() {
    let connection =actuator::create_connection().await;
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
    actuator::receive(data_vec, connection).await;
// there should be a function to calculate the nearset position
// this function should receive everything all at once pick the nearset object from the arm hold
// it and send back the remaining objcets
// requirements-> function to calculat the distance
// function to delete from the objects based on the id and return the remaining objects
// don't forget to clean the code after you are done with the logic
}
