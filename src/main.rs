#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate rocket_dyn_templates;
extern crate serde;
extern crate tera;

use std::error::Error;
use rocket::{Build, Rocket};
use crate::dotenv::dotenv;
use rocket_dyn_templates::Template;

pub mod resources;
pub mod routes;
mod config;

fn main() -> Result<(), impl Error>{
    dotenv().ok();
    let rocket = launch();
    rocket::execute(rocket.launch()).map(|_|())
}

fn launch() -> Rocket<Build> {
    let mut rocket = rocket::build().attach(Template::fairing()).mount(
        "/",
        routes![
            routes::tech::index,
            routes::tech::robots,
            routes::autoconfig::v11_mail_config_v11,
            routes::autoconfig::mail_config_v11,
            routes::autoconfig::well_known_mail_config_v11,
            routes::autoconfig::mail_autodiscover_microsoft,
            routes::autoconfig::mail_autodiscover_microsoft_case,
            routes::autoconfig::mail_autodiscover_microsoft_camel_case,
            routes::autoconfig::post_mail_autodiscover_microsoft,
            routes::autoconfig::post_mail_autodiscover_microsoft_case,
            routes::autoconfig::post_mail_autodiscover_microsoft_camel_case,
            routes::autoconfig::post_mail_autodiscover_microsoft_json,
            routes::autoconfig::post_mail_autodiscover_microsoft_json_legacy,
        ],
    );
    if cfg!(feature = "apple") {
        rocket = rocket.mount(
            "/",
            routes![
                routes::autoconfig::mail_autodiscover_apple_mobileconfig,
                routes::tech::apple,
            ],
        );
    }
    if cfg!(feature = "dns") {
        rocket = rocket.mount("/", routes![routes::dns::dns_txt_zone,]);
    }
    rocket
}
