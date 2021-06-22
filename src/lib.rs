mod crdb;
mod proto;
mod service;
mod todo;

use crdb::TodoRepo;
use tonic::transport::Server;

use crate::proto::todo_api_server::TodoApiServer;
use crate::service::Service;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:7878".parse()?;
    let svc = Service::new(TodoRepo::new("localhost:26257")?);

    Server::builder()
        .add_service(TodoApiServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
