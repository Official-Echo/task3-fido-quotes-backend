#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20261015_184943_quotes;
mod m20261017_004352_limit_author_text_length;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m20261015_184943_quotes::Migration),
			Box::new(m20261017_004352_limit_author_text_length::Migration),
			// inject-above (do not remove this comment)
		]
	}
}
