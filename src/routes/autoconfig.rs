use crate::host_header::HostHeader;
use crate::ressources::AppleResponse::AppleResponse;
use crate::ressources::AutoDiscoverJson::{AutoDiscoverJson, AutoDiscoverJsonError};
use crate::ressources::AutoDiscoverXml::{
    AutoDiscoverXml, AutoDiscoverXmlError, AutoDiscoverXmlPayload,
};
use chrono::Local;
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};
use std::env;

#[derive(Eq, PartialEq, Debug)]
struct Config<'a> {
    domain: &'a str,
    display_name: String,
    imap_hostname: String,
    pop_hostname: String,
    smtp_hostname: String,
}

fn get_config_for_domain(domain: &str) -> Config {
    let is_custom_host: bool = env::var("CUSTOM_DOMAINS")
        .expect("CUSTOM_DOMAINS must be set")
        .split(',')
        .collect::<Vec<&str>>()
        .contains(&domain);

    if is_custom_host {
        return Config {
            domain: domain,
            display_name: domain.to_owned() + " Mail",
            imap_hostname: "imap.".to_owned() + &domain.to_owned(),
            pop_hostname: "pop.".to_owned() + &domain.to_owned(),
            smtp_hostname: "smtp.".to_owned() + &domain.to_owned(),
        };
    }

    let imap_hostname: String = env::var("IMAP_HOSTNAME").expect("IMAP_HOSTNAME must be set");
    let pop_hostname: String = env::var("POP_HOSTNAME").expect("IMAP_HOSTNAME must be set");
    let smtp_hostname: String = env::var("SMTP_HOSTNAME").expect("IMAP_HOSTNAME must be set");

    Config {
        domain: domain,
        display_name: domain.to_owned() + " Mail",
        imap_hostname: imap_hostname,
        pop_hostname: pop_hostname,
        smtp_hostname: smtp_hostname,
    }
}

fn handle_mail_config_v11(host: HostHeader) -> AutoDiscoverXml {
    let config: Config = get_config_for_domain(&host.base_domain);
    AutoDiscoverXml {
        domain: config.domain.to_string(),
        template: Template::render(
            "xml/config-v1.1",
            context! {
                domain: config.domain,
                display_name: config.display_name,
                imap_hostname: config.imap_hostname,
                pop_hostname: config.pop_hostname,
                smtp_hostname: config.smtp_hostname,
            },
        ),
    }
}

// Used by Thunderbird (tested version: Thunderbird 91.10.0)
// Used by FairEmail (tested version: 1.1917) (https://github.com/M66B/FairEmail/blob/1.1917/app/src/main/java/eu/faircode/email/EmailProvider.java#L558)
// Used by Evolution on Ubuntu (tested version: 3.40.0-1) (/mail/config-v1.1.xml?emailaddress=EVOLUTIONUSER%40wdes.fr&emailmd5=46865a3ba18ca94e2c98f15b8cf14125) (https://gitlab.gnome.org/GNOME/evolution/-/blob/3.40.1/src/mail/e-mail-autoconfig.c#L514)
// Used by Spark Mail on Android (tested version: 2.11.8)
#[get("/mail/config-v1.1.xml?<emailaddress>")]
#[allow(unused_variables)]
pub fn mail_config_v11(host: HostHeader, emailaddress: Option<&str>) -> AutoDiscoverXml {
    handle_mail_config_v11(host)
}

// Used by Android Nine (tested version: 4.9.4b) (com.ninefolders.hd3)
#[get("/v1.1/mail/config-v1.1.xml?<emailaddress>")]
#[allow(unused_variables)]
pub fn v11_mail_config_v11(host: HostHeader, emailaddress: Option<&str>) -> AutoDiscoverXml {
    handle_mail_config_v11(host)
}

#[get("/.well-known/autoconfig/mail/config-v1.1.xml?<emailaddress>")]
#[allow(unused_variables)]
pub fn well_known_mail_config_v11(host: HostHeader, emailaddress: Option<&str>) -> AutoDiscoverXml {
    handle_mail_config_v11(host)
}

// Used by Android Nine (tested version: 4.9.4b) (com.ninefolders.hd3)
// Used by Microsoft Outlook for Android (tested version: 4.2220.1)
// Example: /autodiscover/autodiscover.json?Email=test%40wdes.fr&Protocol=ActiveSync&RedirectCount=1
#[get("/autodiscover/autodiscover.json?<Email>&<Protocol>&<RedirectCount>")]
#[allow(unused_variables)]
#[allow(non_snake_case)]
pub fn post_mail_autodiscover_microsoft_json(
    host: HostHeader,
    Email: Option<&str>,
    Protocol: Option<&str>,
    RedirectCount: Option<&str>,
) -> Result<Json<AutoDiscoverJson>, AutoDiscoverJsonError> {
    match Protocol {
        Some("AutodiscoverV1") => Ok(Json(AutoDiscoverJson {
            Protocol: "AutodiscoverV1".to_string(),
            Url: "https://".to_owned() + &host.base_domain + "/Autodiscover/Autodiscover.xml",
        })),
        /*
        Some("ActiveSync") => Some(AutoDiscoverJson {
            Protocol: "ActiveSync".to_string(),
            Url: "https://".to_owned() + &host.base_domain + "/Microsoft-Server-ActiveSync",
        }),*/
        _ => Err(AutoDiscoverJsonError {
            ErrorCode: "InvalidProtocol".to_string(),
            ErrorMessage:
                "The given protocol value is invalid. Supported values are \"AutodiscoverV1\"."
                    .to_string(),
        }),
    }
}

