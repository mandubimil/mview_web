use actix_web::{error, web, Error, HttpResponse};
use tera::{Context, Tera};
use actix_session::Session;

use super::super::comm::*;
use super::super::comm::util::log;

pub async fn get_html(
    tmpl: web::Data<Tera>,
    session: Session,
    html_id: web::Path<(String,)>,    
) -> Result<HttpResponse, Error> {

    if !util::check_session(&session).unwrap() { return Ok(HttpResponse::Ok().body("누구?")) };

    let mut context = Context::new();
    context.insert("app_name", "gogo mview_web");    

    let view_html = format!("{}", &html_id.0);    

    log(20, &format!("{} 페이지 보임", view_html));

    let rendered = tmpl
        .render(&view_html, &context)
        .map_err(|e| error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().body(rendered))
}
