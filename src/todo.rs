use std::error::Error;

use crate::proto::Todo;

pub trait Service: Send + Sync {
    fn create(&'static self, title: &str, description: &str) -> Result<(), Box<dyn Error>>;
    fn get(&self, id: &str) -> Result<Todo, Box<dyn Error>>;
    fn list(&self) -> Result<Vec<Todo>, Box<dyn Error>>;
}