fn autodiscover_microsoft(
    host: HostHeader,
    payload: Option<AutoDiscoverXmlPayload>,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    let config: Config = get_config_for_domain(&host.base_domain);
    match payload {
        Some(p) => match p.Request.AcceptableResponseSchema.as_str() {
            "http://schemas.microsoft.com/exchange/autodiscover/outlook/responseschema/2006a" => {
                Ok(AutoDiscoverXml {
                    domain: config.domain.to_string(),
                    template: Template::render(
                        "xml/autodiscover",
                        context! {
                            domain: config.domain,
                            display_name: config.display_name,
                            imap_hostname: config.imap_hostname,
                            pop_hostname: config.pop_hostname,
                            smtp_hostname: config.smtp_hostname,
                        },
                    ),
                })
            }
            _ => {
                let date = Local::now();
                Err(AutoDiscoverXmlError {
                    template: Template::render(
                        "xml/autodiscover-error",
                        context! {
                            time: date.format("%H:%M:%S").to_string(),
                            id: date.format("%s").to_string(),
                        },
                    ),
                })
            }
        },
        None => Ok(AutoDiscoverXml {
            domain: config.domain.to_string(),
            template: Template::render(
                "xml/autodiscover",
                context! {
                    domain: config.domain,
                    display_name: config.display_name,
                    imap_hostname: config.imap_hostname,
                    pop_hostname: config.pop_hostname,
                    smtp_hostname: config.smtp_hostname,
                },
            ),
        }),
    }
}

// Used by Android MyMail (tested version: 14.26.0.37052) (com.my.mail)
// Used by Android Spike Email (tested version: 3.5.7.0) (com.pingapp.app)
// Used by Microsoft Outlook for Android (tested version: 4.2220.1)
#[get("/autodiscover/autodiscover.xml")]
pub fn mail_autodiscover_microsoft(
    host: HostHeader,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, None)
}

#[get("/Autodiscover/Autodiscover.xml")]
pub fn mail_autodiscover_microsoft_case(
    host: HostHeader,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, None)
}

#[get("/AutoDiscover/AutoDiscover.xml")]
pub fn mail_autodiscover_microsoft_camel_case(
    host: HostHeader,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, None)
}

// Used by Thunderbird (tested version: 91.10.0)
// Used by Microsoft Outlook for Android (tested version: 4.2220.1)
#[post("/autodiscover/autodiscover.xml", data = "<payload>")]
pub fn post_mail_autodiscover_microsoft(
    host: HostHeader,
    payload: AutoDiscoverXmlPayload,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, Some(payload))
}

#[post("/Autodiscover/Autodiscover.xml", data = "<payload>")]
pub fn post_mail_autodiscover_microsoft_case(
    host: HostHeader,
    payload: AutoDiscoverXmlPayload,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, Some(payload))
}

#[post("/AutoDiscover/AutoDiscover.xml", data = "<payload>")]
pub fn post_mail_autodiscover_microsoft_camel_case(
    host: HostHeader,
    payload: AutoDiscoverXmlPayload,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, Some(payload))
}

// iOS / Apple Mail (/email.mobileconfig?email=username@domain.com or /email.mobileconfig?email=username)
#[get("/email.mobileconfig?<email>")]
pub fn mail_autodiscover_microsoft_apple(host: HostHeader, email: &str) -> AppleResponse {
    let config: Config = get_config_for_domain(&host.base_domain);
    let mail_uuid: String = env::var("APPLE_MAIL_UUID").expect("APPLE_MAIL_UUID must be set");
    let profile_uuid: String =
        env::var("APPLE_PROFILE_UUID").expect("APPLE_PROFILE_UUID must be set");

    // See :https://developer.apple.com/business/documentation/Configuration-Profile-Reference.pdf
    AppleResponse {
        domain: config.domain.to_string(),
        template: Template::render(
            "xml/email_mobileconfig",
            context! {
                domain: config.domain,
                display_name: config.display_name,
                imap_hostname: config.imap_hostname,
                pop_hostname: config.pop_hostname,
                smtp_hostname: config.smtp_hostname,
                email_address: email,
                username: email,
                mail_uuid: mail_uuid,
                profile_uuid: profile_uuid,
            },
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_for_domain() {
        temp_env::with_vars(
            vec![
                ("CUSTOM_DOMAINS", Some("foo.tld")),
                ("IMAP_HOSTNAME", Some("imap.foo.tld")),
                ("POP_HOSTNAME", Some("pop.example.tld")),
                ("SMTP_HOSTNAME", Some("smtp.domain.tld")),
            ],
            || {
                assert_eq!(
                    Config {
                        domain: "foo.tld",
                        display_name: "foo.tld Mail".to_string(),
                        imap_hostname: "imap.foo.tld".to_string(),
                        pop_hostname: "pop.foo.tld".to_string(),
                        smtp_hostname: "smtp.foo.tld".to_string(),
                    },
                    get_config_for_domain("foo.tld")
                );
            },
        );
        temp_env::with_vars(
            vec![
                ("CUSTOM_DOMAINS", Some("foo.bar")),
                ("IMAP_HOSTNAME", Some("imap.custom.tld")),
                ("POP_HOSTNAME", Some("pop.example.tld")),
                ("SMTP_HOSTNAME", Some("smtp.domain.tld")),
            ],
            || {
                assert_eq!(
                    Config {
                        domain: "foo.tld",
                        display_name: "foo.tld Mail".to_string(),
                        imap_hostname: "imap.custom.tld".to_string(),
                        pop_hostname: "pop.example.tld".to_string(),
                        smtp_hostname: "smtp.domain.tld".to_string(),
                    },
                    get_config_for_domain("foo.tld")
                );
            },
        );
    }
}
