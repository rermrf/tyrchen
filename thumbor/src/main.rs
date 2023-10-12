use std::{num::NonZeroUsize, sync::Arc, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use axum::{extract::Path, routing::get, Router, Extension, http::{HeaderMap, HeaderValue}};
use bytes::Bytes;
use lru::LruCache;
use pb::ImageSpec;
use percent_encoding::NON_ALPHANUMERIC;
use reqwest::StatusCode;
use serde::Deserialize;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tracing::{instrument, info};
use crate::pb::{Spec, resize, filter};
use anyhow::Result;

// 引入 protobuf 生成的代码
mod pb;
mod engine;
use engine::{Engine, Photon};
use image::ImageOutputFormat;

// 参数使用 serde 做 Deserialize, axum 会自动识别并解析
#[derive(Deserialize)]
struct Params {
    // 对图像处理的参数
    spec: String,
    // 网络图片的url
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();

    let cache: Cache = Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(1024).unwrap())));

    // 构建路由
    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(cache))
                .into_inner(),
        );

    // 运行服务器
    let addr = "127.0.0.1:3000".parse().unwrap();

    print_test_url("https://images.pexels.com/photos/4327575/pexels-photo-4327575.jpeg?auto=compress&cs=tinysrgb&w=1600&lazy=load");

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 参数解析
async fn generate(
    Path(Params { spec, url }): Path<Params>,
    Extension(cache): Extension<Cache>,
) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let url = percent_encoding::percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let data = retrieve_image(&url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // TODO: 处理图片
    // 使用 image engine 处理
    let mut engine: Photon = data
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    engine.apply(&spec.specs);

    let image = engine.generate(ImageOutputFormat::Jpeg(85));

    info!("Finished processing: image size {}", image.len());

    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));
    Ok((headers, image))
}

#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match cache {}", key);
            v.to_owned()
        },
        None => {
            info!("Retrieve url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        },
    };
    Ok(data)
}

// 调试辅助函数
fn print_test_url(url: &str) {
    use std::borrow::Borrow;
    let spec1 = Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20, 20);
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
    let s: String = image_spec.borrow().into();
    let test_image = percent_encoding::percent_encode(url.as_bytes(), NON_ALPHANUMERIC);
    println!("test url: http://localhost:3000/image/{s}/{test_image}");
}