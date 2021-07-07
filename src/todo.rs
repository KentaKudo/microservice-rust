use anyhow::Result;
use uuid::Uuid;

use crate::proto::Todo;

#[tonic::async_trait]
pub trait TodoService {
    async fn get(&self, id: &Uuid) -> Result<Todo>;
    async fn list(&self) -> Result<Vec<Todo>>;
    async fn create(&self, title: &str, description: &str) -> Result<Uuid>;
}
