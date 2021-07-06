use anyhow::Error;
use log::error;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::crdb::TodoRepo;
use crate::proto::todo_api_server::TodoApi;
use crate::proto::{
    CreateTodoRequest, CreateTodoResponse, GetTodoRequest, GetTodoResponse, ListTodosRequest,
    ListTodosResponse,
};

#[derive(Debug)]
pub(crate) struct Service {
    repo: TodoRepo,
}

impl Service {
    pub fn new(repo: TodoRepo) -> Self {
        Service { repo }
    }
}

#[tonic::async_trait]
impl TodoApi for Service {
    async fn create_todo(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoResponse>, Status> {
        let CreateTodoRequest { title, description } = request.into_inner();
        self.repo
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

        self.repo
            .get(&uuid)
            .await
            .map(|todo| Response::new(GetTodoResponse { todo: Some(todo) }))
            .map_err(handle_error)
    }

    async fn list_todos(
        &self,
        _: Request<ListTodosRequest>,
    ) -> Result<Response<ListTodosResponse>, Status> {
        self.repo
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
