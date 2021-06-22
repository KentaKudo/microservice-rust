use tonic::{Request, Response, Status};

use crate::proto::todo_api_server::TodoApi;
use crate::proto::{
    CreateTodoRequest, CreateTodoResponse, GetTodoRequest, GetTodoResponse, ListTodosRequest,
    ListTodosResponse,
};
use crate::todo::Service as TodoService;

#[derive(Debug)]
pub(crate) struct Service<T: TodoService + 'static> {
    todo: T,
}

impl<T: TodoService> Service<T> {
   pub fn new(todo: T) -> Self {
       Service { todo }
   }
}

#[tonic::async_trait]
impl<T: TodoService + 'static> TodoApi for Service<T> {
    async fn create_todo(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<CreateTodoResponse>, Status> {
        let todoReq = request.get_ref();
        self.todo.create(&todoReq.title, &todoReq.description)
            .map(|_| Response::new(CreateTodoResponse::default()))
            .map_err( |_| Status::internal("")) // TODO
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
