use crate::resources::AppleResponse::AppleResponse;
use crate::resources::AutoDiscoverJson::{AutoDiscoverJson, AutoDiscoverJsonError};
use crate::resources::AutoDiscoverXml::{
    AutoDiscoverXml, AutoDiscoverXmlError, AutoDiscoverXmlPayload,
};
use chrono::Local;
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};

fn handle_mail_config_v11<'a>(config: crate::config::DomainConfiguration, emailaddress: Option<&'a str>) -> AutoDiscoverXml {
    let email:std::borrow::Cow<'a, str> = match emailaddress {
        None => "%EMAILADDRESS%".into(),
        Some(e) => if e.contains("@") { e.into() } else { format!("{}@{}", e, config.domain).into() },
    };
    AutoDiscoverXml {
        domain: config.domain,
        template: Template::render(
            "xml/config-v1.1",
            context! {
                domain: config.domain,
                display_name: config.config.display_name,
                email: email,
                imap: config.config.imap,
                pop: config.config.pop,
                smtp: config.config.smtp,
            },
        ),
    }
}

// Used by Maily (tested version: 1.0.0) (de.enough.enough_mail_app: https://github.com/Enough-Software/enough_mail/blob/v2.1.1/lib/src/private/util/discover_helper.dart#L34-L58)
// Used by KMail (tested version: 5.19.3 (21.12.3)) (https://github.com/KDE/kmail-account-wizard/blob/v21.12.3/src/ispdb/ispdb.cpp#L64-L90) (https://invent.kde.org/pim/kmail-account-wizard/-/blob/v21.12.3/src/ispdb/ispdb.cpp#L64-90)
// Used by Thunderbird (tested version: Thunderbird 91.10.0)
// Used by FairEmail (tested version: 1.1917) (https://github.com/M66B/FairEmail/blob/1.1917/app/src/main/java/eu/faircode/email/EmailProvider.java#L558)
// Used by Evolution on Ubuntu (tested version: 3.40.0-1) (/mail/config-v1.1.xml?emailaddress=EVOLUTIONUSER%40wdes.fr&emailmd5=46865a3ba18ca94e2c98f15b8cf14125) (https://gitlab.gnome.org/GNOME/evolution/-/blob/3.40.1/src/mail/e-mail-autoconfig.c#L514)
// Used by Spark Mail on Android (tested version: 2.11.8)
// Used by MailTime on Android (tested version: 2.5.4.0614)
// Used by ProfiMail on Android (tested version: 4.31.08) (https://www.lonelycatgames.com/apps/profimail)
// Used by K-9 Mail on Android (tested version: 6.709, since: 6.709)
#[get("/mail/config-v1.1.xml?<emailaddress>")]
#[allow(unused_variables)]
pub fn mail_config_v11(host: crate::config::DomainConfiguration, emailaddress: Option<&str>) -> AutoDiscoverXml {
    handle_mail_config_v11(host, emailaddress)
}

// Used by Android Nine (tested version: 4.9.4b) (com.ninefolders.hd3)
#[get("/v1.1/mail/config-v1.1.xml?<emailaddress>")]
#[allow(unused_variables)]
pub fn v11_mail_config_v11(host: crate::config::DomainConfiguration, emailaddress: Option<&str>) -> AutoDiscoverXml {
    handle_mail_config_v11(host, emailaddress)
}

#[get("/.well-known/autoconfig/mail/config-v1.1.xml?<emailaddress>")]
#[allow(unused_variables)]
pub fn well_known_mail_config_v11(host: crate::config::DomainConfiguration, emailaddress: Option<&str>) -> AutoDiscoverXml {
    handle_mail_config_v11(host, emailaddress)
}

// Used by Android Nine (tested version: 4.9.4b) (com.ninefolders.hd3)
// Used by Microsoft Outlook for Android (tested version: 4.2220.1)
// Used by Microsoft Office Pro Plus 2021 (tested version: 14326.20454 64 bits)

