use tonic::{transport::Server, Request, Response, Status};

use service::todo_api_server::{TodoApi, TodoApiServer};
use service::{
    CreateTodoRequest, CreateTodoResponse, GetTodoRequest, GetTodoResponse, ListTodosRequest,
    ListTodosResponse,
};

pub mod service {
    tonic::include_proto!("service");
}

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl TodoApi for Service {
    async fn create_todo(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoResponse>, Status> {
        Ok(Response::new(CreateTodoResponse::default()))
    }

    async fn get_todo(
        &self,
        request: Request<GetTodoRequest>,
    ) -> Result<Response<GetTodoResponse>, Status> {
        Ok(Response::new(GetTodoResponse::default()))
    }

    async fn list_todos(
        &self,
        request: Request<ListTodosRequest>,
    ) -> Result<Response<ListTodosResponse>, Status> {
        Ok(Response::new(ListTodosResponse::default()))
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:7878".parse()?;
    let svc = Service::default();

    Server::builder()
        .add_service(TodoApiServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
