# Person
A Rust CLI where you can create persons, connect them in familys and graph those relationships as a family tree graph.

# Requirements
- Mysql as the Database    
- [Graphviz](https://graphviz.org) to display the graph  

# Instalation
Clone the Git Repository
```console
git clone https://github.com/PaulAndert/Person.git
```
  
create a new database via the shema in db.sql  
  
create a .env file with the following shema
```console
DB_IP="your_db_id"
DB_PORT="your_db_port"
DB_USER="your_db_user_name"
DB_PASSWORD="your_db_user_password"
DB_TABLE="your_db_table_name"
```  
  
# Compile Code
build the rust code to a executable
```console
cargo build
```
-> its in target/debug/person

then run it
```console
./target/debug/person <flags>
```

Optionally you can run th executable via cargo
```console
cargo run -- <flags>
```
then you need to replace "./target/debug/person " with "cargo run -- "  
because the flag needs to go to the code not to cargo

### Flags
-p          : create a new Person
```console
./target/debug/person -p
```

-r          : create a new Family
```console
./target/debug/person -f
```

-g [id]    : graph all relations of a person, id = optional: id of root person  
```console
./target/debug/person -g 
./target/debug/person -g 2
./target/debug/person -g 5
```

-u          : update a person or family, depending on the flag it's paired with
```console
./target/debug/person -p -u
./target/debug/person -f -u
```

-a          : display every person in the database
```console
./target/debug/person -a
```

-s          : search a specific person
```console
./target/debug/person -s
```

-h          : display help message
```console
./target/debug/person -h
```
