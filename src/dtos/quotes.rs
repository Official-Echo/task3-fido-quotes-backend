use sea_orm::prelude::{DateTimeWithTimeZone, Uuid};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct QuoteDto {
	pub id: i64,
	pub pid: Uuid,
	pub author: String,
	pub text: String,
	pub created_at: DateTimeWithTimeZone,
	pub updated_at: DateTimeWithTimeZone,
}

impl From<crate::models::_entities::quotes::Model> for QuoteDto {
	fn from(m: crate::models::_entities::quotes::Model) -> Self {
		Self {
			id: m.id,
			pid: m.pid,
			author: m.author,
			text: m.text,
			created_at: m.created_at,
			updated_at: m.updated_at,
		}
	}
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateQuote {
	pub pid: Uuid,
	pub author: String,
	pub text: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateQuote {
	pub pid: Uuid,
	pub author: String,
	pub text: String,
}
