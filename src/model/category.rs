use rbatis::rbdc::datetime::FastDateTime;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use crate::RB;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<usize>,
    pub name: Option<String>,
    pub create_at: Option<FastDateTime>,
    pub update_at: Option<FastDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryVo {
    pub id: Option<usize>,
    pub name: Option<String>,
    pub create_at: Option<FastDateTime>,
    pub update_at: Option<FastDateTime>,
    pub blog_count: Option<usize>,
}
crud!(Category {});
impl_select!(Category {select_by_name(name: &str) => "`where name like '%${name}%'`"});
impl_select!(Category {select_by_id(id: usize) -> Option => "`where id = #{id}`"});
impl_update!(Category {update_by_id(id: usize) => "`where id = #{id}`"});

impl Category {
    pub async fn find_list_all() -> Result<Vec<Category>, Error> {
        Category::select_all(&mut RB.clone()).await
    }

    pub async fn find_by_name(name: &str) -> Result<Vec<Category>, Error> {
        Category::select_by_name(&mut RB.clone(), name).await
    }

    pub async fn find_list_by_count() -> Result<Vec<CategoryVo>, Error> {
        RB.fetch_decode("select id,name,created_at,updated_at, (select count(*) from article where category.id = article.cate_id) as blog_count from category", vec![]).await
    }


    pub async fn create(name: &str) -> Result<ExecResult, Error> {
        let category = Category {
            id: None,
            name: Some(name.to_string()),
            create_at: Some(FastDateTime::now()),
            update_at: Some(FastDateTime::now()),
        };
        Category::insert(&mut RB.clone(), &category).await
    }

    pub async fn update(id: usize, new_name: &str) -> Result<ExecResult, Error> {
        let old_cate = Self::select_by_id(&mut RB.clone(), id).await;
        match old_cate {
            Ok(t) => {
                match t {
                    Some(category) => {
                        let new_category = Category {
                            name: Some(new_name.to_string()),
                            update_at: Some(FastDateTime::now()),
                            ..category
                        };
                        Category::update_by_id(&mut RB.clone(), &new_category, id).await
                    }
                    None => Ok(ExecResult {
                        rows_affected: 0,
                        last_insert_id: Default::default(),
                    })
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn del(name: &str) -> Result<ExecResult, Error> {
        Category::delete_by_column(&mut RB.clone(), "name", name).await
    }
}
