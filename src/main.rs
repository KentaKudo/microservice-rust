use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    microservice_rust::run().await
}
