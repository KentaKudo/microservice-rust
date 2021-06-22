use std::time;

use anyhow::{Context, Result};
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;

use crate::proto::Todo;

#[derive(Debug)]
pub(crate) struct TodoRepo {
    client: Client,
}

impl TodoRepo {
    pub async fn new(conn_str: &str) -> Result<Self> {
        let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        migrate(&client).await.context(format!("migrate()"))?;

        Ok(Self { client })
    }
}

impl TodoRepo {
    pub async fn create(&self, title: &str, description: &str) -> Result<String> {
        let stmt = self
            .client
            .prepare("INSERT INTO todos (title, description) VALUES ($1, $2) RETURNING id")
            .await?;
        let rows = self.client.query(&stmt, &[&title, &description]).await?;

        let mut id = Uuid::nil();
        for row in rows {
            id = row.try_get(0)?;
        }

        Ok(id.to_string())
    }

    pub async fn get(&self, id: &Uuid) -> Result<Todo> {
        let row = self
            .client
            .query_one(
                "SELECT title, description, created_at, updated_at FROM todos WHERE id = $1",
                &[&id],
            )
            .await?;

        let created_at = row.try_get::<&str, time::Time>("created_at")?;
        Ok(Todo {
            id: id.to_string(),
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            created_at,
        })
    }

    pub async fn list(&self) -> Result<Vec<Todo>> {
        let rows = self.client.query("SELECT * FROM todos", &[]).await?;

        let mut todos = vec![];
        for row in rows {
            let todo = Todo {
                id: row.try_get::<&str, Uuid>("id")?.to_string(),
                title: row.try_get("title")?,
                description: row.try_get("description")?,
            };

            todos.push(todo);
        }

        Ok(todos)
    }
}

async fn migrate(client: &Client) -> Result<()> {
    client
        .batch_execute(
            r#"
CREATE TABLE IF NOT EXISTS todos (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    title       STRING      NOT NULL,
    description STRING      NOT NULL    DEFAULT '',
    created_at  TIMESTAMPTZ NOT NULL    DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL    DEFAULT now()
);
"#,
        )
        .await
        .context(format!("client.batch_execute()"))?;

    Ok(())
}
