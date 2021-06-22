use std::error::Error;
use std::sync::{Arc,Mutex};

use crate::proto::Todo;
use crate::todo::Service as TodoService;
use postgres::{Client, NoTls};

pub(crate) struct TodoRepo {
    client: Arc<Mutex<Client>>,
}

impl TodoRepo {
    pub fn new(conn_str: &str) -> Result<Self, Box<dyn Error>> {
        let mut client = Client::connect(conn_str, NoTls)?;
        migrate(&mut client)?;

        Ok(Self { client: Arc::new(Mutex::new(client)) })
    }
}

impl TodoService for TodoRepo {
    fn create(&'static self, title: &str, description: &str) -> Result<(), Box<dyn Error>> {
        self.client.lock()?
            .execute(
            "INSERT INTO todos (title, description) VALUES ($1, $2)", 
            &[&title, &description]
            )?;

        Ok(())
    }

    fn get(&self, id: &str) -> Result<Todo, Box<dyn Error>> {
        Ok(Todo {
            id: id.to_string(),
            title: "".to_string(),
            description: "".to_string(),
        })
    }

    fn list(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        Ok(vec![])
    }
}

fn migrate(client: &mut Client) -> Result<(), Box<dyn Error>> {
    client.batch_execute(r#"
CREATE TABLE IF NOT EXISTS todos (
    id          UUID        PRIMARY KEY DEFAULT generate_uuid(),
    title       STRING      NOT NULL,
    description STRING      NOT NULL    DEFAULT '',
    created_at  TIMESTAMPTZ NOT NULL    DEFAULT now()
);
"#)?;

    Ok(())
}