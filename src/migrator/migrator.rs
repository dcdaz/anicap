use std::{collections::HashMap, fs};

use crate::{models::Migration, utils::{self, SqlConnection}};
use diesel::{insert_into, query_dsl::methods::SelectDsl, sql_query, RunQueryDsl};
use crate::schema::migration::dsl::*;

const SCHEMA_DIR: &str = "database";
const MIGRATIONS_DIR: &str = "database/migrations";

fn read_initial_schema_files() -> HashMap<String, Vec<String>> {
    let current_dir = std::env::current_dir().unwrap();
    let database_dir = format!(
        "{}/{}",
        current_dir.as_os_str().to_str().unwrap(),
        SCHEMA_DIR
    );
    fs::read_dir(database_dir).unwrap()
        .filter_map(|dir_entry| {
            let file = dir_entry.unwrap();
            let file_name = file.file_name().display().to_string();
            let file_path = file.path().display().to_string();
            if file_name == "schema.sql" {
                return match fs::read_to_string(file_path) {
                    Ok(content) => Some((file_name, content.split("CREATE TABLE")
                        .skip(1)
                        .map(|s| format!("CREATE TABLE {}", s.trim()))
                        .collect::<Vec<_>>())),
                    Err(_) => None
                }
            }
            None
        })
        .collect::<HashMap<String, Vec<String>>>()
}

fn read_migration_files(applied_migrations: Vec<String>) -> HashMap<String, Vec<String>> {
    let current_dir = std::env::current_dir().unwrap();
    let database_dir = format!(
        "{}/{}",
        current_dir.as_os_str().to_str().unwrap(),
        MIGRATIONS_DIR
    );
    fs::read_dir(database_dir).unwrap()
        .filter_map(|dir_entry| {
            let file = dir_entry.unwrap();
            let file_name = file.file_name().display().to_string();
            let file_path = file.path().display().to_string();
            if !applied_migrations.contains(&file_name) {
                return match fs::read_to_string(file_path) {
                    Ok(content) => Some((file_name, vec![content])),
                    Err(_) => None
                }
            }
            None
        })
        .collect::<HashMap<String, Vec<String>>>()
}

fn get_applied_migration_names(connection: &mut SqlConnection) -> Vec<String> {
    migration
        .select(migration_name)
        .load::<String>(connection).unwrap()
}

fn run_migrations(
    connection: &mut SqlConnection,
    applicable_migrations: HashMap<String, Vec<String>>,
    is_initial_schema: bool
) {
    applicable_migrations.iter()
        .for_each(|(name,content)| content.iter().for_each(|each| {
            match sql_query(each).execute(connection) {
                Ok(_) => {
                    if is_initial_schema {
                        return;
                    }
                    let migrated = insert_into(migration)
                        .values(&Migration { migration_name: name.to_owned() })
                        .execute(connection);

                    match migrated {
                        Ok(_) => println!("Migration: {} was applied successfully", name),
                        Err(e) => println!("Error during migration due to: {}", e.to_string()),
                    }
                },
                Err(e) => println!("Error during executing statement on migration: \"{}\", due to: {}", name, e.to_string()),
            }
        }
        ))
}

pub fn migrate() {
    let mut connection = utils::connect_database().try_get().unwrap();
    let applied_migrations = get_applied_migration_names(&mut connection);
    let applicable_migrations = read_migration_files(applied_migrations);
    run_migrations(&mut connection, applicable_migrations.clone(), false);
}

pub fn initialize_database() {
    let mut connection = utils::connect_database().try_get().unwrap();
    let initial_schema = read_initial_schema_files();
    run_migrations(&mut connection, initial_schema.clone(), true);
}