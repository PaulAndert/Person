# Person

A Rust CLI where you can create persons, connect them in relationsships and graph those relationships as a family tree graph.

Mysql - to permanently save Persons and Relations  
Rust - language  
Graphviz - to graph the relations  

# Instalation

Clone the Git Repository
```console
git clone https://github.com/PaulAndert/Person.git
```

move into the folder
```console
cd person/person
```

copy the db.sql and execute it on a mysql server  

go to src/db.rs and modify the string line 4 like:  
mysql://user_name:password@server_ip:port/person  
server_ip = mysql server ip OR localhost if running on local maschine  
port = 3306 if mysql is in default configuration  

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
cargo run
```
then you need to replace "./target/debug/person " with "cargo run -- "  
because the flag needs to go to the code not to cargo

### Flags
-p          : create a new Person
```console
./target/debug/person -p
```

-r          : create a new Relation
```console
./target/debug/person -r
```

-g [gen]    : graph all relations of a person, gen = how many gerenations will be displayed, default = 4
```console
./target/debug/person -g 
./target/debug/person -g 2
./target/debug/person -g 5
```

-u          : update a person or relation, depending on the flag in front
```console
./target/debug/person -u
./target/debug/person -u
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
