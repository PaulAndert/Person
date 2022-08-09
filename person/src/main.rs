
extern crate mysql;
#[macro_use]
extern crate lazy_static;
use std::env;
mod print;
mod db;

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Person {
    person_id: i32,
    vorname: Option<String>,
    zweitname: Option<String>,
    nachname: Option<String>,
    geburtsname: Option<String>, 
    geburtstag: Option<String>,
    todestag: Option<String>,
}

impl Person {
    fn new () -> Person {
        return Person{
            person_id: -1,
            vorname: Some(String::new()),
            zweitname: Some(String::new()),
            nachname: Some(String::new()),
            geburtsname: Some(String::new()),
            geburtstag: Some(String::new()),
            todestag: Some(String::new()),
        };
    }
    pub fn vorname(&mut self, vorname: Option<String>) -> &mut Self {
        self.vorname = vorname;
        self
    }
    pub fn zweitname(&mut self, zweitname: Option<String>) -> &mut Self {
        self.zweitname = zweitname;
        self
    }
    pub fn nachname(&mut self, nachname: Option<String>) -> &mut Self {
        self.nachname = nachname;
        self
    }
    pub fn geburtsname(&mut self, geburtsname: Option<String>) -> &mut Self {
        self.geburtsname = geburtsname;
        self
    }
    pub fn geburtstag(&mut self, geburtstag: Option<String>) -> &mut Self {
        self.geburtstag = geburtstag;
        self
    }
    pub fn todestag(&mut self, todestag: Option<String>) -> &mut Self {
        self.todestag = todestag;
        self
    }
}

fn question(old_string: Option<String>, number: i32, question: String) -> Option<String> {
    match old_string{
        None =>  println!("[{}] {}:", number, question),
        Some(z) =>  println!("[{}] {}: {}", number, question, z),
    }
    let mut line: String = String::new();
    let n = std::io::stdin().read_line(&mut line).unwrap();
    if n <= 1 { return None; }
    else{ 
        line.pop();
        return Some(line.clone());
    }
}

fn ask_for_changes(mut p: Option<Person>) -> (Option<Person>, bool) {
    let mut line: String = String::new();
    println!("for changes type the number you want to change, else press enter");
    let n = std::io::stdin().read_line(&mut line).unwrap();
    if n <= 1 {
        return (p, false);
    } else if n >= 3 {
        line.pop();
        println!("{} is not a valid number", line);
        return (p, true);
    }else{
        match line.chars().next() {
            None => return (p, true),
            Some('1') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.vorname = question(z.vorname.clone(), 1, "Vorname".to_string()),
                }
            },
            Some('2') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.zweitname = question(z.zweitname.clone(), 2, "Zweitname".to_string()),
                }
            },
            Some('3') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.nachname = question(z.nachname.clone(), 3, "Nachname".to_string()),
                }
            },
            Some('4') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.geburtsname = question(z.geburtsname.clone(), 4, "Geburtsname".to_string()),
                }
            },
            Some('5') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.geburtstag = question(z.geburtstag.clone(), 5, "Geburtstag".to_string()),
                }
            },
            Some('6') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.todestag = question(z.todestag.clone(), 6, "Todestag".to_string()),
                }
            },
            Some(_) => {
                line.pop();
                println!("{} is not a valid number", line);
            },
        }
        return (p, true);
    }
}

fn create_new_person() -> Option<Person>{
    let mut new_person: Option<Person> = Some(Person::new());

    new_person.clone().map(|mut s| { 
        s.vorname( question(None, 1, "Vorname".to_string())); 
        s.zweitname( question(None, 2, "Zweitname".to_string()));
        s.nachname( question(None, 3, "Nachname".to_string()));
        s.geburtsname( question(None, 4, "Geburtsname".to_string()));
        s.geburtstag( question(None, 5, "Geburtstag".to_string()));
        s.todestag( question(None, 6, "Todestag".to_string()));
        s});

    let mut boo: bool = true;
    while boo{
        println!("\n\nNewly created Person:");
        print::print_person(new_person.clone());
        (new_person, boo) = ask_for_changes(new_person.clone());
    }
    return new_person;
}

fn mixed_to_single(line: String, mode: i32) -> String{ // 0 -> ABC, 1 -> 123
    let mut ret_string: String = String::new();
    for i in 0..line.len() {
        match line.chars().nth(i) {
            None => {},
            Some(a) => {
                if a.is_whitespace() { 
                    return ret_string; 
                }else if mode == 0 {
                    if a.is_alphabetic() { ret_string.push(a); }
                    else { return ret_string; }
                }else if mode == 1 {
                    if a.is_numeric() { ret_string.push(a); }
                    else { return ret_string; }
                }
            },
        }
    }
    return ret_string; 
}

fn search_person() -> Option<Person> {
    let mut boo: bool = true;
    let mut results: Vec<Person>;
    let mut search_person: Option<Person> = None;
    while boo{
        let mut line: String = String::new();
        println!("Who do you want to update? Please enter the First Name or thr Id");
        let _ = std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        match line.chars().next() {
            Some(z) if z.is_numeric() => { // One can onlyx esacpe this loop by typing a Number/Id 
                let id: i32 = mixed_to_single(line.clone(), 1).parse::<i32>().unwrap();
                if id > 0 {
                    results = db::id_to_result(id);
                    if results.len() < 1 {  println!("there is nobody with that id, please try again"); }
                    else {
                        print::print_person_names(Some(results[0].clone()));
                        line.clear();
                        println!("Is that the correct person? Press enter if true or any key if false");
                        let _ = std::io::stdin().read_line(&mut line).unwrap();
                        line.pop();
                        match line.chars().next() {
                            None => {
                                search_person = Some(results[0].clone());
                                boo = false;
                            },
                            Some(_) => {},
                        }
                    }
                }
            },
            Some(z) if z.is_alphabetic() => {
                let name: String = mixed_to_single(line, 0);
                results = db::name_to_result(name);
                if results.len() < 1 { println!("there is nobody with that name, please try again"); }
                else{ print::print_vector_person_names(results); }
            },
            _ => {},     
        }
    }
    return search_person;
}

fn main(){
    let args: Vec<_> = env::args().collect();
    let mut update: bool = false;
    if args.len() > 1 {
        if args.len() > 2 && args[2] == "-u" { update = true; }
        match args[1].as_str() {
            "-c" => {
                if update { 
                    let mut p: Option<Person> = search_person(); 
                    let mut boo: bool = true;
                    while boo{
                        print::print_person(p.clone());
                        (p, boo) = ask_for_changes(p.clone());
                    }
                    db::update_person(p.clone());
                }
                else { db::insert_person(create_new_person()); }
            },
            "-p" => {
                if update { println!("TODO update a parent - parent - child relationship"); }
                else { println!("TODO create a parent - parent - child relationship"); }
            },
            "-m" => {
                if update { println!("TODO update a marriage relationship"); }
                else { println!("TODO create a marriage relationship"); }
            },
            "-s" => {
                print::print_vector_person(db::get_all_persons());
                println!("TODO search after persons");
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