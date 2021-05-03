#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    microservice_rust::run().await
}
