extern crate mysql;
#[macro_use]
extern crate lazy_static;
use std::env;
use dotenv::dotenv;

pub mod person;
pub mod family;
pub mod db;
pub mod graph;

pub mod matrix3;
// pub mod matrix2;

use crate::person::Person;
use crate::family::Family;

#[allow(dead_code)]
fn print_type_of<T>(_: &T) { println!("{}", std::any::type_name::<T>()) }

fn main(){
    dotenv().ok();
    let args: Vec<_> = env::args().collect();
    let mut update: bool = false;
    if args.len() > 1 {
        if args.len() > 2 && args[2] == "-u" { update = true; }
        match args[1].as_str() {
            "-a" => { 
                for p in db::get_all_persons() {
                    print!("{}", p.four_names()) 
                } 
            },
            "-f" => {
                if update { 
                    let original: Family = family::search();
                    let mut update: Family = original.clone();
                    let mut boo: bool = true;
                    while boo{
                        println!("{}", update.to_string());
                        (update, boo) = family::change(update);
                    }
                    db::update_family(original, update);
                }else { db::insert_family(family::create()) }
            },
            "-g" => {
                if args.len() > 2 && args[2] != "-u" {
                    match args[2].parse::<i32>() {
                        Ok(id) => { graph::graph(id); },
                        Err(e) => { println!("{}", e); } 
                    };
                }else {
                    graph::graph(-1);
                }
                graph::dot_to_svg();
            },
            "-h" => { 
                println!("TODO print help message") 
            },
            "-i" => {
                // import from another source ex. obsidian files

            }
            "-p" => {
                if update { 
                    let mut person: Person = person::search(); 
                    while person.person_id == -1 { person = person::search(); }
                    let mut boo: bool = true;
                    while boo{
                        println!("{}", person.to_string());
                        (person, boo) = person::change(person);
                    }
                    db::update_person(person);
                }else { 
                    let person: Person = person::create();
                    if person != Person::new() { db::insert_person(person) }
                }
            },
            "-s" => { 
                let mut person: Person = person::search(); 
                while person.person_id == -1 { person = person::search(); }
                println!("{}", person.to_string());
            },
            z => println!("{} is not a valid parameter", z),
        }
    } else { println!("No Args found, please look up the documentation for how to use this tool"); }
}

/* Usage:
cargo run 
    -p          : create a new person -> all 6 parameter
    -f          : create a new family
    -g [id]     : builds graph with optional parameter: id of root person
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