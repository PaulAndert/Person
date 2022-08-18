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

execute rust code
```console
cargo run
```

### Flags
-p          : create a new Person
```console
cargo run -- -p
```

-r          : create a new Relation
```console
cargo run -- -r
```

-g [gen]    : graph all relations of a person, gen = how many gerenations will be displayed, default = 4
```console
cargo run -- -g 
cargo run -- -g 2
cargo run -- -g 5
```

-u          : update a person or relation, depending on the flag in front
```console
cargo run -- -p -u
cargo run -- -r -u
```

-a          : display every person in the database
```console
cargo run -- -a
```

-s          : search a specific person
```console
cargo run -- -s
```

-h          : display help message
```console
cargo run -- -h
```
