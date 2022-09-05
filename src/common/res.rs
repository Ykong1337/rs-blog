#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Res<'a, T> {
    pub code: u32,
    pub msg: &'a str,
    pub data: T,
}

impl<T: 'static> Res<'_, T> {
    pub fn ok(data: T) -> Res<'static, T> {
        Res {
            code: 200,
            msg: "操作成功",
            data,
        }
    }
}