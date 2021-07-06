use anyhow::Result;
use uuid::Uuid;

use crate::proto::Todo;

pub trait TodoReader {
    fn get(&self, id: &Uuid) -> Result<Todo>;
    fn list(&self) -> Result<Vec<Todo>>;
}

pub trait TodoWriter {
    fn create(&self, title: &str, description: &str) -> Result<Uuid>;
}

pub trait TodoService: TodoReader + TodoWriter {}
