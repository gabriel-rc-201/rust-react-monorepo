pub use sea_orm_migration::prelude::*;

mod m20250824_172255_create_users_and_tasks;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250824_172255_create_users_and_tasks::Migration),
        ]
    }
}
