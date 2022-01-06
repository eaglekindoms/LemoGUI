/// 图像数据结构体
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ImageRaw {
    /// 图像宽度
    pub width: u32,
    /// 图像高度
    pub height: u32,
    /// 图像RGBA值
    pub data: Vec<u8>,
}