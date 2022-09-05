use rocket::serde::json::{Json, Value};
use rocket::serde::json::serde_json::json;
use crate::model::user::{User, UserVo};
use crate::common::res::Res;
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
    let users = User::find_list().await;
    match users {
        Ok(data) => {
            json!(Res::ok(data))
        }
        Err(_) => json!(Res::err())
    }
}

#[get("/user/vo")]
pub async fn vo_list() -> Value {
    match User::find_list().await {
        Ok(data) => {
            let mut vec = vec![];
            for i in data {
                let vo = UserVo {
                    id: i.id,
                    username: i.username,
                    nickname: i.nickname,
                };
                vec.push(vo);
            }
            json!(Res::ok(data))
        }
        Err(_) => json!(Res::err())
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
                    json!(Res::ok(data))
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
        Err(_) => json!(Res::err())
    }
}

#[delete("/user/<username>")]
pub async fn del(username: &str, _t: Token) -> Value {
    let res = User::del(username).await;
    match res {
        Ok(data) => {
            if data.rows_affected > 0 {
                return json!(Res::ok(data));
            }
            json!(Res::none())
        }
        Err(_) => json!(Res::err())
    }
}