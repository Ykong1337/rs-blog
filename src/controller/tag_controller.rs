use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use crate::common::res::Res;
use crate::model::tag::Tag;
use crate::util::token::Token;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostData<'a> {
    pub name: &'a str,
}

#[get("/tag/find/<name>")]
pub async fn find_by_name(name: &str) -> Value {
    match Tag::find_by_name(name).await {
        Ok(data) => {
            if data.is_empty() {
                return json!(Res {
                    code: 400,
                    msg: "数据为空",
                    data: ()
                });
            }
            json!(Res::ok(data))
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "服务错误",
            data: ()
        })
    }
}

#[get("/tag/list")]
pub async fn list() -> Value {
    match Tag::find_list_all().await {
        Ok(data) => {
            if data.is_empty() {
                return json!(Res {
                    code: 400,
                    msg: "数据为空",
                    data: ()
                });
            }
            json!(Res::ok(data))
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "服务错误",
            data: ()
        })
    }
}

#[get("/tag/list_count")]
pub async fn find_list_count() -> Value {
    match Tag::find_list_by_count().await {
        Ok(data) => {
            if data.is_empty() {
                return json!(Res {
                    code: 400,
                    msg: "数据为空",
                    data: ()
                });
            }
            json!(Res::ok(data))
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "服务错误",
            data: ()
        })
    }
}

#[post("/tag/create", data = "<post_data>")]
pub async fn create(post_data: Json<PostData<'_>>, _t: Token) -> Value {
    let name = post_data.name;
    match Tag::create(name).await {
        Ok(data) => {
            if data.rows_affected > 0 {
                return json!(Res::ok(data));
            }
            json!(Res {
                code: 400,
                msg: "数据已存在",
                data: ()
            })
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "服务错误",
            data: ()
        })
    }
}

#[put("/tag/<id>", data = "<post_data>")]
pub async fn update(id: usize, post_data: Json<PostData<'_>>, _t: Token) -> Value {
    match Tag::update(id, post_data.name).await {
        Ok(data) => {
            if data.rows_affected > 0 {
                return json!(Res::ok(data));
            }
            json!(Res {
                code: 400,
                msg: "数据不存在",
                data: ()
            })
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "服务错误",
            data: ()
        })
    }
}

#[delete("/tag/<name>")]
pub async fn del(name: &str, _t: Token) -> Value {
    match Tag::del(name).await {
        Ok(data) => {
            if data.rows_affected > 0 {
                return json!(Res::ok(data));
            }
            json!(Res {
                code: 400,
                msg: "数据不存在",
                data: ()
            })
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "服务错误",
            data: ()
        })
    }
}