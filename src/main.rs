use actix_web::{
    App, Error, HttpResponse, HttpServer,
    web::{self, ThinData},
};

use confik::{Configuration as _, EnvSource};
use deadpool_postgres::{Client, Pool};
use dotenvy::dotenv;
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;

mod config;
mod db;
mod errors;
mod models;

use self::{errors::MyError, models::{User, PayloadUser}};

pub async fn get_users(ThinData(db_pool): ThinData<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let users = db::get_users(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_user(
    user: web::Json<User>,
    ThinData(db_pool): ThinData<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = db::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

pub async fn update_user(
    path: web::Path<PayloadUser>,
    user: web::Json<User>,
    ThinData(db_pool): ThinData<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();
    let payload: PayloadUser = path.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = db::update_user(&client, user_info, payload).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

pub async fn delete_user(
    path: web::Path<PayloadUser>,
    // path: web::Path<String>,
    ThinData(db_pool): ThinData<Pool>,
) -> Result<HttpResponse, Error> {
    let payload: PayloadUser = path.into_inner();
    // let payload: String = path.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let deleted_user = db::delete_user(&client, payload).await?;

    Ok(HttpResponse::Ok().json(deleted_user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = ExampleConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(ThinData(pool.clone()))
            .service(
                web::resource("/users")
                    .route(web::post().to(add_user))
                    .route(web::get().to(get_users))
            )
            .service(
                web::resource("/user/{payload}")
                    // .route(web::get().to(get_users))
                    .route(web::put().to(update_user))
                    .route(web::delete().to(delete_user)),
            )
    })
    .bind(config.server_addr.clone())?
    .run();

    println!("Server running at http://{}/", config.server_addr);

    server.await
}
