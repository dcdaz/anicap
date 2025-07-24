use std::{collections::HashMap, fs, usize};

use crate::{models::{self, Migration, NewMigration}, utils::{self, SqlConnection}};
use diesel::{insert_into, query_dsl::methods::SelectDsl, result::Error, sql_query, RunQueryDsl};
use crate::schema::migration::dsl::*;

const DATABASE_DIR: &str = "database/migrations";

fn read_migration_files(applied_migrations: Vec<String>) -> HashMap<String, String> {
    let current_dir = std::env::current_dir().unwrap();
    let database_dir = format!(
        "{}/{}",
        current_dir.as_os_str().to_str().unwrap(),
        DATABASE_DIR
    );
    fs::read_dir(database_dir).unwrap()
        .filter_map(|dir_entry| {
            let file = dir_entry.unwrap();
            let file_name = file.file_name().display().to_string();
            let file_path = file.path().display().to_string();
            if !applied_migrations.contains(&file_name) {
                return match fs::read_to_string(file_path) {
                    Ok(content) => Some((file_name, content)),
                    Err(_) => None
                }
            }
            None
        })
        .collect::<HashMap<String, String>>()
}

fn get_applied_migration_names(connection: &mut SqlConnection) -> Vec<String> {
    migration.load::<Migration>(connection).unwrap()
    .iter().map(|m| m.clone().migration_name)
    .collect::<Vec<String>>()
}

fn run_migrations(connection: &mut SqlConnection, applicable_migrations: HashMap<String, String>) {
    applicable_migrations.iter()
        .for_each(|(name,content)| {
            match sql_query(content).execute(connection) {
                Ok(_) => {
                    let success = insert_into(migration)
                        .values(&NewMigration { migration_name: name.to_owned() })
                        .execute(connection);

                    match success { 
                        Ok(_) => println!("Migration: {} was applied successfully", name),
                        Err(e) => println!("Error during migration due to {}", e.to_string()),
                    }
                },
                Err(e) => println!("Error during migration on file \"{}\", due to {}", name, e.to_string()),
            }
        })
}

pub fn migrate() {
    let mut connection = utils::connect_database().try_get().unwrap();
    let applied_migrations = get_applied_migration_names(&mut connection);
    let applicable_migrations = read_migration_files(applied_migrations);
    run_migrations(&mut connection, applicable_migrations.clone());
}