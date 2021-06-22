mod crdb;
mod proto;
mod service;

use anyhow::{Context, Result};
use tonic::transport::Server;

use crate::crdb::TodoRepo;
use crate::proto::todo_api_server::TodoApiServer;
use crate::service::Service;

pub async fn run() -> Result<()> {
    let addr = "[::1]:7878";
    let addr = addr.parse().context(format!("{}.parse()", addr))?;
    let dsn = "postgresql://root@localhost:26257/test";
    let repo = TodoRepo::new(dsn)
        .await
        .context(format!("TodoRepo::new({})", dsn))?;

    Server::builder()
        .add_service(TodoApiServer::new(Service::new(repo)))
        .serve(addr)
        .await?;

    Ok(())
}
