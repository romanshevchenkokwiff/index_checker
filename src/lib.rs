use neon::prelude::*;

mod database_module;
use database_module::database_module::*;

fn get_initial_params(mut cx: FunctionContext) -> JsResult<JsNull> {

    let raw_table_name = cx.argument::<JsString>(0).unwrap();
    let raw_table_sql = cx.argument::<JsString>(1).unwrap();

    let table_name: String = raw_table_name.value(&mut cx);
    let table_sql: String = raw_table_sql.value(&mut cx);

    println!("table name: {}\ntable sql: {}\n", table_name, table_sql);

    let table_result: CreateTableResult = CreateTableResult::get_ddl_keys(table_name);
    let query_keys: QueryParse = QueryParse::get_keys(table_sql, table_result);

    println!("Stored query keys {:#?}", query_keys);

    Ok(cx.null())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get_initial_params", get_initial_params)?;
    Ok(())
}
