use chrono::NaiveDateTime;

use crate::schema::migration;

#[derive(Queryable, Clone)]
pub struct Migration {
    pub id: i16,
    pub migration_name: String,
    pub migrated_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name = migration)]
pub struct NewMigration {
    pub migration_name: String
}