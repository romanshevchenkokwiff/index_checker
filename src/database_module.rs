const KEY_MATCH_START: [&str; 2] = ["UNIQUE KEY", "KEY"];
const CLOUSURES: [&str; 2] = ["where", "and"];
const CLOUSURES_OPPERATORS: [&str; 7] = ["=", "in", "<", ">", "=<", "=>", "!="];

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

    #[derive(Clone, Debug)]
    pub struct CreateTableResult {
        pub table: String,
        pub index_keys: collections::HashMap<String, Vec<Option<Vec<String>>>>,
        create_table: String,
    }

    #[derive(Debug)]
    pub struct QueryParse {
        pub keys: Vec<String>,
        pub create_table_result: CreateTableResult
    }

    impl CreateTableResult {
        fn get_ddl (table_name: String) -> result::Result<Self, Box<dyn error::Error>> {
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

        pub fn get_ddl_keys(table_name: String) -> Self {
            let mut ddl = Self::get_ddl(table_name).unwrap();

            let ddl_sliced: Vec<Option<Vec<String>>> = ddl
                .create_table.clone()
                .split_inclusive('\n')
                .rev()
                .filter(|ddl_element| {
                    let trimmed_element = ddl_element.trim();
                    super::KEY_MATCH_START.iter().any(|start_point| trimmed_element.starts_with(start_point))
                })
            .map(|ddl_element| {
                let trimmed_element = ddl_element.trim();
                if let (Some(start), Some(end)) =
                    (trimmed_element.find('('), trimmed_element.rfind(')'))
                    {
                        let inside_parentheses: &str = &trimmed_element[start + 1..end];
                        let items: Vec<String> = inside_parentheses
                            .split(',')
                            .map(|s| String::from(s.trim_matches(|c| c == ' ' || c == '`')))
                            .collect();
                        Some(items)
                    } else {
                        None
                    }
            })
            .collect();

            ddl.index_keys.insert(ddl.table.clone(), ddl_sliced);
            return ddl;
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
                Err(_) => {
                    println!("DB_CONFIG was not found");
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

    impl QueryParse {
        pub fn get_keys(raw_sql: String, create_table_result: CreateTableResult) -> Self {
            let mut raw_split: Vec<&str> = raw_sql.split(' ').collect();
            let position_of_where_key: usize = raw_split.clone().into_iter().position(|key| key == "where").unwrap();
            let logic_slice: Vec<&str> = raw_split.split_off(position_of_where_key);

            let mut search_keys: Vec<String> = vec![];

            for (index, sql_clousure) in logic_slice.iter().enumerate() {
                if super::CLOUSURES.iter().find(|&closure| closure == sql_clousure) != None  {
                    let key_with_quotes = logic_slice[index+1];
                    let key = key_with_quotes.trim_matches('\"');
                    search_keys.push(String::from(key));
                }; 
            }

            return QueryParse {
                keys: search_keys,
                create_table_result,
            }
        }
    }
}

