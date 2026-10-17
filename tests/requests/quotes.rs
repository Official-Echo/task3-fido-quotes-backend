use loco_rs::testing::prelude::*;
use quotes_backend::app::App;
use serde_json::{json, Value};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn create_get_update_delete_flow() {
	request::<App, _, _>(|request, _ctx| async move {
        let create_res = request
            .post("/quotes")
            .json(&json!({"author": "Трохим Бабич", "text": "Я зараз скажу страшне число — 2010 рік"}))
            .await;
        assert_eq!(create_res.status_code(), 201);
        let id = create_res.json::<Value>()["id"].as_str().expect("id must be a string").to_string();
        assert_eq!(id.len(), 36, "id must be a UUID");

        let read_res = request.get(&format!("/quotes/{id}")).await;
        assert_eq!(read_res.status_code(), 200);
        assert_eq!(
            read_res.json::<Value>(),
            json!({"id": id, "author": "Трохим Бабич", "text": "Я зараз скажу страшне число — 2010 рік"})
        );

        let update_res = request
            .put(&format!("/quotes/{id}"))
            .json(&json!({"author": "Руслан Чорней", "text": "Київ — це центр людської цивілізації."}))
            .await;
        assert_eq!(update_res.status_code(), 200);
        let res = request.get(&format!("/quotes/{id}")).await;
        assert_eq!(res.json::<Value>()["author"], "Руслан Чорней");

        let delete_res = request.delete(&format!("/quotes/{id}")).await;
        assert_eq!(delete_res.status_code(), 204);
        assert_eq!(request.get(&format!("/quotes/{id}")).await.status_code(), 404);
        assert_eq!(request.delete(&format!("/quotes/{id}")).await.status_code(), 404);
        assert_eq!(
            request
                .put(&format!("/quotes/{id}"))
                .json(&json!({"author": "A", "text": "B"}))
                .await
                .status_code(),
            404
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn validation_rejects_bad_input() {
	request::<App, _, _>(|request, _ctx| async move {
		let bad = [
			json!({"author": "", "text": "ok"}),
			json!({"author": "ok", "text": ""}),
			json!({"author": "   ", "text": "ok"}),
			json!({"author": "ok"}),
			json!({"author": "a".repeat(201), "text": "ok"}),
			json!({"author": "ok", "text": "t".repeat(1001)}),
		];
		for body in bad {
			let res = request.post("/quotes").json(&body).await;
			assert_eq!(res.status_code(), 400, "body: {body}");
			assert!(res.json::<Value>()["errors"].is_object(), "body: {body}");
		}

		let ok = request
			.post("/quotes")
			.json(&json!({"author": "я".repeat(200), "text": "ж".repeat(1000)}))
			.await;
		assert_eq!(ok.status_code(), 201);
		let id = ok.json::<Value>()["id"].as_str().unwrap().to_string();

		let res = request
			.put(&format!("/quotes/{id}"))
			.json(&json!({"author": "", "text": "new"}))
			.await;
		assert_eq!(res.status_code(), 400);
		let stored = request.get(&format!("/quotes/{id}")).await.json::<Value>();
		assert_eq!(stored["text"], "ж".repeat(1000));
	})
	.await;
}

#[tokio::test]
#[serial]
async fn list_filters_by_author_and_paginates() {
	request::<App, _, _>(|request, _ctx| async move {
		for i in 1..=5 {
			let res = request
				.post("/quotes")
				.json(&json!({"author": "Pager", "text": format!("quote {i}")}))
				.await;
			assert_eq!(res.status_code(), 201);
		}
		request
			.post("/quotes")
			.json(&json!({"author": "Other", "text": "x"}))
			.await;

		let res = request
			.get("/quotes")
			.add_query_param("author", "Pager")
			.add_query_param("page", 2)
			.add_query_param("page_size", 2)
			.await;
		assert_eq!(res.status_code(), 200);
		let body = res.json::<Value>();
		assert_eq!(body["items"].as_array().unwrap().len(), 2);
		assert_eq!(body["total_items"], 5);
		assert_eq!(body["total_pages"], 3);
		assert_eq!(body["page"], 2);
		assert!(body["items"]
			.as_array()
			.unwrap()
			.iter()
			.all(|q| q["author"] == "Pager"));

		let all = request.get("/quotes").await.json::<Value>();
		assert!(all["total_items"].as_u64().unwrap() >= 6);
	})
	.await;
}

#[tokio::test]
#[serial]
async fn list_rejects_too_long_author_filter() {
	request::<App, _, _>(|request, _ctx| async move {
		let res = request
			.get("/quotes")
			.add_query_param("author", "а".repeat(201))
			.await;
		assert_eq!(res.status_code(), 400);
		assert!(res.json::<Value>()["errors"]["author"].is_array());

		let res = request
			.get("/quotes")
			.add_query_param("author", "а".repeat(200))
			.await;
		assert_eq!(res.status_code(), 200);
		let res = request.get("/quotes").add_query_param("author", "").await;
		assert_eq!(res.status_code(), 200);
	})
	.await;
}

#[tokio::test]
#[serial]
async fn author_validation_strips_whitespace_before_length_check() {
	request::<App, _, _>(|request, _ctx| async move {
		let res = request
			.post("/quotes")
			.json(&json!({
				"author": format!("{}   ", "а".repeat(200)),
				"text": "ok"
			}))
			.await;
		assert_eq!(res.status_code(), 201);

		let res = request
			.post("/quotes")
			.json(&json!({
				"author": format!("{}   ", "а".repeat(201)),
				"text": "ok"
			}))
			.await;
		assert_eq!(res.status_code(), 400);
		assert!(res.json::<Value>()["errors"]["author"].is_array());
	})
	.await;
}

#[tokio::test]
#[serial]
async fn author_filter_strips_whitespace_before_length_check() {
	request::<App, _, _>(|request, _ctx| async move {
		let res = request
			.get("/quotes")
			.add_query_param("author", format!("{}   ", "а".repeat(200)))
			.await;
		assert_eq!(res.status_code(), 200);

		let res = request
			.get("/quotes")
			.add_query_param("author", format!("{}   ", "а".repeat(201)))
			.await;
		assert_eq!(res.status_code(), 400);
		assert!(res.json::<Value>()["errors"]["author"].is_array());
	})
	.await;
}

#[tokio::test]
#[serial]
async fn unknown_and_malformed_ids() {
	request::<App, _, _>(|request, _ctx| async move {
		let res = request
			.get("/quotes/00000000-0000-0000-0000-000000000000")
			.await;
		assert_eq!(res.status_code(), 404);

		assert_eq!(request.get("/quotes/67").await.status_code(), 400);
	})
	.await;
}

#[tokio::test]
#[serial]
async fn multiline_text_is_accepted_but_other_control_chars_are_not() {
	request::<App, _, _>(|request, _ctx| async move {
		let text = "Не знаєш, що робити?\nБереш похідну і прирівнюєш до нуля.";
		let res = request
			.post("/quotes")
			.json(&json!({"author": "Митник І.Ю.", "text": text}))
			.await;
		assert_eq!(res.status_code(), 201);
		let id = res.json::<Value>()["id"].as_str().unwrap().to_string();
		let stored = request.get(&format!("/quotes/{id}")).await.json::<Value>();
		assert_eq!(stored["text"], text);

		let res = request
			.post("/quotes")
			.json(&json!({"author": "ok", "text": "bad\u{0}char"}))
			.await;
		assert_eq!(res.status_code(), 400);
	})
	.await;
}
