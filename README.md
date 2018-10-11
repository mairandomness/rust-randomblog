# rust-randomblog
A tiny rust project for building my personal blog

## Usage
### 1. The database
This project uses postgres and the name of the database is assumed to be *rustyrandomblog*, but you can change it if you'd like :)
If you want to clean your database and start it again from scratch you can run:
```sudo -u postgres dropdb DBNAMEHERE```
```sudo -u postgres createdb DBNAMEHERE```

Also run these if you ever make a change to the tables in the up.sql file and the model.rs:
```Diesel migrations run```
```Diesel migrations redo```

