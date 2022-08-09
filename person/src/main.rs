
extern crate mysql;
#[macro_use]
extern crate lazy_static;
use std::env;
mod print;

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
}

lazy_static! {
    static ref POOL : mysql::Pool = mysql::Pool::new("mysql://root:Gravure1247@localhost:3306/person").unwrap();
}

fn get_all_persons() -> Vec<Person>{

    return POOL.prep_exec("SELECT * from person", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (person_id, vorname, zweitname, nachname, geburtsname, geburtstag, todestag) = mysql::from_row(row);

                Person {
                    person_id,
                    vorname,
                    zweitname,
                    nachname,
                    geburtsname,
                    geburtstag,
                    todestag,
                }
            }).collect()
        }).unwrap(); // Unwrap `Vec<Person>`
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

fn ask_for_changes(mut p: Person) -> (Person, bool) {
    let mut line: String = String::new();
    println!("for changes type the number you want to change, else press enter");
    let n = std::io::stdin().read_line(&mut line).unwrap();
    
    if n <= 1 {
        return (p, true);
    } else if n >= 3 {
        line.pop();
        println!("{} is not a valid number", line);
        return (p, false);
    }else{
        match line.chars().next() {
            None => return (p, false),
            Some('1') => p.vorname = question(p.vorname, 1, "Vorname".to_string()),
            Some('2') => p.zweitname = question(p.zweitname, 2, "Zweitname".to_string()),
            Some('3') => p.nachname = question(p.nachname, 3, "Nachname".to_string()),
            Some('4') => p.geburtsname = question(p.geburtsname, 4, "Geburtsname".to_string()),
            Some('5') => p.geburtstag = question(p.geburtstag, 5, "Geburtstag".to_string()),
            Some('6') => p.todestag = question(p.todestag, 6, "Todestag".to_string()),
            Some(_) => {
                line.pop();
                println!("{} is not a valid number", line);
            },
        }
        return (p, false);
    }
}

fn create_new_person() -> Option<Person>{
    let mut new_person: Person = Person::new();
    
    new_person.vorname = question(None, 1, "Vorname".to_string());
    new_person.zweitname = question(None, 2, "Zweitname".to_string());
    new_person.nachname = question(None, 3, "Nachname".to_string());
    new_person.geburtsname = question(None, 4, "Geburtsname".to_string());
    new_person.geburtstag = question(None, 5, "Geburtstag".to_string());
    new_person.todestag = question(None, 6, "Todestag".to_string());

    let mut boo: bool = false;
    while boo == false{
        println!("\n\nNewly created Person:");
        print::print_person(Some(new_person.clone()));
        (new_person, boo) = ask_for_changes(new_person.clone());
    }

    return Some(new_person);
}

#[allow(unused_must_use)]
fn insert_new_person(person: Option<Person>){
    match person {
        None => println!("No person found"),
        Some(person) => {
            let mut insert: String = String::from("INSERT INTO person (vorname, zweitname, nachname, geburtsname, geburtstag, todestag) VALUES (\"");

            match person.vorname {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\", \"");
            match person.zweitname {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\", \"");
            match person.nachname {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\", \"");
            match person.geburtsname {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\", \"");
            match person.geburtstag {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\", \"");
            match person.todestag {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\");");
            
            POOL.prep_exec(insert, ());
        },
    };

}

fn name_to_result(name: String) -> Vec<Person>{
    let mut selec: String = String::from("SELECT * from person where vorname like '%");
    selec.push_str(&name);
    selec.push_str("%';");

    return POOL.prep_exec(selec, ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (person_id, vorname, zweitname, nachname, geburtsname, geburtstag, todestag) = mysql::from_row(row);

                Person {
                    person_id,
                    vorname,
                    zweitname,
                    nachname,
                    geburtsname,
                    geburtstag,
                    todestag,
                }
            }).collect()
        }).unwrap();
}

fn id_to_result(id: i32) -> Vec<Person>{
    let mut selec: String = String::from("SELECT * from person where person_id = ");
    selec.push_str(&id.to_string());
    selec.push(';');

    return POOL.prep_exec(selec, ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (person_id, vorname, zweitname, nachname, geburtsname, geburtstag, todestag) = mysql::from_row(row);

                Person {
                    person_id,
                    vorname,
                    zweitname,
                    nachname,
                    geburtsname,
                    geburtstag,
                    todestag,
                }
            }).collect()
        }).unwrap();
}



fn mixed_To_single(line: String, mode: i32) -> String{ // 0 -> ABC, 1 -> 123
    let mut retString: String = String::new();
    for i in 0..line.len() {
        match line.chars().nth(i) {
            None => {},
            Some(a) => {
                if a.is_whitespace() { 
                    return retString; 
                }else if mode == 0 {
                    if a.is_alphabetic() { retString.push(a); }
                    else { return retString; }
                }else if mode == 1 {
                    if a.is_numeric() { retString.push(a); }
                    else { return retString; }
                }
            },
        }
    }
    return retString; 
}

fn search_person(){ // -> Option<Person> {

    let mut boo: bool = false;
    let mut results: Vec<Person>;
    while boo == false{
        let mut line: String = String::new();
        println!("Who do you want to update? Please enter the First Name or thr Id");
        let _n = std::io::stdin().read_line(&mut line).unwrap();
        line.pop();

        match line.chars().next() {
            Some(z) if z.is_numeric() => { // One can onlyx esacpe this loop by typing a number/Id 
                let a: i32 = mixed_To_single(line.clone(), 1).parse::<i32>().unwrap();
                println!("{}", a);

                // weiter zu print person with id and then update that person (check if ist a valid number)
                if a > 0 {
                    results = id_to_result(a);
                    if results.len() < 1 {
                        println!("there is nobody with that id, please try again");
                    }else{
                
                        print::print_vector_person_names(results);

                        line.clear();
                        println!("Is that the correct person? Press enter if true or any key if false");
                        let _n = std::io::stdin().read_line(&mut line).unwrap();
                        line.pop();

                        match line.chars().next() {
                        None => {
                            println!("go on");
                            boo = true;
                        },
                        Some(_) => {},
                        }
                    }
                }
            },
            Some(z) if z.is_alphabetic() => {
                let a: String = mixed_To_single(line, 0);
                //println!("{}", a);
                results = name_to_result(a);
                if results.len() < 1 {
                    println!("there is nobody with that name, please try again");
                }else{
            
                    print::print_vector_person_names(results);
            
                    // println!("Who do you want to update? Please enter the Number or an new first name");
            
                    // let mut line2: String = String::new();
                    // let _n = std::io::stdin().read_line(&mut line2).unwrap();
                    // line2.pop();
                }
            },
            _ => {},     
        }

    }

    
    // abfragen was jetzt eingegeben wurde und dann weiter machen 


}

fn main(){

    let args: Vec<_> = env::args().collect();
    let mut update: bool = false;
    if args.len() > 1 {
        if args.len() > 2 && args[2] == "-u" { update = true; }
        match args[1].as_str() {
            "-c" => {
                if update { search_person(); }
                else { insert_new_person(create_new_person()); }
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
                print::print_vector_person(get_all_persons());
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

    // insert_new_person(p);

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