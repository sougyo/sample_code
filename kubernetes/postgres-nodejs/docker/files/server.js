var port    = process.env.PORT;
var pg_host = process.env.PG_HOST;
var pg_port = process.env.PG_PORT;
var pg_db   = process.env.PG_DB;
var pg_user = process.env.PG_USER;
var pg_pass = process.env.PG_PASS;

var express = require('express');
var app = express();
app.set("view engine", "ejs");

const { Client } = require("pg");
const client = new Client({
        host:     pg_host,
        port:     pg_port,
        database: pg_db,
        user:     pg_user,
        password: pg_pass
});
client.connect();

var server = app.listen(port, () => {
  console.log("nodejs listen: " + server.address().port);
});

app.get('/', (req, res, next) => {
  client.query('select * from staff;', (err, result) => {
    res.render('index.ejs', {
      title: 'Express',
      data: result.rows
    });
  });
});

