use std::{env, process, result, error, collections};
use mysql::*;
use serde_json::*;
use mysql::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DbConnection {
    user: String,
    host: String,
    port: u16,
    database: String,
    password: String
}

struct CreateTableResult <'a>{
    pub table: String,
    pub create_table: String,
    index_keys: collections::HashMap<&'a str, Vec<Option<Vec<&'a str>>>>,
}


impl<'a> CreateTableResult<'a> {
    pub fn new (table: String, create_table: String) -> Self {
        return Self {
            table,
            create_table,
            index_keys: collections::HashMap::new(),
        }
    }

    pub fn get_ddl_keys (&'a mut self) -> () {
        let ddl_sliced: Vec<Option<Vec<&'a str>>> = self.create_table
            .split_inclusive('\n')
            .rev()
            .filter(|ddl_elemetn| {
                let trimmed_elemet = ddl_elemetn.trim();
                trimmed_elemet.starts_with("KEY")
            })
        .map(|ddl_element| {
            let trimmed_element = ddl_element.trim();
            if let (Some(start), Some(end)) = (trimmed_element.find('('), trimmed_element.rfind(')')) {
                let inside_parentheses: &str = &trimmed_element[start+1..end];
                let items: Vec<&'a str> = inside_parentheses.split(',')
                    .map(|s| s.trim_matches(|c| c == ' ' || c == '`'))
                    .collect();
                Some(items)
            } else {
                None
            }
        })
        .collect();
        self.index_keys.insert(&self.table, ddl_sliced);
    }

    pub fn get_ddl () -> result::Result<(), Box<dyn error::Error>> {
        let new_conn = DbConnection::new();

        let mut conn = new_conn.new_connection()?;

        let val: Option<(String, String)> = conn.query_first("SHOW CREATE TABLE test.rules")?;

        match val {
            Some((table, create_table)) => {
                let result = Self::new(table, create_table);
                println!("Table: {}", result.table);
                println!("DDL: {}", result.create_table);
            },
            None => process::exit(1),
        };
        Ok(())
    }
}

impl DbConnection {
    pub fn new () -> Self {
       let connection_object: DbConnection = match env::var("DB_CONFIG") {
           Ok(obj) => from_str(&obj[..]).unwrap(),
           Err(e) => {
               println!("DB_CONFIG was not found, {e}");
               process::exit(1);
           }
       };
       return connection_object;
    }

    pub fn new_connection(self) -> result::Result<Conn, mysql::Error> {
        let connection_options = Self::get_connection_options(self);

        return Conn::new(connection_options);
    }

    fn get_connection_options(self) -> OptsBuilder {
        let connection_config = OptsBuilder::new()
            .user(Some(self.user))
            .db_name(Some(self.database))
            .ip_or_hostname(Some(self.host))
            .tcp_port(self.port)
            .pass(Some(self.password));

        return connection_config;
    }
}

fn main() {
    println!("test");
}

