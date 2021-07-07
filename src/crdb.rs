use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use tokio_postgres::{Client, NoTls};
use uuid::Uuid;

use crate::proto::Todo;
use crate::timestamp::Timestamp;
use crate::todo::TodoService;

#[derive(Debug)]
pub(crate) struct TodoRepo {
    client: Client,
}

impl TodoRepo {
    pub async fn new(conn_str: &str) -> Result<Self> {
        let (client, connection) = tokio_postgres::connect(conn_str, NoTls)
            .await
            .context(format!("tokip_postgres::connect({})", conn_str))?;

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

#[tonic::async_trait]
impl TodoService for TodoRepo {
    async fn create(&self, title: &str, description: &str) -> Result<Uuid> {
        let stmt = self
            .client
            .prepare("INSERT INTO todos (title, description) VALUES ($1, $2) RETURNING id")
            .await
            .context("self.client.prepare()")?;
        let rows = self
            .client
            .query(&stmt, &[&title, &description])
            .await
            .context("self.client.query()")?;

        let mut id = Uuid::nil();
        for row in rows {
            id = row.try_get(0).context("row.try_get(0)")?;
        }

        Ok(id)
    }

    async fn get(&self, id: &Uuid) -> Result<Todo> {
        let row = self
            .client
            .query_one(
                "SELECT title, description, created_at, updated_at FROM todos WHERE id = $1",
                &[&id],
            )
            .await
            .context(format!("self.client.query_one({})", id))?;

        let created_at = row
            .try_get::<&str, DateTime<Utc>>("created_at")
            .context("row.try_get(\"created_at\")")?;
        let updated_at = row
            .try_get::<&str, DateTime<Utc>>("updated_at")
            .context("row.try_get(\"updated_at\")")?;

        Ok(Todo {
            id: id.to_string(),
            title: row.try_get("title").context("row.try_get(\"title\")")?,
            description: row
                .try_get("description")
                .context("row.try_get(\"description\")")?,
            created_at: Some(Timestamp::from(created_at).into()),
            updated_at: Some(Timestamp::from(updated_at).into()),
        })
    }

    async fn list(&self) -> Result<Vec<Todo>> {
        let rows = self
            .client
            .query("SELECT * FROM todos", &[])
            .await
            .context("self.client.query()")?;

        let mut todos = vec![];
        for row in rows {
            let created_at = row
                .try_get::<&str, DateTime<Utc>>("created_at")
                .context("row.try_get(\"created_at\")")?;
            let updated_at = row
                .try_get::<&str, DateTime<Utc>>("updated_at")
                .context("row.try_get(\"updated_at\")")?;

            let todo = Todo {
                id: row
                    .try_get::<&str, Uuid>("id")
                    .context("row.try_get(\"id\")")?
                    .to_string(),
                title: row.try_get("title").context("row.try_get(\"title\")")?,
                description: row
                    .try_get("description")
                    .context("row.try_get(\"description\")")?,
                created_at: Some(Timestamp::from(created_at).into()),
                updated_at: Some(Timestamp::from(updated_at).into()),
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

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_db() -> Result<String> {
        let (client, connection) =
            tokio_postgres::connect("postgresql://root@localhost:26257", NoTls).await?;
        let db = Uuid::new_v4();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        client
            .batch_execute(&format!(
                "CREATE DATABASE IF NOT EXISTS \"{}\"",
                &db.to_string()
            ))
            .await
            .context("client.batch_execute()")?;

        Ok(format!(
            "postgresql://root@localhost:26257/{}",
            db.to_string()
        ))
    }

    #[tokio::test]
    async fn create_todo() -> Result<()> {
        let dns = &create_db().await.context("create_db()")?;
        let sut = TodoRepo::new(dns).await.context("TodoRepo::new()")?;

        let (title, description) = ("clean the house", "start with kitchen");
        let id = sut.create(title, description).await?;
        assert_ne!(Uuid::nil(), id);

        let row = sut
            .client
            .query_one("SELECT * from todos WHERE id = $1", &[&id])
            .await
            .context("sut.client.query_one()")?;

        let got_title = row.try_get::<&str, &str>("title")?;
        assert_eq!(title, got_title);

        let got_description = row.try_get::<&str, &str>("description")?;
        assert_eq!(description, got_description);

        let got_created_at = row.try_get::<&str, DateTime<Utc>>("created_at")?;
        assert_ne!(0, got_created_at.timestamp());

        let got_updated_at = row.try_get::<&str, DateTime<Utc>>("updated_at")?;
        assert_ne!(0, got_updated_at.timestamp());

        Ok(())
    }

    #[tokio::test]
    async fn get_todo() -> Result<()> {
        let dns = &create_db().await.context("create_db()")?;
        let sut = TodoRepo::new(dns).await.context("TodoRepo::new()")?;

        let (title, description) = ("clean the house", "start with kitchen");
        let id = sut
            .create(title, description)
            .await
            .context(format!("sut.create({}, {})", title, description))?;

        let got = sut.get(&id).await.context(format!("sut.get({})", id))?;
        assert_eq!(
            id,
            Uuid::parse_str(&got.id).context(format!("Uuid::parse_str({})", got.id))?
        );
        assert_eq!(title, got.title);
        assert_eq!(description, got.description);
        assert_ne!(None, got.created_at);
        assert_ne!(None, got.updated_at);

        Ok(())
    }

    #[tokio::test]
    async fn list_todos() -> Result<()> {
        let dns = &create_db().await.context("create_db()")?;
        let sut = TodoRepo::new(dns).await.context("TodoRepo::new()")?;

        let (title, description) = ("clean the house", "start with kitchen");
        let _ = sut
            .create(title, description)
            .await
            .context(format!("sut.create({}, {})", title, description))?;
        let (title, description) = ("go for shopping", "eggs, milk, tomatoes, cheese, ...");
        let _ = sut
            .create(title, description)
            .await
            .context(format!("sut.create({}, {})", title, description))?;

        let got = sut.list().await.context("sut.list()")?;
        assert_eq!(2, got.len());

        Ok(())
    }
}
