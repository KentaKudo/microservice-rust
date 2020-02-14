use crate::store::Store;
use crate::service_grpc::{
    server, Todo,
    GetTodoRequest, GetTodoResponse,
    CreateTodoRequest, CreateTodoResponse,
    ListTodosRequest, ListTodosResponse,
};

use futures::future;
use tower_grpc::{Request, Response};

#[derive(Clone)]
pub(crate) struct API<S: Store> {
    store: S,
}

impl<S: Store> API<S> {
    pub fn new(store: S) -> Self {
        API { store }
    }
}

impl<S: Store> server::TodoApi for API<S> {
    type GetTodoFuture = future::FutureResult<Response<GetTodoResponse>, tower_grpc::Status>;
    type CreateTodoFuture = future::FutureResult<Response<CreateTodoResponse>, tower_grpc::Status>;
    type ListTodosFuture = future::FutureResult<Response<ListTodosResponse>, tower_grpc::Status>;

    fn get_todo(
        &mut self,
        request: Request<GetTodoRequest>,
    ) -> Self::GetTodoFuture {
        let todo = self.store.get(&request.get_ref().id)?;
        future::ok(Response::new(GetTodoResponse {
                todo: Some(Todo{
                    id: todo.id,
                    title: todo.title,
                    description: todo.description,
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
