use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240101_000002_create_version_management_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VersionManagement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(VersionManagement::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(VersionManagement::Date).date().not_null())
                    .col(ColumnDef::new(VersionManagement::Version).string().not_null())
                    .col(ColumnDef::new(VersionManagement::CodeContent).text().not_null())
                    .col(ColumnDef::new(VersionManagement::Name).string().not_null())
                    .col(ColumnDef::new(VersionManagement::CreationDate).date_time().not_null())
                    .col(ColumnDef::new(VersionManagement::ImagePreview).binary())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(VersionManagement::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum VersionManagement {
    Table,
    Id,
    Date,
    Version,
    CodeContent,
    Name,
    CreationDate,
    ImagePreview,
}