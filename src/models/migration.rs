use crate::schema::migration;

#[derive(Insertable)]
#[diesel(table_name = migration)]
pub struct Migration {
    pub migration_name: String
}