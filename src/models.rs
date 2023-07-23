use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
pub struct TodoItem {
    pub task: String,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TodoList {
    task_id: i32,
    task: String,
    complete: bool,
}
