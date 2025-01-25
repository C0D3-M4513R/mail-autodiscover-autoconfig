use crate::resources::DnsTxtResponse::DnsTxtResponse;
use rocket_dyn_templates::{context, Template};

#[get("/dns-zone")]
pub fn dns_txt_zone(host: crate::config::DomainConfiguration) -> DnsTxtResponse {
    // See :https://developer.apple.com/business/documentation/Configuration-Profile-Reference.pdf
    DnsTxtResponse {
        domain: host.domain,
        template: Template::render(
            "dns/zone",
            context! {
                imap_hostname: host.config.imap.hostname,
                pop_hostname: host.config.pop.hostname,
                smtp_hostname: host.config.smtp.hostname,
            },
        ),
    }
}