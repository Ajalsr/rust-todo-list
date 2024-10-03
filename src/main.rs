//use rusqlite::{params, Connection, Result};
use std::result::Result;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};
use std::env;
mod todo;



async fn create_schema(db_url: &str) -> Result<SqliteQueryResult,sqlx::Error> {
    let pool = SqlitePool::connect(&db_url).await?;

    let qry = 
    "PRAGMA foreign_keys = ON;
     CREATE TABLE IF NOT EXISTS todo
     (
        id          INTEGER PRIMARY KEY NOT NULL,
        data        TEXT                NOT NULL,
        created_on  DATETIME DEFAULT    (datetime('now', 'localtime')),
        updated_on  DATETIME DEFAULT    (datetime('now', 'localtime')),
        done        BOOLEAN             NOT NULL DEFAULT 0
     );
     ";
     let result = sqlx::query(&qry).execute(&pool).await;
     pool.close().await;
     return result;
}

#[async_std::main]
async fn main()  {
    let db_url = String::from("sqlite://sqlite.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false){
        Sqlite::create_database(&db_url).await.unwrap();
        match create_schema(&db_url).await{
            Ok(_) => println!("Database created successfully"),
            Err(e) => panic!("{}",e)
        }
    }
   let args:Vec<String> = env::args().collect();

   
    
    
   
   //let separated_args: String = long_string.replace(",","");
   
   

   //println!("{:?}",long_string);
   if args.len() > 1 {
    if &args[1] == "edit" {
        let index = &args[2];
        let long_string = args[3..].join(" ");
        let separated_comma:Vec<&str> = long_string.split(",").collect();
        //println!("{:?}",separated_comma);
        let long_strings:Vec<String> = separated_comma.iter().map(|r| r.replace(",", "").trim().to_string()).collect();
        todo::edit(&db_url, &long_strings[..], &index).await;
    }
     else {
        let long_string = args[2..].join(" ");
    let separated_comma:Vec<&str> = long_string.split(",").collect();
    //println!("{:?}",separated_comma);
    let long_strings:Vec<String> = separated_comma.iter().map(|r| r.replace(",", "").trim().to_string()).collect();
   
    //println!("{:?}",args[1]);
    let command = &args[1];
    match &command[..] {
        "list" => todo::list(&db_url).await,
        "add" => todo::add(&db_url, &long_strings[..]).await,
        "rm" => todo::rm(&db_url, &args[2..]).await,
        "help" | "--help" | "-h" | _ => todo::help(),
    }

     }
    
   } else {
    
    todo::list(&db_url).await;
   }

   
}














