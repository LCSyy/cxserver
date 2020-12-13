use serde::Serialize;

/// 用于返回数据
#[derive(Default, Serialize)]
pub struct ResBody<T> 
    where T: Serialize
{
    pub err: Option<String>,
    pub data: Option<T>,
}
