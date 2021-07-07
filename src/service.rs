use anyhow::{Error, Result};
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

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{DateTime, NaiveDate, Utc};
    use mockall::predicate::*;
    use mockall::*;

    use crate::proto::Todo;
    use crate::timestamp::Timestamp;

    mock! {
        MyTodoService {}
        #[tonic::async_trait]
        impl TodoService for MyTodoService {
            async fn get(&self, id: &Uuid) -> Result<Todo>;
            async fn list(&self) -> Result<Vec<Todo>>;
            async fn create(&self, title: &str, description: &str) -> Result<Uuid>;
        }
    }

    #[tokio::test]
    async fn create_todo() -> Result<()> {
        let (title, description) = ("clean the house", "start from the kitchen");

        let mut mock = MockMyTodoService::new();
        let mock_uuid = Uuid::new_v4();
        mock.expect_create()
            .with(eq(title), eq(description))
            .return_once(move |_, _| Ok(mock_uuid));

        let sut = Service::new(mock);
        let got = sut.create_todo(Request::new(CreateTodoRequest {
            title: title.to_string(),
            description: description.to_string(),
        }));

        let CreateTodoResponse { id } = got.await?.into_inner();
        assert_eq!(mock_uuid.to_string(), id);

        Ok(())
    }

    #[tokio::test]
    async fn get_todo() -> Result<()> {
        let uuid = Uuid::new_v4();
        let input = Todo {
            id: uuid.to_string(),
            title: "Clean the house".to_string(),
            description: "Start from the kitchen".to_string(),
            created_at: Some(
                Timestamp::from(DateTime::from_utc(
                    NaiveDate::from_ymd(2021, 7, 6).and_hms(22, 0, 0),
                    Utc,
                ))
                .into(),
            ),
            updated_at: Some(
                Timestamp::from(DateTime::from_utc(
                    NaiveDate::from_ymd(2021, 7, 7).and_hms(22, 0, 0),
                    Utc,
                ))
                .into(),
            ),
        };

        let mut mock = MockMyTodoService::new();
        let mock_todo = input.clone();
        mock.expect_get()
            .with(eq(uuid))
            .return_once(move |_| Ok(mock_todo));

        let sut = Service::new(mock);
        let got = sut.get_todo(Request::new(GetTodoRequest {
            id: uuid.to_string(),
        }));

        let GetTodoResponse { todo } = got.await?.into_inner();
        assert_ne!(None, todo);
        assert_eq!(input, todo.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn list_todos() -> Result<()> {
        let input = vec![
            Todo {
                id: Uuid::new_v4().to_string(),
                title: "Clean the house".to_string(),
                description: "Start from the kitchen".to_string(),
                created_at: Some(
                    Timestamp::from(DateTime::from_utc(
                        NaiveDate::from_ymd(2021, 7, 6).and_hms(22, 0, 0),
                        Utc,
                    ))
                    .into(),
                ),
                updated_at: Some(
                    Timestamp::from(DateTime::from_utc(
                        NaiveDate::from_ymd(2021, 7, 7).and_hms(22, 0, 0),
                        Utc,
                    ))
                    .into(),
                ),
            },
            Todo {
                id: Uuid::new_v4().to_string(),
                title: "Wash the bathtab".to_string(),
                description: "Use sponge and clean it".to_string(),
                created_at: Some(
                    Timestamp::from(DateTime::from_utc(
                        NaiveDate::from_ymd(2021, 7, 8).and_hms(22, 0, 0),
                        Utc,
                    ))
                    .into(),
                ),
                updated_at: Some(
                    Timestamp::from(DateTime::from_utc(
                        NaiveDate::from_ymd(2021, 7, 9).and_hms(22, 0, 0),
                        Utc,
                    ))
                    .into(),
                ),
            },
        ];

        let mut mock = MockMyTodoService::new();
        let mock_todos = input.clone();
        mock.expect_list().return_once(move || Ok(mock_todos));

        let sut = Service::new(mock);
        let got = sut.list_todos(Request::new(ListTodosRequest {}));

        let ListTodosResponse { todos } = got.await?.into_inner();
        assert_eq!(input, todos);

        Ok(())
    }
}
