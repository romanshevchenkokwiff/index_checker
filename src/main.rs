use std::{env, process, result, error, collections};
use glob::glob;
use mysql::*;
use mysql::prelude::*;

mod parser;

const PATTERN: &str = "/*.ts";
const CONNECTION_STRING: &str = "mysql://root:root@localhost:3306/";

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
        let pool = Pool::new(CONNECTION_STRING)?;
        let mut conn = pool.get_conn()?;

        // Fetch result as a tuple
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

mod scan_dir {

    pub fn get_files(file_path: String) -> Vec<String> {
        let full_path: String = file_path + super::PATTERN;
        let mut entry_list: Vec<String> = vec!();
        println!("full_path {}", full_path);

        for entry in super::glob(&full_path[..]).unwrap() {
            let test_path: String = match entry {
                Ok(path) => path.display().to_string(),
                Err(e) => {
                    println!("{:?}", e);
                    super::process::exit(1);
                }
            };
            entry_list.push(test_path);
        };
        return entry_list;
    }

    pub fn read_file (file_to_read: &String) {
        //TODO https://github.com/Boshen/javascript-parser-in-rust
    }

    //fn check_index () {};

}

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    let args: Option<&String> = raw_args.get(1);
    println!("test args {:?}", args);
    let file_path: String = match args {
        Some(arg) => arg.clone(),
        None => {
            println!("No arguments was passed");
            process::exit(1);
        },
    };
    let file_list = scan_dir::get_files(file_path);
    println!("etries: {:?}", file_list);
    //scan_dir::get_ddl().unwrap();
    //println!("{:#?}");
}

