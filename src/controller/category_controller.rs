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
                return json!(Res {
                    code: 400,
                    msg: "数据为空",
                    data: ()
                });
            }
            json!(Res {
                code: 200,
                msg: "查询成功",
                data
            })
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "服务错误",
            data: ()
        })
    }
}

#[get("/category/list")]
pub async fn list() -> Value {
    match Category::find_list_all().await {
        Ok(data) => {
            if data.is_empty() {
                return json!(Res {
                    code: 400,
                    msg: "数据为空",
                    data: ()
                });
            }
            json!(Res {
                code: 200,
                msg: "查询成功",
                data
            })
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "服务错误",
            data: ()
        })
    }
}

#[get("/category/list_count")]
pub async fn list_count() -> Value {
    match Category::find_list_by_count().await {
        Ok(data) => {
            if data.is_empty() {
                return json!(Res {
                    code: 400,
                    msg: "数据为空",
                    data: ()
                });
            }
            json!(Res {
                code: 200,
                msg: "查询成功",
                data
            })
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "服务错误",
            data: ()
        })
    }
}

#[post("/category/create", data = "<post_data>")]
pub async fn create(post_data: Json<PostData<'_>>, _t: Token) -> Value {
    let name = post_data.name;
    match Category::create(name).await {
        Ok(data) => {
            if data.rows_affected > 0 {
                return json!(Res {
                    code: 200,
                    msg: "操作成功",
                    data
                });
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

#[put("/category/<id>", data = "<post_data>")]
pub async fn update(id: usize, post_data: Json<PostData<'_>>, _t: Token) -> Value {
    let new_name = post_data.name;
    match Category::update(id, new_name).await {
        Ok(data) => {
            if data.rows_affected > 0 {
                return json!(Res {
                    code: 200,
                    msg: "操作成功",
                    data
                });
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

#[delete("/category/<name>")]
pub async fn del(name: &str, _t: Token) -> Value {
    match Category::del(name).await {
        Ok(data) => {
            if data.rows_affected > 0 {
                return json!(Res {
                    code: 200,
                    msg: "操作成功",
                    data
                });
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

