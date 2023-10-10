use prost::Message;
use base64::{engine::general_purpose, Engine};

// 声明abi.rs
mod abi;
pub use abi::*;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

// 让ImageSpec 可以生成一个字符串
impl From<&ImageSpec> for String {
    fn from(value: &ImageSpec) -> Self {
        let data = value.encode_to_vec();
        // 返回encode的字符串
        general_purpose::URL_SAFE_NO_PAD.encode(data)
    }
}