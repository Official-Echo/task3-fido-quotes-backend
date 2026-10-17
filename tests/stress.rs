use loco_rs::testing::prelude::*;
use quotes_backend::app::App;
use serde_json::{json, Value};
use serial_test::serial;

const AUTHORS: usize = 5;
const PER_AUTHOR: usize = 60;

#[ignore = "run stress test manually with -- --ignored"]
#[tokio::test]
#[serial]
async fn mass_create_update_delete() {
	request::<App, _, _>(|request, _ctx| async move {
		let mut ids: Vec<Vec<String>> = vec![];
		for a in 0..AUTHORS {
			let mut author_ids = vec![];
			for i in 0..PER_AUTHOR {
				let res = request
					.post("/quotes")
					.json(
						&json!({"author": format!("stress-{a}"), "text": format!("quote {i}")}),
					)
					.await;
				assert_eq!(res.status_code(), 201);
				author_ids.push(res.json::<Value>()["id"].as_str().unwrap().to_string());
			}
			ids.push(author_ids);
		}

		for a in 0..AUTHORS {
			let mut seen = vec![];
			for page in 1..=3 {
				let res = request
					.get("/quotes")
					.add_query_param("author", format!("stress-{a}"))
					.add_query_param("page", page)
					.add_query_param("page_size", 25)
					.await;
				assert_eq!(res.status_code(), 200);
				let body = res.json::<Value>();
				assert_eq!(body["total_items"], PER_AUTHOR);
				assert_eq!(body["total_pages"], 3);
				for q in body["items"].as_array().unwrap() {
					seen.push(q["id"].as_str().unwrap().to_string());
				}
			}
			assert_eq!(seen.len(), PER_AUTHOR);
			seen.sort();
			let mut expected = ids[a].clone();
			expected.sort();
			assert_eq!(seen, expected);
		}

		for a in 0..AUTHORS {
			for (i, id) in ids[a].iter().enumerate() {
				let res = request
					.put(&format!("/quotes/{id}"))
					.json(
						&json!({"author": format!("stress-{a}"), "text": format!("edited {i}")}),
					)
					.await;
				assert_eq!(res.status_code(), 200);
			}
			for (i, id) in ids[a].iter().enumerate() {
				let q = request.get(&format!("/quotes/{id}")).await.json::<Value>();
				assert_eq!(q["text"], format!("edited {i}"));
			}
		}

		for a in 0..AUTHORS {
			for (i, id) in ids[a].iter().enumerate().filter(|(i, _)| i % 2 == 0) {
				assert_eq!(
					request.delete(&format!("/quotes/{id}")).await.status_code(),
					204
				);
				assert_eq!(
					request.get(&format!("/quotes/{id}")).await.status_code(),
					404,
					"{i}"
				);
			}
			let res = request
				.get("/quotes")
				.add_query_param("author", format!("stress-{a}"))
				.add_query_param("page_size", 67)
				.await;
			assert_eq!(res.json::<Value>()["total_items"], PER_AUTHOR / 2);
		}

		for a in 0..AUTHORS {
			for id in ids[a].iter().skip(1).step_by(2) {
				assert_eq!(
					request.delete(&format!("/quotes/{id}")).await.status_code(),
					204
				);
			}
			let res = request
				.get("/quotes")
				.add_query_param("author", format!("stress-{a}"))
				.await;
			assert_eq!(res.json::<Value>()["total_items"], 0);
		}
	})
	.await;
}

#[tokio::test]
#[serial]
async fn repeated_create_delete_cycles() {
	request::<App, _, _>(|request, _ctx| async move {
		for i in 0..200 {
			let res = request
				.post("/quotes")
				.json(&json!({"author": "cycle", "text": format!("v1 {i}")}))
				.await;
			assert_eq!(res.status_code(), 201);
			let id = res.json::<Value>()["id"].as_str().unwrap().to_string();

			let res = request
				.put(&format!("/quotes/{id}"))
				.json(&json!({"author": "cycle", "text": format!("v2 {i}")}))
				.await;
			assert_eq!(res.status_code(), 200);
			let q = request.get(&format!("/quotes/{id}")).await.json::<Value>();
			assert_eq!(q["text"], format!("v2 {i}"));

			assert_eq!(
				request.delete(&format!("/quotes/{id}")).await.status_code(),
				204
			);
			assert_eq!(
				request.get(&format!("/quotes/{id}")).await.status_code(),
				404
			);
		}
		let res = request
			.get("/quotes")
			.add_query_param("author", "cycle")
			.await;
		assert_eq!(res.json::<Value>()["total_items"], 0);
	})
	.await;
}
