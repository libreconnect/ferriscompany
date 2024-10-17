use async_nats::{connect, Client};
use std::sync::Arc;

use crate::application::ports::messaging_ports::MessagingPort;

pub struct Nats {
    client: Arc<Client>,
}

impl Nats {
    pub async fn new(addrs: &str) -> anyhow::Result<Nats> {
        let client = connect(addrs).await?;

        Ok(Nats {
            client: Arc::new(client),
        })
    }

    pub fn get_client(&self) -> Arc<Client> {
        Arc::clone(&self.client)
    }
}

#[derive(Clone)]
pub struct NatsMessaging {
    connection: Arc<Client>,
}

impl NatsMessaging {
    pub async fn new(addrs: &str) -> anyhow::Result<NatsMessaging> {
        let client = connect(addrs).await?;

        Ok(NatsMessaging {
            connection: Arc::new(client),
        })
    }

    pub fn get_connection(&self) -> Arc<Client> {
        Arc::clone(&self.connection)
    }
}

impl MessagingPort for NatsMessaging {
    async fn publish_message(&self, topic: String, message: String) -> anyhow::Result<()> {
        let conn = self.get_connection();

        conn.publish(topic, message.into())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .map(|_| ())
    }
}
