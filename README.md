# rust-randomblog
A tiny rust project for building my personal blog

## Things to have installed
Postgres
The Rust language

### 1?. The local database restart
This project uses postgres and the name of the database is assumed to be *rustyrandomblog*, but you can change it if you'd like :)
If you want to clean your database and start it again from scratch after populating it once, you can run:
```sudo -u postgres dropdb DBNAMEHERE```
```sudo -u postgres createdb DBNAMEHERE```

Also run these if you ever make a change to the tables in the up.sql file and the model.rs:
```diesel migration run```
```diesel migration redo```

If you can read this, I owe you the rest of the documentation for this :<
