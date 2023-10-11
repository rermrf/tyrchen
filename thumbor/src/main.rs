use axum::{Router, routing::get, extract::Path};
use pb::ImageSpec;
use reqwest::StatusCode;
use serde::Deserialize;
// use anyhow::{Result};


// 引入 protobuf 生成的代码
mod pb;

// 参数使用 serde 做 Deserialize, axum 会自动识别并解析
#[derive(Deserialize)] 
struct Params {
    spec: String,
    url: String,
}

#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();

    // 构建路由
    let app = Router::new()
        .route("/image/:spec/:url", get(generate));
    
    // 运行服务器
    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 参数解析
async fn generate(Path(Params { spec, url}): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_encoding::percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {url}, \n spec: {spec:?}"))
}