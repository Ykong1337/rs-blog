use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use crate::model::user::{User, UserVo};
use crate::common::res::Res;
use rocket::{get, post, delete};
use serde::{Serialize, Deserialize};
use crate::util::token::{create_token, Token};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostData<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginResData {
    pub username: String,
    pub token: String,
}

#[get("/user/list")]
pub async fn list() -> Value {
    let users = User::list().await;
    match users {
        Ok(data) => {
            json!(Res{
                code: 200,
                msg: "查询成功",
                data
            })
        }
        Err(_) => {
            json!(Res{
                code: 400,
                msg: "查询失败",
                data: ()
            })
        }
    }
}

#[get("/user/vo")]
pub async fn vo_list() -> Value {
    match User::vo_list().await {
        Ok(data) => {
            json!(Res{
                code: 200,
                msg: "查询成功",
                data
            })
        }
        Err(_) => {
            json!(Res{
                code: 400,
                msg: "查询失败",
                data: ()
            })
        }
    }
}

#[post("/admin/login", data = "<post_data>")]
pub async fn login(post_data: Json<PostData<'_>>) -> Value {
    let user = User::login(post_data.username, post_data.password).await;
    match user {
        Ok(t) => {
            match t {
                Some(data) => {
                    let username = data.username.unwrap();
                    let token = create_token(&username);
                    let data = LoginResData {
                        username,
                        token,
                    };
                    json!(Res{
                        code: 200,
                        msg: "登陆成功",
                        data
                    })
                }
                None => {
                    json!(Res{
                        code: 400,
                        msg: "用户名或密码错误",
                        data: ()
                    })
                }
            }
        }
        Err(_) => {
            json!(Res{
                code: 500,
                msg: "服务错误",
                data: ()
            })
        }
    }
}

#[delete("/user/<username>")]
pub async fn del(username: &str, _t: Token) -> Value {
    let res = User::del(username).await;
    match res {
        Ok(t) => {
            if t.rows_affected > 0 {
                return json!(Res{
                    code: 200,
                    msg: "删除成功",
                    data: t.rows_affected
                });
            }
            json!(Res{
                code: 400,
                msg: "不存在",
                data: ()
            })
        }
        Err(_) => {
            json!(Res{
                code: 400,
                msg: "删除失败",
                data: ()
            })
        }
    }
}