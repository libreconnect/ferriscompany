use std::sync::Arc;

use async_nats::{connect, Client};

pub struct Nats {
  client: Arc<Client>
}

impl Nats {
  pub async fn new(addrs: &str) -> anyhow::Result<Nats> {
    let client = connect(addrs).await?;

    Ok(Nats {
      client: Arc::new(client)
    })
  }

  pub fn get_client(&self) -> Arc<Client> {
    Arc::clone(&self.client)
  }
}