use crate::error::Error;
use crate::model::Todo;

pub trait Store {
    fn create(&self, todo: Todo) -> Result<(), Error>;
    fn get(&self, id: &str) -> Result<Todo, Error>;
    fn gets(&self) -> Result<Vec<Todo>, Error>;
}
