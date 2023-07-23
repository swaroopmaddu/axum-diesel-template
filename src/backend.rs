use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection, RunQueryDsl,
};

use crate::models::TodoItem;

#[derive(Clone)]
pub struct DbClient {
    pub db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl DbClient {
    pub fn new(db_url: &str) -> Self {
        Self {
            db_pool: Arc::new(
                Pool::builder()
                    .build(ConnectionManager::<PgConnection>::new(db_url))
                    .expect("Failed to create pool."),
            ),
        }
    }
    pub async fn create_task(&self, task: &TodoItem) -> Result<(), diesel::result::Error> {
        let conn = &mut self.db_pool.get().unwrap();

        let _ = diesel::insert_into(crate::schema::tasks::table)
            .values(task)
            .execute(conn)?;

        Ok(())
    }
}
