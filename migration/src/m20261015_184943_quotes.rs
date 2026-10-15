use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
		create_table(
			m,
			"quotes",
			&[
				("id", ColType::PkAuto),
				("pid", ColType::UuidUniq),
				("author", ColType::String),
				("text", ColType::Text),
			],
			&[],
		)
		.await
	}

	async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
		drop_table(m, "quotes").await
	}
}
