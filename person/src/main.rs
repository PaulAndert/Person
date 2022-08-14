extern crate mysql;
#[macro_use]
extern crate lazy_static;
use std::env;
use std::io::Write;

pub mod person;
pub mod relation;
pub mod db;

use crate::person::Person;
use crate::relation::Relation;

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

fn get_all_nodes() -> String{
    let mut ret: String = String::new();
    let net = Some(db::id_to_person(3)[0].clone());
    let rela: Relation = db::person_to_relations(net)[0].clone();

    match rela.male{
        None => {},
        Some(z) => { 
            ret.push_str("male [shape=square, label=\"");
            ret.push_str(&person::vor_zweit_nach(Some(z.clone())));
            ret.push_str("\n");
            match z.geburtstag{ 
                None => {},
                Some(y) => {ret.push_str(&y)},
            }
            ret.push_str("\",color=\"blue\", pos=\"0,2!\"];\n");
        },
    }

    match rela.female{
        None => {},
        Some(z) => { 
            ret.push_str("female [shape=circle, label=\"");
            ret.push_str(&person::vor_zweit_nach(Some(z.clone())));
            ret.push_str("\n");
            match z.geburtstag{ 
                None => {},
                Some(y) => {ret.push_str(&y)},
            }
            ret.push_str("\",color=\"pink\", pos=\"2,2!\"];\n");
        },
    }
    match rela.child{
        None => {},
        Some(z) => { 
            ret.push_str("child [shape=square, label=\"");
            ret.push_str(&person::vor_zweit_nach(Some(z.clone())));
            ret.push_str("\n");
            match z.geburtstag{ 
                None => {},
                Some(y) => {ret.push_str(&y)},
            }
            ret.push_str("\",color=\"blue\", pos=\"1,0!\"];\n");
        },
    }

    ret.push_str("t01 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"0,1!\"];\nt11 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"1,1!\"];\nt21 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"2,1!\"];\n");
    ret.push_str("male -> t01\nfemale -> t21\nt01 -> t11 -> t21\nt11 -> child\n");

    return ret;
}

fn graph(){
    let gra: String = String::from("digraph P {\nedge [dir=none];\nnode [shape=box];\n");

    let mut file = std::fs::File::create("data.dot").expect("create failed");

    file.write_all(gra.as_bytes()).expect("write failed");
    file.write_all(get_all_nodes().as_bytes()).expect("write failed");
    file.write_all("}".as_bytes()).expect("write failed");
}

fn main(){
    let args: Vec<_> = env::args().collect();
    let mut update: bool = false;
    if args.len() > 1 {
        if args.len() > 2 && args[2] == "-u" { update = true; }
        match args[1].as_str() {
            "-g" => {
                graph();
            }
            "-c" => {
                if update { 
                    let mut p: Option<Person> = person::search(); 
                    let mut boo: bool = true;
                    while boo{
                        person::print(p.clone());
                        (p, boo) = person::change(p.clone());
                    }
                    db::update_person(p.clone());
                }else { db::insert_person(person::create()); }
            },
            "-r" => {
                if update { 
                    let original: Option<Relation> = relation::search();
                    let mut update: Option<Relation> = original.clone();
                    let mut boo: bool = true;
                    while boo{
                        relation::print(update.clone());
                        (update, boo) = relation::change(update.clone());
                    }
                    db::update_relation(original.clone(), update.clone());
                }else { db::insert_relation(relation::create()) }
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
    -r : create a new relation
    -u : update a given thing (only in combination with another mode)
    -s : search a person
    -h : help -> shows this and / or more
    
Examples:
    cargo run -c : new person
    cargo run -c -u : update a person
    cargo run -h : show help
    ...
*/