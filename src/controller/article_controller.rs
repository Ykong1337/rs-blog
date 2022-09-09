use rbatis::rbdc::datetime::FastDateTime;
use rocket::form::FromForm;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use crate::common::res::{Res, Resp};
use crate::model::article::{Article, ArticleForUpdateVo, ArticleVO};
use crate::util::token::Token;

#[derive(Debug, PartialEq, FromForm)]
pub struct Page {
    page: usize,
    per_page: usize,
}

#[get("/articles?<page..>")]
pub async fn list(page: Page) -> Value {
    let page_size = page.per_page;
    let current_page = page.page;
    let total = Article::find_total().await.unwrap();

    let per_page = page.per_page;
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

#[get("/article/category")]
pub async fn list_with_category() -> Value {
    let res = Article::find_all_with_category().await;
    match res {
        Ok(data) => {
            if data.is_empty() {
                return json!(Res {
                    code: 400,
                    msg: "null",
                    data: ()
                });
            }
            json!(Res::ok(data))
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "error",
            data: ()
        })
    }
}

#[get("/article/editing/<id>")]
pub async fn editing(id: usize) -> Value {
    let res = Article::find_editing_by_id(id).await;
    if let Ok(t) = res {
        return match t {
            Some(data) => json!(Res::ok(data)),
            None => json!(Res {
                code: 400,
                msg: "null",
                data: ()
            })
        };
    }
    json!(Res {
        code: 500,
        msg: "error",
        data: ()
    })
}

#[get("/article/category/<id>")]
pub async fn with_category(id: usize) -> Value {
    let res = Article::find_by_id_with_category(id).await;
    if let Ok(t) = res {
        return match t {
            Some(data) => json!(Res::ok(data)),
            None => json!(Res {
                code: 400,
                msg: "null",
                data: ()
            })
        };
    }
    json!(Res {
        code: 500,
        msg: "error",
        data: ()
    })
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

#[delete("/article/<id>")]
pub async fn delete(id: usize, _t: Token) -> Value {
    let res = Article::remove(id).await;
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

#[put("/article/<id>", data = "<put_data>")]
pub async fn update(id: usize, put_data: Json<PostOrPutArticleData<'_>>, _t: Token) -> Value {
    let art = ArticleForUpdateVo {
        title: put_data.title.to_string(),
        description: put_data.description.to_string(),
        content: put_data.content.to_string(),
        cate_id: put_data.cate_id,
        tags: put_data.tags.clone(),
    };

    let res = Article::update(id, art).await;
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

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct SearchData<'a> {
    title: &'a str,
    category: usize,
}

#[get("/article/search?<data>")]
pub async fn search(data: SearchData<'_>) -> Value {
    let title = data.title;
    let category = data.category;
    let res = Article::search(title, category).await;
    match res {
        Ok(data) => {
            if data.is_empty() {
                return json!(Res {
                    code: 400,
                    msg: "null",
                    data: ()
                });
            }
            json!(Res::ok(data))
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "error",
            data: ()
        })
    }
}

#[get("/article/search/<word>")]
pub async fn home_search(word: &str) -> Value {
    let res = Article::home_search(word).await;
    match res {
        Ok(data) => {
            if data.is_empty() {
                return json!(Res {
                    code: 400,
                    msg: "null",
                    data: ()
                });
            }
            json!(Res::ok(data))
        }
        Err(_) => json!(Res {
            code: 500,
            msg: "error",
            data: ()
        })
    }
}
