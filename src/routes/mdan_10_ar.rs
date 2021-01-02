use actix_web::{web, Error, HttpResponse};
use actix_session::Session;
use bytes::Bytes;

use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use super::super::comm::*;


pub async fn post_job(
    job_id: web::Path<(String,)>,
    body: Bytes,
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
    session: Session,
) -> Result<HttpResponse, Error> {
    
    if !util::check_session(&session).unwrap() { return Ok(HttpResponse::Ok().body("누구?")) };

    let id = &job_id.0[..];
    let (para1, _, _) = util::body_to_hash(body).unwrap();
    let mut sql_jo: Vec<String> = Vec::new();

    let result_json = match id
    {
        "test_select" =>
        {
            
            let sql_text ="
            select 메모번호,제목 from 메모 
            ";
            
            db_postgres::select_json(db, sql_text.to_string(), sql_jo).unwrap()
        }
        "read_config" =>
        {

            let sql_text ="
            select 설정 from 사용자 where 아이디 = $1
            ";

            if let Some(user_id) = session.get::<String>("user_id").unwrap() {
                sql_jo.push(user_id);
            }
            
            db_postgres::select_json(db, sql_text.to_string(), sql_jo).unwrap()
        }
        "read_mdan_all" =>
        {
            
            let sql_text ="
            select (row_number() over()) as 순번, * from (select 단어번호, 제목, 분류, 핵심어, 내용, 히스토리 from 단어장 where 분류 = $1 and 아이디 = $2order by 제목) tt
            ";

            sql_jo.push(para1.get("분류").unwrap().to_string());
            if let Some(user_id) = session.get::<String>("user_id").unwrap() {
                sql_jo.push(user_id);
            }
            
            db_postgres::select_json(db, sql_text.to_string(), sql_jo).unwrap()
        }
        "read_mdan_imsi" =>
        {            
            let sql_text ="
            select 분류, 단어번호, 제목, 분류, 핵심어, 내용, 히스토리 
            from 단어장_백업 
            where 단어번호 = $1
            ";

            sql_jo.push(para1.get("단어번호").unwrap().to_string());
            
            db_postgres::select_json(db, sql_text.to_string(), sql_jo).unwrap()
        }
        "save_dan" =>
        {
            let sql_text ="
            insert into 단어장
            (분류, 단어번호, 제목, 핵심어, 내용, 히스토리, 아이디, 입력일자, 수정일자)
            values
            (   $1,
                (SELECT coalesce(max(단어번호),0)+1 from 단어장),
                $2, $3, $4, $5, $6, to_char(now(), 'YYYY-MM-DD HH24:MI:SS'), to_char(now(), 'YYYY-MM-DD HH24:MI:SS') 
            )
            returning 단어번호::text
            ";
            
            sql_jo.push(para1.get("분류").unwrap().to_string());
            sql_jo.push(para1.get("제목").unwrap().to_string());
            sql_jo.push(para1.get("핵심어").unwrap().to_string());
            sql_jo.push(para1.get("내용").unwrap().to_string());
            sql_jo.push(para1.get("히스토리").unwrap().to_string());
            
            //세션에서 아이디 가지고 오기
            if let Some(user_id) = session.get::<String>("user_id").unwrap() {
                sql_jo.push(user_id);
            }
            let dan_num = db_postgres::exe(db.clone(), sql_text.to_string(), sql_jo.clone()).unwrap();

            //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            
            let sql_text ="
            delete from 단어장_백업 
            where 단어번호 = $1
            returning 단어번호::text
            ";
            
            sql_jo.clear();
            sql_jo.push(dan_num.clone());
            
            db_postgres::exe(db.clone(), sql_text.to_string(), sql_jo.clone()).unwrap();

            //////////////////////////////////////////////////////////////////////////////////////////////////////////////////

            let sql_text ="
            insert into 단어장_백업
            (분류, 단어번호, 제목, 핵심어, 내용, 히스토리, 아이디, 입력일자, 수정일자)
            values
            (   $1,
                $2,
                $3, $4, $5, $6, '임시', to_char(now(), 'YYYY-MM-DD HH24:MI:SS'), to_char(now(), 'YYYY-MM-DD HH24:MI:SS') 
            )
            returning 단어번호::text
            ";
            
            sql_jo.clear();
            sql_jo.push(para1.get("분류").unwrap().to_string());
            sql_jo.push(dan_num.clone());
            sql_jo.push(para1.get("제목").unwrap().to_string());
            sql_jo.push(para1.get("핵심어").unwrap().to_string());
            sql_jo.push(para1.get("내용").unwrap().to_string());
            sql_jo.push(para1.get("히스토리").unwrap().to_string());
            
            db_postgres::exe(db.clone(), sql_text.to_string(), sql_jo.clone()).unwrap();

            //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            
            dan_num
        }
        "update_dan" =>
        {
            let sql_text ="
            update 단어장 
            set 분류 = $1,
                제목 = $2,
                핵심어 = $3,
                내용 = $4,
                히스토리 = $5,
                수정일자 = to_char(now(), 'YYYY-MM-DD HH24:MI:SS') 
            where 단어번호::text = $6
              and 아이디 = $7
            returning 단어번호::text
            ";
            
            sql_jo.push(para1.get("분류").unwrap().to_string());
            sql_jo.push(para1.get("제목").unwrap().to_string());
            sql_jo.push(para1.get("핵심어").unwrap().to_string());
            sql_jo.push(para1.get("내용").unwrap().to_string());
            sql_jo.push(para1.get("히스토리").unwrap().to_string());
            sql_jo.push(para1.get("단어번호").unwrap().to_string());
            
            //세션에서 아이디 가지고 오기
            if let Some(user_id) = session.get::<String>("user_id").unwrap() {
                sql_jo.push(user_id);
            }
            db_postgres::exe(db, sql_text.to_string(), sql_jo).unwrap()
        }
        "update_dan_imsi" =>
        {
            let sql_text ="
            update 단어장_백업 
            set 분류 = $1,
                제목 = $2,
                핵심어 = $3,
                내용 = $4,
                히스토리 = $5,
                수정일자 = to_char(now(), 'YYYY-MM-DD HH24:MI:SS') 
            where 단어번호 = $6
            returning 단어번호::text
            ";
            
            sql_jo.clear();
            sql_jo.push(para1.get("분류").unwrap().to_string());
            sql_jo.push(para1.get("제목").unwrap().to_string());
            sql_jo.push(para1.get("핵심어").unwrap().to_string());
            sql_jo.push(para1.get("내용").unwrap().to_string());
            sql_jo.push(para1.get("히스토리").unwrap().to_string());
            sql_jo.push(para1.get("단어번호").unwrap().to_string());
            
            db_postgres::exe(db, sql_text.to_string(), sql_jo).unwrap()
        }
        "del_dan" =>
        {
            let sql_text ="
            delete from 단어장_백업 
            where 단어번호 = $1
            returning 단어번호::text
            ";
            
            sql_jo.clear();
            sql_jo.push(para1.get("단어번호").unwrap().to_string());
            
            db_postgres::exe(db.clone(), sql_text.to_string(), sql_jo.clone()).unwrap();
            
            //////////////////////////////////////////////////////////////////////////////////////////////
            
            let sql_text ="
            delete from 단어장 
            where 단어번호::text = $1
              and 아이디 = $2
            returning 단어번호::text
            ";
            
            sql_jo.clear();
            sql_jo.push(para1.get("단어번호").unwrap().to_string());
            
            //세션에서 아이디 가지고 오기
            if let Some(user_id) = session.get::<String>("user_id").unwrap() {
                sql_jo.push(user_id);
            }
            db_postgres::exe(db.clone(), sql_text.to_string(), sql_jo.clone()).unwrap()
        }
        "save_config" =>
        {
            let sql_text ="
            update 사용자
            set 설정 = $1
            where 아이디 = $2
            returning 설정::text
            ";
            
            sql_jo.push(para1.get("설정").unwrap().to_string());            
            //세션에서 아이디 가지고 오기
            if let Some(user_id) = session.get::<String>("user_id").unwrap() {
                sql_jo.push(user_id);
            }
            db_postgres::exe(db, sql_text.to_string(), sql_jo).unwrap()
        }        
        _ =>
        {
            "no job~".to_string()
        }        
    };

    println!("result. {:?}", result_json);

    Ok(HttpResponse::Ok()
    .content_type("application/json")
    .body(result_json))
}

