use rbatis::rbdc::datetime::FastDateTime;
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


impl Category {
    pub async fn find_list_all() -> Result<Vec<Category>, Error> {
        Category::select_all(&mut RB.clone()).await
    }

    pub async fn find_by_name(name: &str) -> Result<Vec<Category>, Error> {
        Category::select_by_name(&mut RB.clone(), name).await
    }


}