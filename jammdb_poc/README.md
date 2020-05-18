# jammdb-poc

Testing JammDB - Just Another Memory Mapped Database
[jammdb](https://github.com/pjtatlow/jammdb) is an embedded, single-file database, that allows you to store key/value pairs as bytes

# How to Run

* Run Writer to create database
```bash
cd writer
cargo run
```
* It should create database in my-database.db file
* Run Reader to read created database
```bash
cd reader
cargo run
```
* It should show the user created