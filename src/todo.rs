//use rusqlite::{Connection, Result};
use sqlx::{ Row, SqlitePool};
//use serde_json::json;
//use std::{collections::HashMap, fmt::format};




pub async  fn list(conn: &str) 
{
    let instances = SqlitePool::connect(conn).await.unwrap();
     let qry = "SELECT * FROM todo";
     
     
    let result: Vec<_> = sqlx::query(&qry).fetch_all(&instances).await.unwrap();
     /* .into_iter()
     .map(|row| {
       json!(row
         .columns()
         .into_iter()
         .map(|column| {
           let ordinal = column.ordinal();
           let type_name = column.type_info().name();
           (
             column.name(),
             match type_name {
               "TEXT" => json!(row.get::<String, _>(ordinal)),
               "INTEGER" => json!(row.get::<i64, _>(ordinal)),
               "BOOLEAN" => json!(row.get::<bool, _>(ordinal)),
               "REAL" => json!(row.get::<f64, _>(ordinal)),
               // probably missed a few other types?
               _ => {
                 json!(format!("UNPROCESSED TYPE '{}'", type_name))
               }
             },
           )
         })
         .collect::<HashMap<_, _>>())
     })
     .collect();
   println!("{}", serde_json::to_string_pretty(&result).unwrap()); 
   
   instances.close().await; */
        
    /*let result = sqlx::query(&qry).fetch_all(&instances).await.unwrap(); */

    /*for row in result.iter() {
        for col in row.columns() {
            let value = row.try_get(1).unwrap();
            println!("{}",value)
        }
    } */

    let str_result = result
                    .iter()
                    .map(|r| format!("{} - {}",r.get::<i64,_>("id"),r.get::<String,_>("data")))
                    .collect::<Vec<String>>();
    
    for item in str_result {
        println!("{}", item);
    }

    instances.close().await; 
        

     
    
}
    
pub async  fn add(conn: &str, values: &[String]) 
{
     let instances = SqlitePool::connect(conn).await.unwrap();
     let qry = "INSERT INTO todo (data) VALUES($1)";

     for value  in values {
        let result = sqlx::query(&qry).bind(value).execute(&instances).await;
        println!("{:?}", result);
        
     } 
     instances.close().await;
     
}
    
pub async fn rm(conn: &str, val: &[String]) 
{
    let instances = SqlitePool::connect(conn).await.unwrap();

    let count_qry = "SELECT COUNT(id) FROM todo WHERE id = $1";
    for value in val {
        let count:(i32,) = sqlx::query_as(&count_qry).bind(value).fetch_one(&instances).await.unwrap();
        if count.0 > 0{
            let qry = "DELETE FROM todo WHERE id = $1";
    //println!("{qry}");
            for value in val {
                let result = sqlx::query(&qry).bind(value).execute(&instances).await;
                println!("{:?}", result);
                println!("The requested ID: {} is removed !!!",value);
            }
        }
        else {
            println!("The requested ID = {} does not exist, Please provide existing ID.",value);

        }
    }
    
    

    let qry = "SELECT * FROM todo";
     
     
    let result: Vec<_> = sqlx::query(&qry).fetch_all(&instances).await.unwrap();

    let str_result = result
                    .iter()
                    .map(|r| format!("{} - {}", r.get::<i64,_>("id"), r.get::<String,_>("data")))
                    .collect::<Vec<String>>();
    
    for item in str_result {
        println!("{}", item);
    }

    instances.close().await;

}
    
pub async fn edit(conn: &str, val: &[String], index: &String) 
{
    let instances = SqlitePool::connect(conn).await.unwrap();
    let count_qry = "SELECT COUNT(id) FROM todo WHERE id = $1";
    let count:(i32,) = sqlx::query_as(&count_qry).bind(&index).fetch_one(&instances).await.unwrap();
    //println!("{:?}",count);
    if count.0 > 0 {

            let update_qry = "UPDATE todo SET data = $1 WHERE id = $2";
    
    
            let result = sqlx::query(&update_qry).bind(&val[0]).bind(&index).execute(&instances).await;
            
            println!("{:?}",result);
            println!("The List is successfully updated!!!");
            
        }
        else  {
            println!("The requested ID = {} does not exist, Please provide existing ID.",&index);

        }
        
    //println!("{} {}",val[0], val[1]);

    let qry = "SELECT * FROM todo";
     
     
    let new_result: Vec<_> = sqlx::query(&qry).fetch_all(&instances).await.unwrap();

    let str_result = new_result
                    .iter()
                    .map(|r| format!("{} - {}",r.get::<i64,_>("id"),r.get::<String,_>("data")))
                    .collect::<Vec<String>>();
    
    for item in str_result {
        println!("{}", item);
    }

    instances.close().await;


}

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add \"buy carrots\"
    - edit [INDEX] [EDITED TASK/s]
        edits an existing task/s
        Example: todo edit 1 banana
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - reset
        deletes all tasks
    - restore 
        restore recent backup after reset
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
";
pub fn help() {
    // For readability
    println!("{}", TODO_HELP);
}
    
    



