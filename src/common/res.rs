#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Res<'a, T> {
    pub code: u32,
    pub msg: &'a str,
    pub data: T,
}

impl<T> Res<'_, T> {
    pub fn ok(data: T) -> Self {
        Res {
            code: 200,
            msg: "操作成功",
            data,
        }
    }

    pub fn none(_t: T) -> Self {
        Res {
            code: 400,
            msg: "数据为空或已存在",
            data: (),
        }
    }

    pub fn err(_t: T) -> Self {
        Res {
            code: 500,
            msg: "服务错误",
            data: (),
        }
    }
}