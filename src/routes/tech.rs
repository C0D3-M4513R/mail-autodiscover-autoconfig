use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index(host: crate::config::DomainConfiguration) -> Template {
    Template::render(
        "index",
        context! {
            name: host.domain
        },
    )
}

#[get("/apple")]
pub fn apple(host: crate::config::DomainConfiguration) -> Template {
    Template::render(
        "apple",
        context! {
            name: host.domain
        },
    )
}

#[get("/robots.txt")]
pub fn robots() -> &'static str {
    "User-agent: *\nDisallow: /\n"
}