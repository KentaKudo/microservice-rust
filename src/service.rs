use crate::service_grpc::{
    server, Todo,
    GetTodoRequest, GetTodoResponse,
    CreateTodoRequest, CreateTodoResponse,
    ListTodosRequest, ListTodosResponse,
};

use futures::future;
use tower_grpc::{Request, Response};

#[derive(Clone)]
pub(crate) struct API {}

impl API {
    pub fn new() -> Self {
        API {}
    }
}

impl server::TodoApi for API {
    type GetTodoFuture = future::FutureResult<Response<GetTodoResponse>, tower_grpc::Status>;
    type CreateTodoFuture = future::FutureResult<Response<CreateTodoResponse>, tower_grpc::Status>;
    type ListTodosFuture = future::FutureResult<Response<ListTodosResponse>, tower_grpc::Status>;

    fn get_todo(
        &mut self,
        request: Request<GetTodoRequest>,
    ) -> Self::GetTodoFuture {
        future::ok(Response::new(GetTodoResponse {
                todo: Some(Todo{
                    id: String::default(),
                    title: String::default(),
                    description: String::default(),
                })
            }))
    }

    fn create_todo(
        &mut self,
        request: Request<CreateTodoRequest>,
    ) -> Self::CreateTodoFuture {
        future::ok(Response::new(CreateTodoResponse {
            id: String::default(),
        }))
    }

    fn list_todos(
        &mut self,
        request: Request<ListTodosRequest>,
    ) -> Self::ListTodosFuture {
        future::ok(Response::new(ListTodosResponse {
            todos: vec!(),
        }))
    }
}
