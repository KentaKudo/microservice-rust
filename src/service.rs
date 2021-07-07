use anyhow::Error;
use log::error;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::proto::todo_api_server::TodoApi;
use crate::proto::{
    CreateTodoRequest, CreateTodoResponse, GetTodoRequest, GetTodoResponse, ListTodosRequest,
    ListTodosResponse,
};
use crate::todo::TodoService;

#[derive(Debug)]
pub(crate) struct Service<T> {
    todo: T,
}

impl<T> Service<T> {
    pub fn new(todo: T) -> Self {
        Service { todo }
    }
}

#[tonic::async_trait]
impl<T: TodoService + Send + Sync + 'static> TodoApi for Service<T> {
    async fn create_todo(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoResponse>, Status> {
        let CreateTodoRequest { title, description } = request.into_inner();
        self.todo
            .create(&title, &description)
            .await
            .map(|id| Response::new(CreateTodoResponse { id: id.to_string() }))
            .map_err(handle_error)
    }

    async fn get_todo(
        &self,
        request: Request<GetTodoRequest>,
    ) -> Result<Response<GetTodoResponse>, Status> {
        let GetTodoRequest { id } = request.into_inner();
        let uuid =
            Uuid::parse_str(&id).map_err(|_| Status::invalid_argument("invalid id format"))?;

        self.todo
            .get(&uuid)
            .await
            .map(|todo| Response::new(GetTodoResponse { todo: Some(todo) }))
            .map_err(handle_error)
    }

    async fn list_todos(
        &self,
        _: Request<ListTodosRequest>,
    ) -> Result<Response<ListTodosResponse>, Status> {
        self.todo
            .list()
            .await
            .map(|todos| Response::new(ListTodosResponse { todos }))
            .map_err(handle_error)
    }
}

fn handle_error(e: Error) -> Status {
    error!("{}", e);
    Status::internal("")
}
