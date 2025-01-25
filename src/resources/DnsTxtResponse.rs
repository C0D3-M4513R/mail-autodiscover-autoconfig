use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_dyn_templates::Template;

pub struct DnsTxtResponse {
    pub template: Template,
    pub domain: &'static str,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for DnsTxtResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Response::build_from(self.template.respond_to(req)?)
            .raw_header("Content-Type", "text/plain; charset=utf-8")
            .raw_header(
                "Content-Disposition",
                format!(r#"attachment; filename="{}-dns-zone.txt""#, self.domain),
            )
            .ok()
    }
}
