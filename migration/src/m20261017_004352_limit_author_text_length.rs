use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Quotes {
	Table,
	Author,
	Text,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
		m.alter_table(
			Table::alter()
				.table(Quotes::Table)
				.modify_column(string_len(Quotes::Author, 200))
				.modify_column(string_len(Quotes::Text, 1000))
				.to_owned(),
		)
		.await
	}

	async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
		m.alter_table(
			Table::alter()
				.table(Quotes::Table)
				.modify_column(string(Quotes::Author))
				.modify_column(text(Quotes::Text))
				.to_owned(),
		)
		.await
	}
}
