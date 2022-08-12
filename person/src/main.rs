extern crate mysql;
#[macro_use]
extern crate lazy_static;
use std::env;

pub mod person;
pub mod parent;
pub mod db;

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn mixed_to_single(line: String, mode: i32) -> (String, Option<String> ){ // 0 -> ABC, 1 -> 123
    let mut first: String = String::new();
    let mut second: String = String::new();
    let mut boo: bool = true;

    for i in 0..line.len() {
        match line.chars().nth(i) {
            None => {},
            Some(a) => {
                if mode == 0 { // text mode
                    if a.is_alphabetic() { 
                        if boo { first.push(a) }
                        else { second.push(a) }
                    }else if a.is_whitespace() { boo = false }
                }else if mode == 1 && a.is_numeric() { first.push(a) }
            },
        }
    }
    if second.is_empty() { return (first, None) }
    else { return (first, Some(second)) }
}

fn main(){
    let args: Vec<_> = env::args().collect();
    let mut update: bool = false;
    if args.len() > 1 {
        if args.len() > 2 && args[2] == "-u" { update = true; }
        match args[1].as_str() {
            "-c" => {
                if update { 
                    let mut p: Option<person::Person> = person::search(); 
                    let mut boo: bool = true;
                    while boo{
                        person::print(p.clone());
                        (p, boo) = person::change(p.clone());
                    }
                    db::update_person(p.clone());
                }
                else { db::insert_person(person::create()); }
            },
            "-p" => {
                if update { println!("TODO update a parent - parent - child relationship"); }
                else { 
                    let p: Option<parent::Parent> = parent::create();
                    parent::print(p);
                }
            },
            "-m" => {
                if update { println!("TODO update a marriage relationship"); }
                else { println!("TODO create a marriage relationship"); }
            },
            "-s" => {
                person::print(person::search());
            }
            "-h" => {
                println!("TODO print help message");
            },
            z => println!("{} is not a valid parameter", z),
        }
    } else { println!("No Args found, please look up the documentation for how to use this tool"); }


    // let p1: Option<Person> = create_new_person();
    // print_person(p1);
    // Person{
    //     person_id: -1, 
    //     vorname: Some("Peter".to_string()), 
    //     zweitname: Some("zwei".to_string()), 
    //     nachname: Some("Neu".to_string()), 
    //     geburtsname: Some("Alt".to_string()),
    //     geburtstag: Some("01.01.2000".to_string()),
    //     todestag: None,
    // };
    // let p: Option<Person> = create_new_person();
    // insert_person(p);
    // let all: Vec<Person> = get_all_persons();
    // print_vector_person(all);
    //print_person(Some(all[0].clone()));
}

/* Usage:
cargo run 
    -c : create a new person -> all 6 parameter
    -p : create a new parent - parent - child relationship
    -m : create a new marriage relationship
    -u : update a given thing (only in combination with another mode)
    -s : search a person
    -h : help -> shows this and / or more
    
Examples:
    cargo run -c : new person
    cargo run -c -u : update a person
    cargo run -h : show help
    ...
*/