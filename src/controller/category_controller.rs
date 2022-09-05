use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use crate::common::res::Res;
use crate::model::category::Category;
use crate::tag_controller::PostData;
use crate::util::token::Token;

#[get("/category/find/<name>")]
pub async fn find_by_name(name: &str) -> Value {
    match Category::find_by_name(name).await {
        Ok(data) => {
            if data.is_empty() {
                return json!(Res::none());
            }
            json!(Res::ok(data))
        }
        Err(_) => json!(Res::err())
    }
}

#[get("/category/list")]
pub async fn list() -> Value {
    match Category::find_list_all().await {
        Ok(data) => {
            if data.is_empty() {
                return json!(Res::none());
            }
            json!(Res::ok(data))
        }
        Err(_) => json!(Res::err())
    }
}

#[get("/category/list_count")]
pub async fn list_count() -> Value {
    match Category::find_list_by_count().await {
        Ok(data) => {
            if data.is_empty() {
                return json!(Res::none());
            }
            json!(Res::ok(data))
        }
        Err(_) => json!(Res::err())
    }
}

#[post("/category/create", data = "<post_data>")]
pub async fn create(post_data: Json<PostData<'_>>, _t: Token) -> Value {
    let name = post_data.name;
    match Category::create(name).await {
        Ok(data) => {
            if data.rows_affected > 0 {
                return json!(Res::ok(data));
            }
            json!(Res::none())
        }
        Err(_) => json!(Res::err())
    }
}

#[put("/category/<id>", data = "<post_data>")]
pub async fn update(id: usize, post_data: Json<PostData<'_>>, _t: Token) -> Value {
    let new_name = post_data.name;
    match Category::update(id, new_name).await {
        Ok(data) => {
            if data.rows_affected > 0 {
                return json!(Res::ok(data));
            }
            json!(Res::none())
        }
        Err(_) => json!(Res::err())
    }
}

#[delete("/category/<name>")]
pub async fn del(name: &str, _t: Token) -> Value {
    match Category::del(name).await {
        Ok(data) => {
            if data.rows_affected > 0 {
                return json!(Res::none());
            }
            json!(Res::ok(data))
        }
        Err(_) => json!(Res::err())
    }
}

