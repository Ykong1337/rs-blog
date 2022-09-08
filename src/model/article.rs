use std::thread::sleep;
use std::time::Duration;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::rbdc::Error;
use rbs::to_value;
use crate::RB;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Option<usize>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub cate_id: Option<usize>,
    pub istop: Option<usize>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleVO {
    pub id: Option<usize>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub cate_id: Option<usize>,
    pub cate_name: Option<String>,
    pub istop: Option<usize>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleEditVo {
    pub id: Option<usize>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub cate_id: Option<usize>,
    pub tags: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleForUpdateVo {
    pub title: String,
    pub description: String,
    pub content: String,
    pub cate_id: usize,
    pub tags: Vec<usize>,
}
crud!(Article {});
impl_select!(Article {select_by_id(id: usize) -> Option => "`where id = #{id}`"});
impl_select!(Article {select_hot() => "`limit 5`"});

impl Article {
    pub async fn find_total() -> Result<usize, Error> {
        RB.fetch_decode("SELECT COUNT(id) FROM article", vec![]).await
    }

    pub async fn find_all_page_with_category(page: &str, per_page: &str) -> Result<Vec<ArticleVO>, Error> {
        RB.fetch_decode("SELECT a.*,c.name as cate_name FROM article a,category c WHERE a.cate_id = c.id LIMIT ?,?", vec![to_value!(page), to_value!(per_page)]).await
    }

    pub async fn find_all_with_category() -> Result<Vec<ArticleVO>, Error> {
        RB.fetch_decode("SELECT a.*,c.name as cate_name FROM article a,category c WHERE a.cate_id = c.id", vec![]).await
    }

    pub async fn find_by_id(id: usize) -> Result<Option<Article>, Error> {
        Self::select_by_id(&mut RB.clone(), id).await
    }

    pub async fn find_editing_by_id(id: usize) -> Result<Option<Article>, Error> {
        RB.fetch_decode("SELECT a.id,a.title,a.description,a.content,a.cate_id,GROUP_CONCAT(att.tag_id) as tags FROM article a LEFT JOIN article_to_tag att ON a.id = att.article_id WHERE a.id = ? GROUP BY a.id;", vec![to_value!(id)]).await
    }

    pub async fn find_by_id_with_category(id: usize) -> Result<Option<Article>, Error> {
        RB.fetch_decode("SELECT a.*,c.name as cate_name FROM article a ,category c WHERE a.cate_id = c.id AND a.id = ?", vec![to_value!(id)]).await
    }

    pub async fn find_hot() -> Result<Vec<Article>, Error> {
        Self::select_hot(&mut RB.clone()).await
    }

    pub async fn add_article(article: Article, tag_ids: Vec<usize>) -> Result<(), Error> {
        let tx = RB.acquire_begin().await.unwrap();
        let mut tx = tx.defer_async(|mut tx| async move {
            if !tx.done {
                tx.rollback().await.unwrap();
                println!("rollback")
            }
        });

        let art_res = Self::insert(&mut tx, &article).await;
        match art_res {
            Ok(res) => {
                let new_art_id = res.last_insert_id;
                for i in tag_ids {
                    tx.exec("INSERT INTO article_to_tag (article_id,tag_id) VALUE (?,?);",
                            vec![to_value!(new_art_id.as_u64()), to_value!(i)]).await?;
                }
            }
            Err(_) => {
                tx.rollback().await.unwrap();
                println!("rollback")
            }
        }
        tx.commit().await.unwrap();
        println!("commit");
        drop(tx);
        sleep(Duration::from_secs(1));
        Ok(())
    }

    pub async fn remove(id: usize) -> Result<(), Error> {
        let tx = RB.acquire_begin().await.unwrap();
        let mut tx = tx.defer_async(|mut tx| async move {
            if !tx.done {
                tx.rollback().await.unwrap();
                println!("rollback")
            }
        });
        let art = Self::find_by_id(id).await;
        match art {
            Ok(t) => {
                match t {
                    Some(data) => {
                        tx.exec("DELETE FROM article_to_tag WHERE article_id = ?;",
                                vec![to_value!(data.id)]).await?;
                        tx.exec("DELETE FROM article WHERE id = ?;",
                                vec![to_value!(id)]).await?;
                    }
                    None => return Err(Error::E("文章不存在".to_string()))
                }
            }
            Err(_) => {
                tx.rollback().await.unwrap();
                println!("rollback")
            }
        }
        tx.commit().await.unwrap();
        println!("commit");
        drop(tx);
        sleep(Duration::from_secs(1));
        Ok(())
    }

    pub async fn update(id: usize, put_art: ArticleForUpdateVo) -> Result<(), Error> {
        let tx = RB.acquire_begin().await.unwrap();
        let mut tx = tx.defer_async(|mut tx| async move {
            if !tx.done {
                tx.rollback().await.unwrap();
                println!("rollback")
            }
        });

        let old_art = Self::find_by_id(id).await;
        match old_art {
            Ok(t) => {
                match t {
                    Some(data) => {
                        let tags = put_art.tags;

                        tx.exec("DELETE FROM article_to_tag WHERE article_id = ?;",
                                vec![to_value!(data.id)]).await?;
                        for i in tags {
                            tx.exec("INSERT INTO article_to_tag VALUES (?,?);",
                                    vec![to_value!(data.id), to_value!(i)]).await?;
                        }
                        tx.exec("UPDATE article SET title = ?,description = ?,content = ?,cate_id = ?,updated_at = ? WHERE id = ?;",
                                vec![to_value!(put_art.title), to_value!(put_art.description), to_value!(put_art.content), to_value!(put_art.cate_id), to_value!(FastDateTime::now()), to_value!(data.id)]).await?;
                    }
                    None => return Err(Error::E("分类不存在".to_string()))
                }
            }
            Err(_) => {
                tx.rollback().await.unwrap();
                println!("rollback")
            }
        }
        tx.commit().await.unwrap();
        println!("commit");
        drop(tx);
        sleep(Duration::from_secs(1));
        Ok(())
    }
}
