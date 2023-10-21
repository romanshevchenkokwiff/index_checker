use std::{env, fs, process, result, error, collections};
use glob::glob;

const PATTERN: &str = "/*.ts";
const CONNECTION_STRING: &str = "mysql://root:root@localhost:3306/";

struct CreateTableResult <'a> {
    pub table: String,
    pub create_table: String,
    index_keys: collections::HashMap<&'a str, Vec<Option<Vec<&'a str>>>>,
}


impl CreateTableResult <'static> {
    pub fn new (table: String, create_table: String) -> Self {
        return Self {
            table,
            create_table,
            index_keys: collections::HashMap::new(),
        }
    }

    pub fn get_ddl_keys <'a>(&'a mut self) -> () {
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
}

mod scan_dir {
    use mysql::*;
    use mysql::prelude::*;

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
        let content = super::fs::read_to_string(file_to_read).expect("No content was found");

        println!("reading file: {}", &file_to_read);
        println!("files content: {}", content);
    }

    pub fn get_ddl () -> super::result::Result<(), Box<dyn super::error::Error>> {
        let pool = Pool::new(super::CONNECTION_STRING)?;
        let mut conn = pool.get_conn()?;

        // Fetch result as a tuple
        let val: Option<(String, String)> = conn.query_first("SHOW CREATE TABLE test.rules")?;

        match val {
            Some((table, create_table)) => {
                let result = super::CreateTableResult::new(table, create_table);
                println!("Table: {}", result.table);
                println!("DDL: {}", result.create_table);
            },
            None => super::process::exit(1),
        };

        Ok(())
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
    scan_dir::read_file(&file_list[1]);
    //scan_dir::get_ddl().unwrap();
    let create_table_stmt = "
        CREATE TABLE `rules` (
          `id` int unsigned NOT NULL AUTO_INCREMENT,
          `createdAt` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
          `updatedAt` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
          `name` varchar(255) NOT NULL,
          `active` tinyint(1) NOT NULL DEFAULT '0',
          `startDate` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
          `endDate` timestamp NULL DEFAULT NULL,
          PRIMARY KEY (`id`),
          KEY `rules_enddate_index` (`endDate`),
          KEY `rules_startdate_index` (`startDate`, `testsasdasd`)
          KEY `rules_startdate_index`
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci";
    //println!("{:#?}");

}

