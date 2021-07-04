use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    microservice_rust::run().await
}
