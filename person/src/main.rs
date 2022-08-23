extern crate mysql;
#[macro_use]
extern crate lazy_static;
use std::env;

pub mod person;
pub mod relation;
pub mod db;
pub mod graph;

pub mod matrix;

use crate::person::Person;
use crate::relation::Relation;

#[allow(dead_code)]
fn print_type_of<T>(_: &T) { println!("{}", std::any::type_name::<T>()) }

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
            "-a" => { for p in db::get_all_persons() { person::print_all_4_names(p) } },
            "-g" => {
                if args.len() > 2 && match args[2].clone().chars().nth(0){ None => false, Some(z) => z.is_numeric(),} {
                    let (number , _) = mixed_to_single(args[2].clone(), 1);
                    graph::graph(number.parse::<i32>().unwrap());
                } else { graph::graph(-1) }
                graph::dot_to_svg();
            },
            "-p" => {
                if update { 
                    let mut person: Person = person::search(); 
                    let mut boo: bool = true;
                    while boo{
                        person::print(person.clone());
                        (person, boo) = person::change(person);
                    }
                    db::update_person(person);// TODO entweder match p.clone() oder mach deren return type anders von search
                }else { db::insert_person(person::create()) }
            },
            "-r" => {
                if update { 
                    let original: Relation = relation::search();
                    let mut update: Relation = original.clone();
                    let mut boo: bool = true;
                    while boo{
                        relation::print(update.clone());
                        (update, boo) = relation::change(update);
                    }
                    db::update_relation(original, update);
                }else { db::insert_relation(relation::create()) }
            },
            "-s" => { person::print( person::search() ) },
            "-h" => { println!("TODO print help message") },
            z => println!("{} is not a valid parameter", z),
        }
    } else { println!("No Args found, please look up the documentation for how to use this tool"); }
}

/* Usage:
cargo run 
    -p          : create a new person -> all 6 parameter
    -r          : create a new relation
    -g [gens]   : builds graph with optional parameter of how many generations deep, default = 4
    -u          : update a given thing (only in combination with another mode)
    -s          : search a person
    -h          : help -> shows this and / or more
    
Examples:
    cargo run -p : new person
    cargo run -p -u : update a person
    cargo run -g : builds default graph
    cargo run -g 3 : builds 3 generations deep graph
    cargo run -h : show help
    ...
*/