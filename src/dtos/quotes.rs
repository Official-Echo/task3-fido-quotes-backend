use loco_rs::model::query::PaginationQuery;
use sea_orm::prelude::{DateTimeWithTimeZone, Uuid};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct QuoteDto {
	pub id: Uuid,
	pub author: String,
	pub text: String,
}

impl From<crate::models::_entities::quotes::Model> for QuoteDto {
	fn from(m: crate::models::_entities::quotes::Model) -> Self {
		Self {
			id: m.pid,
			author: m.author,
			text: m.text,
		}
	}
}

// POST /quotes, PUT /quotes/{id}
#[derive(Debug, Deserialize, Validate)]
pub struct QuoteParams {
	#[serde(default)]
	#[validate(length(
		min = 1,
		max = 200,
		message = "author cannot be empty or longer than 200 characters"
	))]
	pub author: String,
	#[serde(default)]
	#[validate(length(
		min = 1,
		max = 1000,
		message = "text cannot be empty or longer than 1000 characters"
	))]
	pub text: String,
}

// GET /quotes
#[derive(Debug, Deserialize, Validate)]
pub struct ListParams {
	#[validate(length(max = 200))]
	pub author: Option<String>,
	#[serde(flatten)]
	pub pagination: PaginationQuery,
}

// POST /quotes
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatedQuoteResponse {
	pub id: Uuid,
}
