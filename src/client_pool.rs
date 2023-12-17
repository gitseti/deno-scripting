use leaky_bucket::RateLimiter;
use paho_mqtt::{
    AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, Message, MessageBuilder,
};
use rand::prelude::ThreadRng;
use rand::Rng;
use std::time::Duration;

pub struct ClientPool {
    clients: Vec<AsyncClient>,
}

impl ClientPool {
    pub fn new(client_amount: usize) -> Self {
        let mut clients = Vec::with_capacity(client_amount);
        for i in 0..client_amount {
            let create_options = CreateOptionsBuilder::new()
                .client_id(format!("client_pool_client-{}", i))
                .server_uri("broker.hivemq.com")
                .finalize();
            let client = AsyncClient::new(create_options).unwrap();

            client.set_message_callback(|client, message| match message {
                None => {}
                Some(message) => {
                    println!(
                        "Client '{}' received message: {}",
                        client.client_id(),
                        message.to_string()
                    )
                }
            });

            clients.push(client);
        }

        ClientPool { clients }
    }

    pub async fn connect(&self, connects_per_second: usize) {
        let rate_limiter = RateLimiter::builder()
            .initial(connects_per_second)
            .refill(connects_per_second)
            .interval(Duration::from_secs(1))
            .build();

        let mut connect_tokens = Vec::with_capacity(self.clients.len());

        for client in &self.clients {
            rate_limiter.acquire_one().await;
            let connect_options = ConnectOptionsBuilder::new().finalize();
            let token = client.connect(connect_options);
            connect_tokens.push(token);
        }

        for (i, connect_token) in connect_tokens.iter().enumerate() {
            let response = connect_token.clone().wait();
            match response {
                Ok(response) => {
                    if response.reason_code().is_err() {
                        println!(
                            "Failed to connect client {}: {}.",
                            i,
                            response.reason_code().to_string()
                        )
                    } else {
                        println!("Connected client {}.", i)
                    }
                }
                Err(err) => {
                    println!("Failed to connect client {}: {}.", i, err.to_string())
                }
            }
        }
    }

    pub fn publish_rand(&self, message: Message) {
        let index = rand::thread_rng().gen_range(0..self.clients.len());
        let client = &self.clients[index];
        client.publish(message).wait().unwrap()
    }

    pub fn subscribe(&self, topic: String) {
        let mut tokens = Vec::with_capacity(self.clients.len());
        for client in &self.clients {
            let token = client.subscribe(topic.clone(), 0);
            tokens.push(token);
        }

        for token in tokens {
            let result = token.wait();
            match result {
                Ok(reponse) => {
                    println!("Subscribed!")
                }
                Err(err) => {
                    println!("Failed to subscribe")
                }
            }
        }
    }
}
