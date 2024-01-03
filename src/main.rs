/*
    Copyright (c)  Abraão Isvi <avraamisvi@users.noreply.github.com>

    Permission is hereby granted, free of charge, to any
    person obtaining a copy of this software and associated
    documentation files (the "Software"), to deal in the
    Software without restriction, including without
    limitation the rights to use, copy, modify, merge,
    publish, distribute, sublicense, and/or sell copies of
    the Software, and to permit persons to whom the Software
    is furnished to do so, subject to the following
    conditions:

    The above copyright notice and this permission notice
    shall be included in all copies or substantial portions
    of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
    ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
    TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
    PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
    SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
    CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
    OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
    IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
 */

#[macro_use] extern crate rocket;

use config::read_config;
use futures::executor;
use rocket_cors::{AllowedOrigins, AllowedHeaders, Method};
use routes::{post_collections, get_collections, get_collections_feature, 
    get_collections_features, post_feature, put_collections, options_collections,
    get_features_by_bbox, put_feature};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Error};

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

mod data;
mod model;
mod services;
mod repository;
mod config;
mod routes;

#[launch]
fn rocket() -> _ {

    // let allowed_origins = AllowedOrigins::All;

    // // You can also deserialize this
    // let cors = rocket_cors::CorsOptions {
    //     allowed_origins,
    //     allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
    //     allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
    //     allow_credentials: true,
    //     ..Default::default()
    // }
    // .to_cors().unwrap();

    let pool: Pool<Postgres> = executor::block_on(async {
        create_pool().await
    }).unwrap();

    rocket::build()
    .manage(pool)
    .mount("/", routes![options_collections,
                        post_collections, 
                        post_feature,
                        put_collections,
                        put_feature,
                        get_collections_feature,
                        get_features_by_bbox,
                        get_collections, 
                        get_collections_features])
}

async fn create_pool() -> Result<Pool<Postgres>, Error> {
    let configuration = read_config().unwrap();

    PgPoolOptions::new()
    .max_connections(5)
    .connect(&configuration.get_database_url()).await
}

// pub struct CORS;

// #[rocket::async_trait]
// impl Fairing for CORS {
//     fn info(&self) -> Info {
//         Info {
//             name: "Add CORS headers to responses",
//             kind: Kind::Response
//         }
//     }

//     async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
//         response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
//         response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
//         response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
//         response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
//     }
// }