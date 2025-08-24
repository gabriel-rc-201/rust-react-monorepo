use sea_orm_migration::{prelude::*};
use crate::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // cria enum EnumStatus
        let _ = manager
            .create_type(
                Type::create()
                    .as_enum(EnumStatus::Table)
                    .values([
                        EnumStatus::Todo,
                        EnumStatus::InProgress,
                        EnumStatus::Completed,
                    ])
                    .to_owned(),
            )
            .await;

        // cria tabela de usuarios
        let _ = manager
            .create_table(
                Table::create()
                    .table(Usuarios::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Usuarios::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()")
                    )
                    .col(ColumnDef::new(Usuarios::Nome).string().not_null())
                    .col(ColumnDef::new(Usuarios::Senha).string().not_null())
                    .col(ColumnDef::new(Usuarios::Email).string().unique_key().not_null())
                    .col(
                        ColumnDef::new(Usuarios::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()")
                    )
                    .col(
                        ColumnDef::new(Usuarios::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()")
                    )
                    .to_owned()
            )
            .await;

        // cria tabela de tarefas
        let _ = manager
            .create_table(
                Table::create()
                    .table(Tarefas::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tarefas::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()")
                    )
                    .col(
                        ColumnDef::new(Tarefas::UsuarioId)
                            .uuid()
                            .not_null()
                    )
                    .col(ColumnDef::new(Tarefas::Titulo).string().not_null())
                    .col(ColumnDef::new(Tarefas::Descricao).string().not_null())
                    .col(
                        ColumnDef::new(Tarefas::Status)
                            .enumeration(EnumStatus::Table, [
                                EnumStatus::Todo,
                                EnumStatus::InProgress,
                                EnumStatus::Completed,
                            ])
                            .not_null()
                            .default(EnumStatus::Todo.to_string())
                    )
                    .col(ColumnDef::new(Tarefas::Praso).date().not_null())
                    .col(
                        ColumnDef::new(Tarefas::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()")
                    )
                    .col(
                        ColumnDef::new(Tarefas::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()")
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_usuario")
                            .from(Tarefas::Table, Tarefas::UsuarioId)
                            .to(Usuarios::Table, Usuarios::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned()
            )
            .await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Usuarios::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Tarefas::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(EnumStatus::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Usuarios{
    Table,
    Id,
    Nome,
    Senha,
    Email,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Tarefas {
    Table,
    Id,
    UsuarioId,
    Titulo,
    Descricao,
    Status,
    Praso,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum EnumStatus {
    Table,
    #[sea_orm(iden = "TODO")]
    Todo,
    #[sea_orm(iden = "IN_PROGRESS")]
    InProgress,
    #[sea_orm(iden = "COMPLETED")]
    Completed
}