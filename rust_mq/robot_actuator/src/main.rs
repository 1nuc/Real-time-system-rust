use tokio::{*, time::sleep};
use futures_lite::stream::StreamExt;
use lapin::{types::FieldTable, *, options::*};
use serde_json;
mod actuator;
use manufacturer::{sensing_data::{Actual, Target}, *};

use std::time::Duration;
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
#[tokio::main]
async fn main() {
    let connection=create_connection().await;
    let channel=connection.create_channel().await.expect("error in creating a channel");
    let mut consumer= channel.basic_consume("sensing_data", "Actuator", BasicConsumeOptions::default(), FieldTable::default()).await;
    let queue=channel.queue_declare("sensing_data",QueueDeclareOptions::default(), FieldTable::default()).await.expect("unable to read from the queue");
    while consumer.is_err(){
         println!("Waiting for a message to recieve");
         consumer= channel.basic_consume("sensing_data", "consumer", BasicConsumeOptions::default(), FieldTable::default()).await;
         sleep(Duration::from_secs(2)).await;
    }

    while queue.message_count()!=0{
        let consumer_cloned=consumer.clone();
        task::spawn(async move {
            if let Some(msg)=consumer_cloned.unwrap().next().await{
                if let Ok(msg)=msg{
                    let actuator::ReadingType::RoboticArm(arm,object,id)=serde_json::from_slice::<actuator::ReadingType>(&(msg.data)).expect("Unable to serialize the data");
                    println!("Message recieved, Arm current position:{:?}, Objcet with ID:{:?}, stats:{:?}",arm, id, object);
                    msg.acker.ack(BasicAckOptions::default()).await;
                }
            }
        });
    }
    
    
}
