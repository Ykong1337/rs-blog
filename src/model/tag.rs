use rbatis::rbdc::datetime::FastDateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use rbs::Value::U64;
use crate::RB;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<usize>,
    pub name: Option<String>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagVo {
    pub id: Option<usize>,
    pub name: Option<String>,
    pub blog_count: Option<usize>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
}

crud!(Tag {});
impl_select!(Tag {select_by_name(name: &str) => "`where name like '%${name}%'`"});
impl_select!(Tag {select_by_id(id: usize) -> Option => "`where id = #{id}`"});
impl_update!(Tag {update_by_id(id: usize) => "`where id = #{id}`"});

impl Tag {
    pub async fn find_list_all() -> Result<Vec<Tag>, Error> {
        Tag::select_all(&mut RB.clone()).await
    }

    pub async fn find_by_name(name: &str) -> Result<Vec<Tag>, Error> {
        Tag::select_by_name(&mut RB.clone(), name).await
    }

    pub async fn find_list_by_count() -> Result<Vec<TagVo>, Error> {
        RB.fetch_decode("select id, name, created_at, updated_at, (select count(*) from article inner join article_to_tag on article.id = article_to_tag.article_id where tag.id = article_to_tag.tag_id) as blog_count from tag", vec![]).await
    }

    // pub async fn find_one_count(id: usize) -> Result<Vec<Article>, Error> {
    //     RB.fetch_decode("SELECT a.* FROM article a WHERE a.id IN (SELECT at2.article_id FROM article_tag at2 WHERE at2.tag_id = ?)", vec![U64(id as u64)]).await
    // }

    pub async fn create(name: &str) -> Result<ExecResult, Error> {
        let tag = Tag {
            id: None,
            name: Some(name.to_string()),
            created_at: Some(FastDateTime::now()),
            updated_at: Some(FastDateTime::now()),
        };
        Tag::insert(&mut RB.clone(), &tag).await
    }

    pub async fn update(id: usize, new_name: &str) -> Result<ExecResult, Error> {
        let old_tag = Self::select_by_id(&mut RB.clone(), id).await;
        match old_tag {
            Ok(t) => {
                match t {
                    Some(tag) => {
                        let new_tag = Tag {
                            name: Some(new_name.to_string()),
                            updated_at: Some(FastDateTime::now()),
                            ..tag
                        };
                        Tag::update_by_id(&mut RB.clone(), &new_tag, id).await
                    }
                    None => Ok(ExecResult{
                        rows_affected: 0,
                        last_insert_id: Default::default()
                    })
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn del(name: &str) -> Result<ExecResult, Error> {
        Tag::delete_by_column(&mut RB.clone(), "name", name).await
    }
}