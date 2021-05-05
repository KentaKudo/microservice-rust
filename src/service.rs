use tonic::{Request, Response, Status};

use crate::proto::todo_api_server::TodoApi;
use crate::proto::{
    CreateTodoRequest, CreateTodoResponse, GetTodoRequest, GetTodoResponse, ListTodosRequest,
    ListTodosResponse,
};

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
