use std::time::SystemTime;

use diesel::{Selectable, prelude::Queryable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
