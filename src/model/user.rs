use rbatis::rbdc::Error;
use rbatis::rbdc::db::ExecResult;
use crate::RB;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<usize>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVo {
    pub id: Option<usize>,
    pub username: Option<String>,
    pub nickname: Option<String>,
}

crud!(User {});
impl_select!(User {select_username_password(username: &str, password: &str) -> Option => "`where username = #{username} and password = #{password}`"});
impl_select!(User {vo_list_all(table_column: &str) -> Vec => ""});

pub fn encode(password: &str) -> String {
    let digest = md5::compute(password);
    format!("{:x}", digest)
}

impl User {
    pub async fn login(username: &str, password: &str) -> Result<Option<User>, Error> {
        User::select_username_password(&mut RB.clone(), username, encode(password).as_str()).await
    }

    pub async fn list() -> Result<Vec<User>, Error> {
        User::select_all(&mut RB.clone()).await
    }

    pub async fn del(username: &str) -> Result<ExecResult, Error> {
        User::delete_by_column(&mut RB.clone(), "username", username).await
    }

    pub async fn vo_list() -> Result<Vec<User>, Error> {
        User::vo_list_all(&mut RB.clone(), "id, username, nickname").await
    }
}