// Example: /autodiscover/autodiscover.json?Email=test%40wdes.fr&Protocol=ActiveSync&RedirectCount=1
// Example: /autodiscover/autodiscover.json?Email=test%40wdes.fr&Protocol=Autodiscoverv1&RedirectCount=1
#[get("/autodiscover/autodiscover.json?<Email>&<Protocol>&<RedirectCount>")]
#[allow(unused_variables)]
#[allow(non_snake_case)]
pub fn post_mail_autodiscover_microsoft_json(
    host: crate::config::DomainConfiguration,
    Email: Option<&str>,
    Protocol: Option<&str>,
    RedirectCount: Option<&str>,
) -> Result<Json<AutoDiscoverJson>, AutoDiscoverJsonError> {
    match Protocol {
        Some("AutodiscoverV1") => Ok(Json(AutoDiscoverJson {
            Protocol: "AutodiscoverV1".to_string(),
            Url: "https://".to_owned() + &host.domain + "/Autodiscover/Autodiscover.xml",
        })),
        Some("Autodiscoverv1") => Ok(Json(AutoDiscoverJson {
            Protocol: "Autodiscoverv1".to_string(),
            Url: "https://".to_owned() + &host.domain + "/Autodiscover/Autodiscover.xml",
        })),
        /*
        Some("ActiveSync") => Some(AutoDiscoverJson {
            Protocol: "ActiveSync".to_string(),
            Url: "https://".to_owned() + &host.host + "/Microsoft-Server-ActiveSync",
        }),*/
        _ => Err(AutoDiscoverJsonError {
            ErrorCode: "InvalidProtocol".to_string(),
            ErrorMessage:
                "The given protocol value is invalid. Supported values are \"AutodiscoverV1\" or \"Autodiscoverv1\"."
                    .to_string(),
        }),
    }
}

// Used by Microsoft Office 2009 (to be confirmed)
// Example: /autodiscover/autodiscover.json/v1.0/infos%40domain.tld?Protocol=ActiveSync&RedirectCount=1
#[get("/autodiscover/autodiscover.json/v1.0/infos?<Email>&<Protocol>&<RedirectCount>")]
#[allow(unused_variables)]
#[allow(non_snake_case)]
pub fn post_mail_autodiscover_microsoft_json_legacy(
    host: crate::config::DomainConfiguration,
    Email: Option<&str>,
    Protocol: Option<&str>,
    RedirectCount: Option<&str>,
) -> Result<Json<AutoDiscoverJson>, AutoDiscoverJsonError> {
    match Protocol {
        Some("AutodiscoverV1") => Ok(Json(AutoDiscoverJson {
            Protocol: "AutodiscoverV1".to_string(),
            Url: "https://".to_owned() + &host.domain + "/Autodiscover/Autodiscover.xml",
        })),
        Some("Autodiscoverv1") => Ok(Json(AutoDiscoverJson {
            Protocol: "Autodiscoverv1".to_string(),
            Url: "https://".to_owned() + &host.domain + "/Autodiscover/Autodiscover.xml",
        })),
        /*
        Some("ActiveSync") => Some(AutoDiscoverJson {
            Protocol: "ActiveSync".to_string(),
            Url: "https://".to_owned() + &host.host + "/Microsoft-Server-ActiveSync",
        }),*/
        _ => Err(AutoDiscoverJsonError {
            ErrorCode: "InvalidProtocol".to_string(),
            ErrorMessage:
                "The given protocol value is invalid. Supported values are \"AutodiscoverV1\" or \"Autodiscoverv1\"."
                    .to_string(),
        }),
    }
}

