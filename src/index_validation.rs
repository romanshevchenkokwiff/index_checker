const CLOUSURES: [&str; 2] = ["where", "and"];
pub mod index_validation {
    use crate::CreateTableResult;
    #[derive(Debug)]
    pub struct QueryParse {
        pub keys: Vec<String>,
        pub create_table_result: CreateTableResult
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

        pub fn get_table_name (raw_sql: String) -> String {
            let tokens: Vec<&str> = raw_sql.split_whitespace().collect();
            let mut table_name = String::new();

            for (i, token) in tokens.iter().enumerate() {
                if token.to_lowercase() == "from" && i + 1 < tokens.len() {
                    table_name = tokens[i + 1].trim_matches('"').to_string();
                    break;
                }
            }

            table_name
        }
    }
}
