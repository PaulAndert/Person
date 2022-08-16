// use super::db::*;
// use crate::mixed_to_single;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Person {
    pub person_id: i32,
    pub vorname: Option<String>,
    pub zweitname: Option<String>,
    pub nachname: Option<String>,
    pub geburtsname: Option<String>, 
    pub gender: Option<String>, 
    pub geburtstag: Option<String>,
    pub todestag: Option<String>,
}

impl Person {
    fn new () -> Person {
        return Person{
            person_id: -1,
            vorname: Some(String::new()),
            zweitname: Some(String::new()),
            nachname: Some(String::new()),
            geburtsname: Some(String::new()),
            gender: Some(String::new()),
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
    pub fn gender(&mut self, gender: Option<String>) -> &mut Self {
        self.gender = gender;
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

pub fn change(mut p: Option<Person>) -> (Option<Person>, bool) {
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
                    Some(ref mut z) => z.gender = question(z.gender.clone(), 5, "Gender".to_string()),
                }
            },
            Some('6') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.geburtstag = question(z.geburtstag.clone(), 6, "Geburtstag".to_string()),
                }
            },
            Some('7') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.todestag = question(z.todestag.clone(), 7, "Todestag".to_string()),
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

pub fn create() -> Option<Person>{
    let mut new_person: Option<Person> = Some(Person::new());

    new_person.clone().map(|mut s| { 
        s.vorname( question(None, 1, "Vorname".to_string())); 
        s.zweitname( question(None, 2, "Zweitname".to_string()));
        s.nachname( question(None, 3, "Nachname".to_string()));
        s.geburtsname( question(None, 4, "Geburtsname".to_string()));
        s.gender( question(None, 5, "Gender".to_string()));
        s.geburtstag( question(None, 6, "Geburtstag".to_string()));
        s.todestag( question(None, 7, "Todestag".to_string()));
        s});

    let mut boo: bool = true;
    while boo{
        println!("\n\nNewly created Person:");
        print(new_person.clone());
        (new_person, boo) = change(new_person.clone());
    }
    return new_person;
}

pub fn search() -> Option<Person> {
    let mut boo: bool = true;
    let mut results: Vec<Person>;
    let mut search: Option<Person> = None;
    while boo{
        let mut line: String = String::new();
        println!("Who are you searching? Please enter the First Name and/or Last Name or their Id");
        let _ = std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        match line.chars().next() {
            Some(z) if z.is_numeric() => { // One can only esacpe this loop by typing a Number/Id 
                let(a, _) = crate::mixed_to_single(line.clone(), 1);
                let id: i32 = a.parse::<i32>().unwrap();
                if id > 0 {
                    results = crate::db::id_to_person(id);
                    if results.len() < 1 {  println!("there is nobody with that id, please try again"); }
                    else {
                        print_names(Some(results[0].clone()));
                        line.clear();
                        println!("Is that the correct person? Press enter if true or any key if false");
                        let _ = std::io::stdin().read_line(&mut line).unwrap();
                        line.pop();
                        match line.chars().next() {
                            None => {
                                search = Some(results[0].clone());
                                boo = false;
                            },
                            Some(_) => {},
                        }
                    }
                }
            },
            Some(z) if z.is_alphabetic() => {
                let (a, b) = crate::mixed_to_single(line, 0);
                match b{
                    None => results = crate::db::single_name_person(a),
                    Some(z) => results = crate::db::double_name_person(a, z),
                }
                if results.len() < 1 { println!("there is nobody with that name, please try again"); }
                else{ print_vector_names(results); }
            },
            _ => {},     
        }
    }
    return search;
}

pub fn print_vector(all: Vec<Person>) {
    for person in all.iter() {
        print(Some(person.clone()));
    }
}

pub fn print(person: Option<Person>) {
    let mut per : String = String::new();
    match person {
        None => per.push_str("No person found"),
        Some(person) => {
            per.push_str("Person No. ");
            per.push_str(&person.person_id.to_string());

            per.push_str("\n[1] Vorname: ");
            match &person.vorname {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push_str("\n[2] Zweitname: ");
            match &person.zweitname {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push_str("\n[3] Nachname: ");
            match &person.nachname {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push_str("\n[4] Geburtsname: ");
            match &person.geburtsname {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push_str("\n[5] Gender: ");
            match &person.gender {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push_str("\n[6] Geburtstag: ");
            match &person.geburtstag {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push_str("\n[7] Todestag: ");
            match &person.todestag {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };
        },
    }
    println!("{}", per);
}

pub fn print_vector_names(all: Vec<Person>) {
    for person in all.iter() {
        print_names(Some(person.clone()));
    }
}

pub fn get_person_names(person: Option<Person>) -> String{
    let mut per : String = String::new();
    match person {
        None => per.push_str("No person found"),
        Some(person) => {
            per.push('[');
            per.push_str(&person.person_id.to_string());
            per.push_str("] ");

            per.push_str(&vor_zweit_nach(Some(person.clone())));
            match &person.geburtsname {
                Some(z) => {
                    if !z.is_empty() {  
                        per.push_str(" (");
                        per.push_str(z);
                        per.push(')');
                    }
                },
                _ => { per.push(' ') },
            };
        },
    }
    return per;
}

pub fn print_names(person: Option<Person>){ println!("{}", get_person_names(person)); }

pub fn vor_zweit_nach(person: Option<Person>) -> String{
    let mut per : String = String::new();
    match person {
        None => per.push_str("No person found"),
        Some(person) => {
        
            match &person.vorname {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };
            match &person.zweitname {
                None => {},
                Some(z) => {
                    if !z.is_empty() { 
                        per.push(' ');
                        per.push_str(z);
                    }
                },
            };

            per.push(' ');
            match &person.nachname {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };
        },
    }
    return per;
}