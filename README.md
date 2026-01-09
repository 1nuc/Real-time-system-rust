# Robotic Actuator Real Time System using rust both async and sync mechanisms

## Async version
- the project have been built with tokio for asynchrounous communication between components.

## Sync version
- The project is designed to enable efficient synchronization through effective handling of shared data.

## Rabbit MQ version
- Actual real time communication between the client and the server with efficient handling of deadlines and feedbacks. 

## Production Steps
### Requirements: Download Docker-compose
`sudo apt/dnf install docker-compose`<br>
`cd rust_mq`<br>
`docker compose up -d`<br>
### Running benches
`cd Main`<br>
`cargo b`<br>
`cargo bench`<br>
