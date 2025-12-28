use tokio::{*, time::sleep};
use futures_lite::stream::StreamExt;
use lapin::{types::FieldTable, *, options::*};
use serde_json;
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
    while consumer.is_err(){
         println!("Waiting for a message to recieve");
         consumer= channel.basic_consume("sensing_data", "consumer", BasicConsumeOptions::default(), FieldTable::default()).await;
         sleep(Duration::from_secs(2)).await;
    }
    while let Some(msg)=consumer.clone().unwrap().next().await{
        if let Ok(msg)=msg{
            println!("income data: {:?}", msg.data);
            let data=serde_json::from_slice::<(Actual, Vec<(Target,String,i32)>)>(&(msg.data)).expect("Unable to serialize the data");
            println!("Message recieved, {:?}", data);
            msg.acker.ack(BasicAckOptions::default()).await;
        }
    }
    
}
