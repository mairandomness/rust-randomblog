# rust-randomblog
A tiny rust project for building my personal blog

## Things to have installed
Postgres
libpq
The Rust language
Diesel cli

### 1?. The local database restart
This project uses postgres and the name of the database is assumed to be *rustyrandomblog*, but you can change it if you'd like :)

If you want to clean your database and start it again from scratch after populating it once, you can run this to delete a database:
```sudo -u postgres dropdb DBNAMEHERE```

And to create a new one:
```sudo -u postgres createdb DBNAMEHERE```

Set up a .env file with the database url

And run ```diesel migration run``` so Diesel will create the tables according to `model.rs`

Also run these if you ever make a change to the tables in the up.sql file and the model.rs:
```diesel migration run```
```diesel migration redo```

If you can read this, I owe you the rest of the documentation for this :<
