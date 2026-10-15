pub use super::_entities::quotes::{ActiveModel, Column, Entity, Model};
use loco_rs::prelude::*;

pub type Quotes = Entity;

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
	async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
	where
		C: ConnectionTrait,
	{
		let mut this = self;
		if !insert {
			this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
		} else {
			this.pid = sea_orm::ActiveValue::Set(Uuid::new_v4());
		}
		Ok(this)
	}
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {
	pub async fn find_by_pid(db: &DatabaseConnection, pid: Uuid) -> Result<Model> {
		Self::find()
			.filter(Column::Pid.eq(pid))
			.one(db)
			.await?
			.ok_or(Error::NotFound)
	}
	pub async fn delete_many_by_pid(db: &DatabaseConnection, pid: Uuid) -> Result<u64> {
		let res = Self::delete_many()
			.filter(Column::Pid.eq(pid))
			.exec(db)
			.await?;

		Ok(res.rows_affected)
	}
}
