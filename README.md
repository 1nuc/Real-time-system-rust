# Robotic Actuator Real Time System using rust both async and sync mechanisms

## Async version
- the project have been built with tokio for asynchrounous communication between components.

## Sync version
- The project is designed to enable efficient synchronization through effective handling of shared data.

## Rabbit MQ version
- Actual real time communication between the client and the server with efficient handling of deadlines and feedbacks. 

## Production Steps
### Requirements: Download Docker-compose
`sudo apt/dnf install docker-compose`
`cd rust_mq`
`docker compose up -d`
### Running benches
`cd Main`
`cargo b`
`cargo bench`
