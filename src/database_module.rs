const KEY_MATCH_START: [&str; 2] = ["UNIQUE KEY", "KEY"];

pub mod database_module {
    use std::{env, process, result, error, collections};
    use mysql::*;
    use serde_json::*;
    use serde::{Serialize, Deserialize};
    use mysql::prelude::*;
    extern crate dotenv;
    use dotenv::dotenv;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DbConnection {
        user: String,
        host: String,
        port: u16,
        database: String,
        password: String,
    }

    pub struct CreateTableResult <'a>{
        pub table: String,
        pub index_keys: collections::HashMap<&'a str, Vec<Option<Vec<&'a str>>>>,
        create_table: String,
    }

    impl<'a> CreateTableResult <'a> {
        pub fn new (table: String, create_table: String) -> Self {
            return Self {
                table,
                create_table,
                index_keys: collections::HashMap::new(),
            }
        }
        pub fn get_ddl_keys (&'a mut self) -> &collections::HashMap<&'a str, Vec<Option<Vec<&'a str>>>> {
            let ddl_sliced: Vec<Option<Vec<&'a str>>> = self.create_table
                .split_inclusive('\n')
                .rev()
                .filter(|ddl_element| {
                    let trimmed_element = ddl_element.trim();
                    super::KEY_MATCH_START.iter().any(|start_point| trimmed_element.starts_with(start_point))
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
            & self.index_keys
        }

        pub fn get_ddl (table_name: String) -> result::Result<Self, Box<dyn error::Error>> {
            let new_conn = DbConnection::new();
            let mut connection = new_conn.new_connection()?;
            let query: String = String::from("SHOW CREATE TABLE ") + &table_name[..];
            let val: Option<(String, String)> = connection.query_first(query)?;

            match val {
                Some((table, create_table)) => { 
                    println!("Table DDL: \n{}", create_table);
                    return Ok(Self::new(table, create_table));
                },
                None => {
                    println!("Wasn't able to get DDL");
                    process::exit(1);
                },
            };
        }
    }

    impl DbConnection {
        pub fn new () -> Self {
            dotenv().ok();
            let connection_object: DbConnection = match env::var("DB_CONFIG") {
                Ok(obj) => {
                    let test = from_str(&obj[..]).unwrap();
                    println!("\nOBJ{:#?}", test);
                    test
                },
                Err(e) => {
                    println!("DB_CONFIG was not found");
                    process::exit(1);
                }
            };
            return connection_object;
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

        fn new_connection(self) -> result::Result<Conn, mysql::Error> {
            let connection_options = Self::get_connection_options(self);

            return Conn::new(connection_options);
        }
    }
}
