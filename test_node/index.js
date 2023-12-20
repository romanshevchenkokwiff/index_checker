const knex = require('knex');
const workerUrl = new URL('./db_handler.js', import.meta.url).href;
const worker = new Worker(workerUrl);
const test = require('../.');

const config = {
  client: 'mysql2',
  connection: {
    host : '127.0.0.1',
    port : 3306,
    user : 'root',
    password : 'test',
    database : 'test'
  }
};

const connection = knex(config);

connection.on('query', (data) => {
  console.log('on query event test');
  console.log(data);
  test.get_initial_params(data.sql);
  worker.postMessage(data.sql);
  worker.onmessage = event => console.log(event)
});

connection.on('query-error', (data) => {
  console.log('on query event test error');
  console.log(data);
});

//await connection.schema.createTableIfNotExists('newTest', (table) => {
//  table.increments('id');
//  table.string('name');
//  table.integer('int');
//});
//
//await connection('newTest').insert({name: 'test', int: 1234});
//
//await connection.schema.alterTable('newTest', (table) => {
//  table.index(['id', 'name']);
//})

await connection('newTest').select('*');

process.exit(0);
