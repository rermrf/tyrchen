use crate::pb::*;
use super::{Engine, SpecTransform};
use bytes::Bytes;
use image::{ImageOutputFormat, ImageBuffer, DynamicImage};
use lazy_static::lazy_static;
use photon_rs::{PhotonImage, native::open_image_from_bytes, transform, effects, filters};

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
                Some(spec::Data::Crop(ref v)) => self.transform(v),
                Some(spec::Data::Contrast(ref v)) => todo!(),
                Some(spec::Data::Filter(ref v)) => todo!(),
                Some(spec::Data::Fliph(ref v)) => todo!(),
                Some(spec::Data::Flipv(ref v)) => todo!(),
                Some(spec::Data::Resize(ref v)) => todo!(),
                Some(spec::Data::Watermark(ref v)) => todo!(),
                _ => {},
            }
        }
    }

    fn generate(self, format: image::ImageOutputFormat) -> Vec<u8> {
        todo!()
    }
}

impl SpecTransform<&Crop> for Photon {
    fn transform(&mut self, op: &Crop) {
        let img = transform::crop(&mut self.0, op.x1, op.y1, op.x2, op.y2);
        self.0 = img;
    }
}

impl SpecTransform<&Contrast> for Photon {
    fn transform(&mut self, op: &Contrast) {
        effects::adjust_contrast(&mut self.0, op.contrast)
    }
}

impl SpecTransform<&Filter> for Photon {
    fn transform(&mut self, op: &Filter) {
        match filter::Filter::from_i32(op.filter){
            Some(filter::Filter::Unspecified) => {},
            Some(f) => filters::filter(&mut self.0, f.to_str().unwrap()),
            _ => {}
        }
    }
}

impl SpecTransform<&Flipv> for Photon {
    fn transform(&mut self, op: &Flipv) {
        transform::flipv(&mut self.0)
    }
}

impl SpecTransform<&Fliph> for Photon {
    fn transform(&mut self, op: &Fliph) {
        transform::fliph(&mut self.0)
    }
}

impl SpecTransform<&Resize> for Photon {
    fn transform(&mut self, op: &Resize) {
        let img = match resize::Resizetype::from_i32(op.rtype).unwrap() {
            resize::Resizetype::Normal => transform::resize(
                &mut self.0,
                op.width,
                op.height,
                resize::SampleFilter::try_from(op.filter).unwrap().into()
                // resize::SampleFilter::from_i32(op.filter).unwrap().into()
            ),
            resize::Resizetype::SeamCarve => {
                transform::seam_carve(&mut self.0, op.width, op.height)
            },
        };
        self.0 = img;
    }
}

// 实现在内存中对图片转换格式的方法
fn image_to_buf(img: PhotonImage, format: ImageOutputFormat) -> Vec<u8> {
    let raw_pixels = img.get_raw_pixels();
    let width = img.get_width();
    let height = img.get_height();

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = DynamicImage::ImageLuma8(img_buffer);

    let mut buffer = Vec::with_capacity(32768);
    dynimage.write_to(&mut buffer, format).unwrap();
    buffer
}