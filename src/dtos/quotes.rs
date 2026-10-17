use loco_rs::model::query::PaginationQuery;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use serde_trim::{option_string_trim, string_trim};
use validator::{Validate, ValidationError};

const AUTHOR_MAX_LENGTH: usize = 200;
const TEXT_MAX_LENGTH: usize = 1000;

fn check_length(
	field: &str,
	value: &str,
	range: std::ops::RangeInclusive<usize>,
) -> Result<(), ValidationError> {
	let len = value.chars().count();

	if value.chars().any(|c| c.is_control()) {
		return Err(ValidationError::new("invalid_chars")
			.with_message(format!("поле `{field}` містить недопустимі символи").into()));
	}

	if len == 0 && *range.start() > 0 {
		return Err(ValidationError::new("blank")
			.with_message(format!("поле `{field}` не може бути порожнім").into()));
	}

	if !range.contains(&len) {
		return Err(ValidationError::new("too_long").with_message(
			format!(
				"поле `{field}` не може бути довшим за {} символів",
				range.end()
			)
			.into(),
		));
	}

	Ok(())
}

fn validate_author(value: &str) -> Result<(), ValidationError> {
	check_length("author", value, 1..=AUTHOR_MAX_LENGTH)
}

fn validate_author_filter(value: &str) -> Result<(), ValidationError> {
	check_length("author", value, 0..=AUTHOR_MAX_LENGTH)
}

fn validate_text(value: &str) -> Result<(), ValidationError> {
	check_length("text", value, 1..=TEXT_MAX_LENGTH)
}

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
	#[serde(default, deserialize_with = "string_trim")]
	#[validate(custom(function = "validate_author"))]
	pub author: String,
	#[serde(default, deserialize_with = "string_trim")]
	#[validate(custom(function = "validate_text"))]
	pub text: String,
}

// GET /quotes
#[derive(Debug, Deserialize, Validate)]
pub struct ListParams {
	#[serde(default, deserialize_with = "option_string_trim")]
	#[validate(custom(function = "validate_author_filter"))]
	pub author: Option<String>,
	#[serde(flatten)]
	pub pagination: PaginationQuery,
}

// POST /quotes
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatedQuoteResponse {
	pub id: Uuid,
}
