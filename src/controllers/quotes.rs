#![allow(clippy::unused_async)]
use axum::http::StatusCode;
use loco_rs::prelude::*;

use crate::{
	dtos::{
		common::Page,
		quotes::{CreateQuote, QuoteDto, UpdateQuote},
	},
	models::_entities::quotes::{ActiveModel, Column, Entity},
};

#[debug_handler]
async fn list(
	State(ctx): State<AppContext>,
	Query(pagination): Query<query::PaginationQuery>,
) -> Result<Json<Page<QuoteDto>>> {
	let res = query::paginate(
		&ctx.db,
		Entity::find().order_by_asc(Column::Id),
		None,
		&pagination,
	)
	.await?;

	Ok(Json(Page::from_query(res)))
}

#[debug_handler]
async fn get_one(State(ctx): State<AppContext>, Path(id): Path<i64>) -> Result<Response> {
	let Some(model) = Entity::find_by_id(id).one(&ctx.db).await? else {
		return not_found();
	};
	Ok(Json(QuoteDto::from(model)).into_response())
}

#[debug_handler]
async fn create(
	State(ctx): State<AppContext>,
	Json(params): Json<CreateQuote>,
) -> Result<Response> {
	let item = ActiveModel {
		pid: Set(params.pid),
		author: Set(params.author),
		text: Set(params.text),
		..Default::default()
	};
	let item = item.insert(&ctx.db).await?;

	Ok((StatusCode::CREATED, Json(QuoteDto::from(item))).into_response())
}

#[debug_handler]
async fn update(
	State(ctx): State<AppContext>,
	Path(id): Path<i64>,
	Json(params): Json<UpdateQuote>,
) -> Result<Response> {
	let Some(model) = Entity::find_by_id(id).one(&ctx.db).await? else {
		return not_found();
	};

	let mut item = model.into_active_model();
	item.pid = Set(params.pid);
	item.author = Set(params.author);
	item.text = Set(params.text);
	let item = item.update(&ctx.db).await?;

	Ok(Json(QuoteDto::from(item)).into_response())
}

#[debug_handler]
async fn remove(State(ctx): State<AppContext>, Path(id): Path<i64>) -> Result<Response> {
	let Some(model) = Entity::find_by_id(id).one(&ctx.db).await? else {
		return not_found();
	};

	model.into_active_model().delete(&ctx.db).await?;

	Ok(StatusCode::NO_CONTENT.into_response())
}

pub fn routes() -> Routes {
	Routes::new()
		.prefix("/api/quotes")
		.add("/", get(list))
		.add("/", post(create))
		.add("/{id}", get(get_one))
		.add("/{id}", put(update))
		.add("/{id}", delete(remove))
}
