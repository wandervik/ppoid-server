use std::{env, io};
use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer, HttpResponse, get, post, web::Path};
use serde::{Serialize, Deserialize};

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, model::AttributeValue};
use uuid::Uuid;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(list)
            .service(add)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}

#[derive(Debug, Deserialize, Serialize)]
struct Score {
    player: String,
    score: u64,
}

async fn get_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}

#[get("/score")]
async fn list() -> HttpResponse {
    let client = get_client().await;

    let resp = client
        .query()
        .table_name("score_board")
        .index_name("game_index")
        .key_condition_expression("game = :ppoid")
        .expression_attribute_values(":ppoid", AttributeValue::S("ppoid".to_string()))
        .scan_index_forward(false)
        .limit(10)
        .projection_expression("player, score")
        .send()
        .await;

    match resp {
        Ok(items) => match items.items() {
            Some(itms) => {
                HttpResponse::Ok()
                    .content_type("application/json")
                    .json(itms.into_iter().map(|itm| {
                        Score {
                            player: itm.get("player").unwrap().as_s().unwrap().to_owned(), 
                            score: itm.get("score").unwrap().as_n().unwrap().to_owned().parse().unwrap()
                        }
                    })
                    .collect::<Vec<_>>()
                )
            },
            _ => HttpResponse::Forbidden().finish()
        },
        _ => HttpResponse::Forbidden().finish()
    }
}

#[post("/score/{player}/{score}")]
async fn add(path: Path<(String, u64)>) -> HttpResponse {
    let client = get_client().await;

    match client.put_item()
        .table_name("score_board")
        .item("id", AttributeValue::S(Uuid::new_v4().to_string()))
        .item("player", AttributeValue::S(path.0.clone()))
        .item("score", AttributeValue::N(path.1.to_string()))
        .item("game", AttributeValue::S("ppoid".to_string()))
        .send()
        .await {
        Ok(_result) => {
            HttpResponse::Created()
                .content_type("application/json")
                .json(Score{player: path.0.clone(), score: path.1})
        },
        Err(_) => {
            HttpResponse::Forbidden().finish()
        }
    }
}