//https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/protocol-pox
fn autodiscover_microsoft(
    host: crate::config::DomainConfiguration,
    payload: Option<AutoDiscoverXmlPayload>,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    let email_address = payload.as_ref().map(|v|v.Request.EMailAddress.as_ref()).flatten().map(|v|{
        if v.contains("@") {
            v.clone()
        } else {
            format!("{}@{}", v, host.domain)
        }
    }).unwrap_or(String::new());
    // TODO: http://schemas.microsoft.com/exchange/autodiscover/mobilesync/responseschema/2006
    match &payload {
        Some(p) => match p.Request.AcceptableResponseSchema.as_str() {
            "http://schemas.microsoft.com/exchange/autodiscover/outlook/responseschema/2006a" => {
                Ok(AutoDiscoverXml {
                    domain: host.domain,
                    template: Template::render(
                        "xml/autodiscover",
                        context! {
                            email_address: email_address,
                            domain: host.domain,
                            display_name: host.config.display_name,
                            imap: host.config.imap,
                            pop: host.config.pop,
                            smtp: host.config.smtp,
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
            domain: host.domain,
            template: Template::render(
                "xml/autodiscover",
                context! {
                    email_address: email_address,
                    domain: host.domain,
                    display_name: host.config.display_name,
                    imap: host.config.imap,
                    pop: host.config.pop,
                    smtp: host.config.smtp,
                },
            ),
        }),
    }
}

// Used by Android MyMail (tested version: 14.26.0.37052) (com.my.mail)
// Used by Android Spike Email (tested version: 3.5.7.0) (com.pingapp.app)
// Used by Microsoft Outlook for Android (tested version: 4.2220.1)
// Used by Microsoft Office Pro Plus 2013 (tested version: 15.0.5399.1000 64 bits)
// Used by Microsoft Office Pro Plus 2021 (tested version: 14326.20454 64 bits)
#[get("/autodiscover/autodiscover.xml")]
pub fn mail_autodiscover_microsoft(
    host: crate::config::DomainConfiguration,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, None)
}

#[get("/Autodiscover/Autodiscover.xml")]
pub fn mail_autodiscover_microsoft_case(
    host: crate::config::DomainConfiguration,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, None)
}

#[get("/AutoDiscover/AutoDiscover.xml")]
pub fn mail_autodiscover_microsoft_camel_case(
    host: crate::config::DomainConfiguration,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, None)
}

// Used by Thunderbird (tested version: 91.10.0)
// Used by Microsoft Outlook for Android (tested version: 4.2220.1)
// Used by Microsoft Office Pro Plus 2013 (tested version: 15.0.5399.1000 64 bits)
// Used by Microsoft Office Pro Plus 2021 (tested version: 14326.20454 64 bits)
// Used by Microsoft Office 2009 (to be confirmed)
#[post("/autodiscover/autodiscover.xml", data = "<payload>")]
pub fn post_mail_autodiscover_microsoft(
    host: crate::config::DomainConfiguration,
    payload: AutoDiscoverXmlPayload,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, Some(payload))
}

#[post("/Autodiscover/Autodiscover.xml", data = "<payload>")]
pub fn post_mail_autodiscover_microsoft_case(
    host: crate::config::DomainConfiguration,
    payload: AutoDiscoverXmlPayload,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, Some(payload))
}

#[post("/AutoDiscover/AutoDiscover.xml", data = "<payload>")]
pub fn post_mail_autodiscover_microsoft_camel_case(
    host: crate::config::DomainConfiguration,
    payload: AutoDiscoverXmlPayload,
) -> Result<AutoDiscoverXml, AutoDiscoverXmlError> {
    autodiscover_microsoft(host, Some(payload))
}

// iOS / Apple Mail (/email.mobileconfig?email=username@domain.com or /email.mobileconfig?email=username)
#[get("/email.mobileconfig?<email>")]
pub fn mail_autodiscover_apple_mobileconfig(host: crate::config::DomainConfiguration, email: &str) -> AppleResponse {
    let email_address = if !email.contains("@") {
        format!("{}@{}", email, host.domain)
    } else {
        email.to_string()
    };

    // See :https://developer.apple.com/business/documentation/Configuration-Profile-Reference.pdf
    AppleResponse {
        domain: host.domain,
        template: Template::render(
            "xml/email_mobileconfig",
            context! {
                domain: host.domain,
                display_name: host.config.display_name,
                imap: host.config.imap,
                pop: host.config.pop,
                smtp: host.config.smtp,
                email_address: email_address,
                mail_uuid: host.config.mail_uuid,
                profile_uuid: host.config.profile_uuid,
            },
        ),
    }
}