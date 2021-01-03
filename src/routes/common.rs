use actix_web::{http, web, Error, HttpResponse};
use actix_session::Session;
use bytes::Bytes;

use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use super::super::comm::*;
use super::super::comm::util::log;

pub async fn connect(
    body: Bytes,
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let (para1, _, _) = util::body_to_hash(body).unwrap();
    log(20, &format!("connect key: {:?}", para1));

    let sql_text ="
    select case count(*) when 1 then 'ok' else 'no' end from 사용자
    where 아이디 = $1
    and 비밀번호 = $2
    ";

    let mut sql_jo: Vec<String> = Vec::new();
    sql_jo.push(para1.get("p1").unwrap().to_string());
    sql_jo.push(para1.get("p2").unwrap().to_string());

    let return_str1 = db_postgres::select_one(db.clone(), sql_text.to_string(), sql_jo).unwrap();

    //////////////////////////////////////////////////////////////////////////////////////////////////////////

    let sql_text ="
    select case count(*) when 1 then 'ok' else 'no' end from 접속장비
    where 시리얼 = $1
    ";

    let mut sql_jo: Vec<String> = Vec::new();
    sql_jo.push(para1.get("p3").unwrap().to_string());

    let return_str2 = db_postgres::select_one(db, sql_text.to_string(), sql_jo).unwrap();
    log(30, &format!("한줄 가져오기 값 : {}", return_str2));
    //////////////////////////////////////////////////////////////////////////////////////////////////////////

    // if return_str1 == "ok" && return_str2 == "ok"{
    if return_str1 == "ok" {
        session.set("check_login", "ok_check")?;
        session.set("user_id", para1.get("p1").unwrap().to_string())?;

        log(30, "세션 저장~");
    } else {
        session.set("check_login", "who?")?;
        session.set("user_id", "who?")?;

        log(30, "세션 저장~ 근데 누구?");
    }

    Ok(HttpResponse::Found().header(http::header::LOCATION, "/view/mdan_10_ar.html").finish().into_body())
}


pub async fn get_common(
    job_id: web::Path<(String,)>,
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
    session: Session,
) -> Result<HttpResponse, Error> {

    if !util::check_session(&session).unwrap() { return Ok(HttpResponse::Ok().body("누구?")) };

    let id = &job_id.0[..];
    let sql_jo: Vec<String> = Vec::new();

    let result_json = match id
    {
        "get_dan_group" =>
        {
            let sql_text ="
            select 코드 as id, 코드명 as value from 공통코드 where 구분 = '단어장_분류' order by 코드
            ";

            db_postgres::select_json(db, sql_text.to_string(), sql_jo).unwrap()
        }
        "get_dan_display" =>
        {
            let sql_text ="
            select 코드 as id, 코드명 as value from 공통코드 where 구분 = '단어장_출력' order by 코드
            ";

            db_postgres::select_json(db, sql_text.to_string(), sql_jo).unwrap()
        }
        _ =>
        {
            "no job~".to_string()
        }
    };

    log(30, &format!("result. {:?}", result_json));

    Ok(HttpResponse::Ok()
    .content_type("application/json")
    .body(result_json))
}
