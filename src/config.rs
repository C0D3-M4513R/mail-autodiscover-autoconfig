use rocket::Request;
use rocket::request::Outcome;
const DOMAINS: phf::map::Map<&'static str, DomainConfig> = phf::phf_map!();
const CONFIG: Config = Config {
    default_domain: "c0d3m4513r.com",
    default_config: DomainConfig {
        display_name: "C0D3 M4513R Mail",
        imap: Service {
            hostname: "mail.c0d3m4513r.com",
            instances: &[
                ServiceInstance {
                    port: 993,
                    ssl: SSLType::SSL,
                },
                ServiceInstance {
                    port: 143,
                    ssl: SSLType::StartTls,
                },
            ],
        },
        pop: Service {
            hostname: "mail.c0d3m4513r.com",
            instances: &[
                ServiceInstance {
                    port: 995,
                    ssl: SSLType::SSL,
                },
                ServiceInstance {
                    port: 110,
                    ssl: SSLType::StartTls,
                },
            ],
        },
        smtp: Service {
            hostname: "mail.c0d3m4513r.com",
            instances: &[
                ServiceInstance {
                    port: 465,
                    ssl: SSLType::SSL,
                },
                ServiceInstance {
                    port: 25,
                    ssl: SSLType::StartTls,
                },
                ServiceInstance {
                    port: 587,
                    ssl: SSLType::StartTls,
                },
            ],
        },
        profile_uuid: "84641856-9840-48ae-be21-023ff4e750a7",
        mail_uuid: "f812f625-8e98-4181-a634-738fc72e763b",
    },
    domains: &DOMAINS,
};

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub default_config: DomainConfig,
    pub default_domain: &'static str,
    pub domains: &'static phf::map::Map<&'static str, DomainConfig>
}

#[derive(Debug, Clone, Copy)]
pub struct DomainConfig {
    pub display_name: &'static str,
    pub imap: Service,
    pub pop: Service,
    pub smtp: Service,
    pub profile_uuid: &'static str,
    pub mail_uuid: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct DomainConfiguration{
    pub domain: &'static str,
    pub config: &'static DomainConfig,
}
#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for DomainConfiguration {
    type Error = &'static str;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let domain = match request.host().map(|v|v.domain().as_str().to_lowercase()) {
            Some(domain) => CONFIG.domains.get_entry(domain.as_str()),
            None => None,
        };
        match domain {
            None => Outcome::Success(DomainConfiguration{
                domain: &CONFIG.default_domain,
                config: &CONFIG.default_config
            }),
            Some((domain, config)) => Outcome::Success(DomainConfiguration {
                domain: *domain,
                config,
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct Service {
    pub hostname: &'static str,
    pub instances: &'static [ServiceInstance],
}

#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub struct ServiceInstance {
    pub port: u16,
    pub ssl: SSLType
}

#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub enum SSLType {
    None,
    SSL,
    StartTls
}