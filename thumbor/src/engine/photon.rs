use crate::pb::spec;

use super::Engine;
use bytes::Bytes;
use lazy_static::lazy_static;
use photon_rs::{PhotonImage, native::open_image_from_bytes, transform};

lazy_static!{
    // 预先把水印文件加载为静态变量
    static ref WATERMARK: PhotonImage = {
        // 在编译的时候 include_bytes! 宏会直接把文件读入编译后的二进制
        let data = include_bytes!("../../assets/rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

// 我们目前支持 Photon engine
pub struct Photon(PhotonImage);

// 从 Bytes 转换成 Photon 结构
impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&value)?))
    }
}

impl Engine for Photon{
    fn apply(&mut self, specs: &[crate::pb::Spec]) {
        for spec in specs.iter() {
            match spec.data {
                Some(spec::Data::Crop(ref v)) => todo!(),
                _ => {},
            }
        }
    }

    fn generate(self, format: image::ImageOutputFormat) -> Vec<u8> {
        todo!()
    }
}