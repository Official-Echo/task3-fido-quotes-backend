#![allow(clippy::unused_async)]
use axum::http::StatusCode;
use loco_rs::{controller::extractor::validate::QueryValidateWithMessage, prelude::*};

use crate::{
	dtos::{
		common::Page,
		quotes::{CreatedQuoteResponse, ListParams, QuoteDto, QuoteParams},
	},
	models::_entities::quotes::{ActiveModel, Column, Entity},
};

const PAGE_SIZE_LIMIT: u64 = 67;

// GET /quotes?author=&page=&page_size=
#[debug_handler]
async fn list(
	State(ctx): State<AppContext>,
	QueryValidateWithMessage(params): QueryValidateWithMessage<ListParams>,
) -> Result<Json<Page<QuoteDto>>> {
	// use consistent id instead of uuid for stable pagination
	let mut select = Entity::find().order_by_asc(Column::Id);
	if let Some(author) = params.author.as_deref().filter(|a| !a.is_empty()) {
		select = select.filter(Column::Author.eq(author));
	}

	let pagination = query::PaginationQuery {
		page: params.pagination.page.max(1),
		page_size: params.pagination.page_size.clamp(1, PAGE_SIZE_LIMIT),
	};
	let res = query::paginate(&ctx.db, select, None, &pagination).await?;

	Ok(Json(Page::from_query(res)))
}

// GET /quotes/{id}
#[debug_handler]
async fn get_one(
	State(ctx): State<AppContext>,
	Path(id): Path<Uuid>,
) -> Result<Json<QuoteDto>> {
	let quote = Entity::find_by_pid(&ctx.db, id).await?;
	Ok(Json(QuoteDto::from(quote)))
}

// POST /quotes
#[debug_handler]
async fn create(
	State(ctx): State<AppContext>,
	JsonValidateWithMessage(params): JsonValidateWithMessage<QuoteParams>,
) -> Result<Response> {
	let item = ActiveModel {
		author: Set(params.author),
		text: Set(params.text),
		..Default::default()
	};

	let item = item.insert(&ctx.db).await?;

	Ok((
		StatusCode::CREATED,
		Json(CreatedQuoteResponse { id: item.pid }),
	)
		.into_response())
}

#[debug_handler]
async fn update(
	State(ctx): State<AppContext>,
	Path(id): Path<Uuid>,
	JsonValidateWithMessage(params): JsonValidateWithMessage<QuoteParams>,
) -> Result<Json<QuoteDto>> {
	let mut item = Entity::find_by_pid(&ctx.db, id).await?.into_active_model();
	item.author = Set(params.author);
	item.text = Set(params.text);

	let item = item.update(&ctx.db).await?;

	Ok(Json(QuoteDto::from(item)))
}

// DELETE /quotes/{id}
#[debug_handler]
async fn remove(State(ctx): State<AppContext>, Path(id): Path<Uuid>) -> Result<StatusCode> {
	let res = Entity::delete_many_by_pid(&ctx.db, id).await?;

	if res == 0 {
		return not_found();
	}

	Ok(StatusCode::NO_CONTENT)
}

pub fn routes() -> Routes {
	Routes::new()
		.prefix("/quotes")
		.add("/", get(list))
		.add("/", post(create))
		.add("/{id}", get(get_one))
		.add("/{id}", put(update))
		.add("/{id}", delete(remove))
}
