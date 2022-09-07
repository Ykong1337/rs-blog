#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Res<'a, T> {
    pub code: u32,
    pub msg: &'a str,
    pub data: T,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resp<'a, T> {
    pub code: u32,
    pub msg: &'a str,
    pub data: T,
    pub current_page: usize,
    pub page_size: usize,
    pub total: usize,
}

impl<T: 'static> Res<'_, T> {
    pub fn ok(data: T) -> Res<'static, T> {
        Res {
            code: 200,
            msg: "success",
            data,
        }
    }
}