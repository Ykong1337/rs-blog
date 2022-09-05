use std::error::Error;
use std::sync::Arc;
use fast_log::Config;
use jwt_simple::algorithms::HS256Key;
use once_cell::sync::Lazy;
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use rocket::fairing::AdHoc;
use rocket::Request;
use crate::controller::user_controller;
use crate::controller::tag_controller;
use crate::controller::category_controller;

mod common;
mod controller;
mod model;
mod util;

#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

pub static RB: Lazy<Rbatis> = Lazy::new(|| Rbatis::new());
pub static KEY: Lazy<HS256Key> = Lazy::new(|| HS256Key::generate());

#[catch(404)]
pub async fn not_found(req: &Request<'_>) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    fast_log::init(Config::new().console()).unwrap();
    let rb = Arc::new(&RB);
    RB.init(MysqlDriver {}, "mysql://root:root@127.0.0.1:3306/rustblog").unwrap();
    let _ = rocket::build()
        .mount("/api",
               routes![
                   user_controller::list,
                   user_controller::login,
                   user_controller::del,
                   user_controller::vo_list,
                   tag_controller::list,
                   tag_controller::create,
                   tag_controller::update,
                   tag_controller::del,
                   tag_controller::find_by_name,
                   tag_controller::find_list_count,
                   category_controller::find_by_name,
                   category_controller::list,
                   category_controller::list_count,
                   category_controller::create,
                   category_controller::update,
                   category_controller::del,
        ])
        .register("/", catchers![not_found])
        .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
            rocket.manage(rb)
        }))
        .launch()
        .await?;

    Ok(())
}