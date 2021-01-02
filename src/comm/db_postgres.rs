use super::util;

use actix_web::{web, Error};

use json::JsonValue;
use std::thread;

use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::postgres::types::ToSql;

use super::super::comm::util::log;

pub fn connect_db() -> Result<Pool<PostgresConnectionManager<NoTls>>, Error>{

    let injson: JsonValue = util::get_config_file().unwrap();

    let db_ip_postgres = injson["db_ip_postgres"].as_str().unwrap();
    let db_name_postgres = injson["db_name_postgres"].as_str().unwrap();
    let db_id_postgres = injson["db_id_postgres"].as_str().unwrap();
    let db_passwd_postgres = injson["db_passwd_postgres"].as_str().unwrap();

    let manager = PostgresConnectionManager::new( format!("postgresql://{}:{}@{}/{}", 
                                                    db_id_postgres, 
                                                    db_passwd_postgres, 
                                                    db_ip_postgres, 
                                                    db_name_postgres
                                                ).parse().unwrap(), NoTls); 

    let pool = r2d2::Pool::new(manager).unwrap();    
    
    return Ok(pool);
}


pub fn select_one(
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
    sql_text: String,
    sql_jo: Vec<String>,
) -> Result<String, Error> {        
    let return_str = thread::spawn(move || {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();                            
        for column in sql_jo.iter(){
            params.push(column);
        }

        let mut row_one = String::new();
        let mut conn = db.get().unwrap();        
        let rows = &conn.query(&sql_text[..], &params[..]).unwrap();

        if rows.len() == 1{
            log(20, "한줄 가져오기 query 성공~");
            for row in rows.iter() {
                row_one = row.get(0);
                break;
            }   
        }

        log(30, &format!("한줄 가져오기 값 : {}", row_one));

        return row_one;
    }).join().unwrap();
    
    return Ok(return_str);    
}

pub fn sum_query(main_query : String) -> Result<String, Error>{
    let return_query_text = format!("SELECT coalesce(array_to_json(array_agg(t))::text, '{}') FROM ( {} ) t", "{\"data\":\"none\"}", main_query);
    return Ok(return_query_text);
}

pub fn select_json(
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
    sql_text: String,
    sql_jo: Vec<String>,
) -> Result<String, Error> {        

    let return_str = thread::spawn(move || {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();                            
        for column in sql_jo.iter(){
            params.push(column);
        }

        let mut row_one = String::new();
        let mut conn = db.get().unwrap();        

        let tot_sql_text = sum_query(sql_text).unwrap();
        log(30, &format!("query\n {}", tot_sql_text));
        log(30, &format!("param\n {:?}", sql_jo));
        
        let rows = &conn.query(&tot_sql_text[..], &params[..]).unwrap();

        if rows.len() == 1{
            log(20, "postgresql db 에서 json row 가져오기 query 성공~");
            for row in rows.iter() {
                row_one = row.get(0);
                break;
            }   
        }
        
        log(30, &format!("data\n {}", row_one));

        return row_one;
    }).join().unwrap();

    return Ok(return_str);    
}


pub fn exe(
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
    sql_text: String,
    sql_jo: Vec<String>,
) -> Result<String, Error> {        
    let return_str = thread::spawn(move || {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();                            
        for column in sql_jo.iter(){
            params.push(column);
        }

        let mut row_one = String::new();
        let mut conn = db.get().unwrap();  
        log(30, &format!("query\n {}", sql_text));       
        log(30, &format!("param\n {:?}", sql_jo));

        let rows = &conn.query(&sql_text[..], &params[..]).unwrap();

        if rows.len() == 1{
            log(20, "postgresql db 에서 exe query 성공~");
            for row in rows.iter() {
                row_one = row.get(0);
                break;
            }   
        }
        
        log(30, &format!("data\n {}", row_one));

        return row_one;        
    }).join().unwrap();

    return Ok(return_str);    
}
