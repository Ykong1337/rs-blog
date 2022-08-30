use std::error::Error;
use std::sync::Arc;
use fast_log::Config;
use jwt_simple::algorithms::HS256Key;
use once_cell::sync::Lazy;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use rocket::fairing::AdHoc;
use rocket::{Request, routes, catch, catchers};
use crate::controller::user_controller;

mod common;
mod controller;
mod model;
mod util;

#[macro_use]
extern crate rbatis;

pub static RB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());
pub static KEY: Lazy<HS256Key> = Lazy::new(|| HS256Key::generate());

#[catch(404)]
pub async fn not_found(req: &Request<'_>) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    fast_log::init(Config::new().console())?;
    let rb = Arc::new(&RB);
    RB.link(MysqlDriver {}, "mysql://root:root@127.0.0.1:3306/rustblog").await.unwrap();
    let _ = rocket::build()
        .mount("/api",
               routes![
                   user_controller::list,
                   user_controller::login,
                   user_controller::del,
                   user_controller::vo_list,
        ])
        .register("/", catchers![not_found])
        .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
            rocket.manage(rb)
        }))
        .launch()
        .await?;

    Ok(())
}