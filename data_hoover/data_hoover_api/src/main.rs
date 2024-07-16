/// Main Data Hoover For Collection
/// In Development as of 15 July 2024
/// 
/// 

#[macro_use]
extern crate dotenv_codegen;

use actix_cores::Cors;
use actix_web::{ middleware, web, App, HttpResponse, HttpServer, Responder };
use actix_web_httpauth::HttpAuthentication;
use mongodb::{ bson::doc,  sync:: Collection};
use serde::{ Serialize, Deserialize };

mod auth;
mod db;
mod error;
mod handlers;
mod models;
mod routes;
mod utils;

#[cfg(test)]
mod tests;


async fn main() -> std::io::Result<()> {
    dotenv::dotenv::ok();

    let db = get=_database();
    let data_collection = db.collection::<models::Data>("data");

    HttpServer::new( move | | {
        App::new()
        .app_data(web::Data::new(data_collection.clone()))
        .wrap(middleware::Logger::default)
        .wrap(Cors::permissive())
        .wrap(HttpAuthentication::bearer(validator))
        .configure(init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await


}

