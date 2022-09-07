use rbatis::rbdc::datetime::FastDateTime;
use rocket::form::FromForm;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use crate::common::res::{Res, Resp};
use crate::model::article::{Article, ArticleVO};
use crate::util::token::Token;

#[derive(Debug, PartialEq, FromForm)]
pub struct Page {
    page: usize,
    perPage: usize,
}

#[get("/articles?<page..>")]
pub async fn list(page: Page) -> Value {
    let page_size = page.perPage;
    let current_page = page.page;
    let total = Article::find_total().await.unwrap();

    let per_page = page.perPage;
    let page = (page.page - 1) * per_page;
    let articles = Article::find_all_page_with_category(&page.to_string(), &per_page.to_string()).await;

    match articles {
        Ok(data) => {
            let data = Resp::<Vec<ArticleVO>> {
                code: 200,
                msg: "success",
                data,
                current_page,
                page_size,
                total,
            };
            json!(data)
        }
        Err(_) => json!(Res{
            code: 500,
            msg: "服务错误",
            data: ()
        })
    }
}

#[get("/article/<id>")]
pub async fn detail(id: usize) -> Value {
    let article = Article::find_by_id(id).await;
    match article {
        Ok(t) => {
            match t {
                Some(data) => json!(Res::ok(data)),
                None => json!(Res {
                    code: 400,
                    msg: "数据不存在",
                    data: ()
                })
            }
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "error",
            data: ()
        })
    }
}

#[get("/article/hot")]
pub async fn hot() -> Value {
    let hots = Article::find_hot().await;
    match hots {
        Ok(data) => json!(Res::ok(data)),
        Err(_) => json!(Res {
            code: 500,
            msg: "error",
            data: ()
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostOrPutArticleData<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub cate_id: usize,
    pub content: &'a str,
    pub tags: Vec<usize>,
}

#[post("/article", data = "<post_data>")]
pub async fn create(post_data: Json<PostOrPutArticleData<'_>>, _t: Token) -> Value {
    let art = Article {
        id: None,
        title: Some(post_data.title.to_string()),
        description: Some(post_data.description.to_string()),
        content: Some(post_data.content.to_string()),
        cate_id: Some(post_data.cate_id),
        istop: Some(0),
        created_at: Some(FastDateTime::now()),
        updated_at: Some(FastDateTime::now()),
    };
    let tag_ids: Vec<usize> = post_data.tags.clone();
    let res = Article::add_article(art, tag_ids).await;
    match res {
        Ok(_) => json!(Res {
            code: 200,
            msg: "success",
            data: ()
        }),
        Err(_) => json!(Res {
            code: 500,
            msg: "error",
            data: ()
        })
    }
}
