// mod ast;
// mod generate;
// mod tests;

// mod typecheck;
//
mod db_core;
mod db_mysql;
use crate::{
    db_core::{Database, Operation},
    db_mysql::MySql,
};

fn main() {
    let username = "root";
    let password = "";
    let hostname = "localhost";
    let database_name = "nation";

    let mut mysql = MySql::new(username, password, hostname, database_name);
    let result = mysql.check_operation(Operation {
        file_path: "testing".to_string(),
        sql: "
        select 
            continents.name, count(*), regions.name
        from 
            continents inner join regions 
                on continents.continent_id = regions.continent_id 
        where regions.name = ?
        group by continents.name
            "
        .to_string(),
    });
    println!("result = {:#?}", result);
}
