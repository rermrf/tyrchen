use std::{str::FromStr, collections::HashMap};

use clap::{Subcommand, Parser, arg};
use anyhow::{Result, anyhow, Ok};
use reqwest::{Url, Client, header::{HeaderMap, self}};
// use serde::{Serialize, Deserialize};

// 定义cli主程序
#[derive(Parser, Debug)]
#[command(author, version)]
struct Options {
    #[command(subcommand)]
    subcmd: SubCommand,
}

// 定义子命令
#[derive(Subcommand, Debug)]
enum SubCommand{
    Get(Get),
    Post(Post),
}

// get子命令
#[derive(Parser, Debug)]
struct Get{
    // HTTP请求的URL
    // 为参数添加验证函数
    #[arg(value_parser = parse_url)]
    url: String,
}

// get函数具体执行
async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    println!("{}", resp.text().await?);
    Ok(())
}

// post子命令
#[derive(Parser, Debug)]
struct Post{
    // HTTP请求的URL
    // 为参数添加验证函数
    #[arg(value_parser = parse_url)]
    url: String,
    // HTTP请求的body
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body: HashMap<String, String> = HashMap::new();
    for pair in args.body.iter() {
        body.insert(pair.key.clone(), pair.value.clone());
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    println!("{}", resp.text().await?);
    Ok(())
}

// 验证输入的url是否合法
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse().expect("url 格式错误");
    Ok(s.into())
}

// 命令行中的 key=value，通过 parse_kv_pair 解析
#[derive(Debug, Clone)]
// #[derive(Serialize, Deserialize)]
struct KvPair {
    key: String,
    value: String,
}

// 当我们实现了FromStr trait，可以用 str.sparse()方法将字符串解析成KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行split，这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self { 
            // 从迭代器中获取第一个值为key，迭代器返回Some(T)/None
            key: (split.next().ok_or_else(err)?.to_string()), 
            // 从迭代器中获取第二个值为value
            value: (split.next().ok_or_else(err)?).to_string(), 
        })
    }
}

// 为post子命令需要的arg body添加解析函数
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Options::parse();

    // 生成一个HTTP客户端
    let mut headers = HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "rermrf".parse()?);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => {
            get(client, args).await?
        },
        SubCommand::Post(ref args) => {
            post(client, args).await?
        }, 
    };

    Ok(result)
}
