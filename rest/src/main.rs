use std::env;

use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use actix_web::client::{Client, Connector};
use openssl::ssl::{SslConnector, SslMethod};
use serde::{Deserialize};


#[derive(Debug, Deserialize)]
struct Block {
    jsonrpc: String,
    id: u32,
    result: String,
}

#[derive(Debug, Deserialize)]
struct BlockReward {
    status: String,
    message: String,
    result: BlockRewardResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockRewardResult {
    block_number: String,
    time_stamp: String,
    block_miner: String,
    block_reward: String,
    uncles: Vec<Uncle>,
    uncle_inclusion_reward: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Uncle {
    miner: String,
    uncle_position: String,
    blockreward: String,
}


static API: &str = "https://api.etherscan.io/api";

fn get_api_key() -> String {
    match env::var("API_KEY") {
        Ok(val) => val,
        Err(e) => panic!("can't do {}", e),
    }
}

async fn current_block_time(_req: HttpRequest) -> impl Responder {

    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    let client = Client::builder()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    let block: Block = client.get(format!("{}{}{}", API, "?module=proxy&action=eth_blockNumber&apikey=", get_api_key()))
        .header("User-Agent", "Actix-web")
        .send()
        .await
        .unwrap()
        .json::<Block>()
        .await
        .unwrap();

    let block_number = u32::from_str_radix(block.result.strip_prefix("0x").unwrap(), 16).unwrap();

    let block_reward: BlockReward = client.get(format!("{}{}{}{}{}", API, "?module=block&action=getblockreward&blockno=", block_number, "&apikey=", get_api_key()))
        .header("User-Agent", "Actix-web")
        .send()
        .await
        .unwrap()
        .json::<BlockReward>()
        .await
        .unwrap();

    let newest_block_time = block_reward.result.time_stamp;

    HttpResponse::Ok().body(format!("{:?}", newest_block_time))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/currentBlockTime", web::get().to(current_block_time))
    })
    .bind(("0.0.0.0", 11111))?
    .run()
    .await
}
