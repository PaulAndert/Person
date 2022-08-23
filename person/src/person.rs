use crate::Relation;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Person {
    pub person_id: i32,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub surname: Option<String>,
    pub maiden_name: Option<String>, 
    pub gender: Option<String>, 
    pub birthday: Option<String>,
    pub deathday: Option<String>,
}

impl Person {
    pub fn new () -> Person {
        return Person{
            person_id: -1,
            first_name: None,
            middle_name: None,
            surname: None,
            maiden_name: None,
            gender: None,
            birthday: None,
            deathday: None,
        };
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

pub fn change(mut person: Person) -> (Person, bool) {
    let mut line: String = String::new();
    println!("for changes type the number you want to change, else press enter");
    let n = std::io::stdin().read_line(&mut line).unwrap();
    if n <= 1 {
        return (person, false);
    } else if n >= 3 {
        line.pop();
        println!("{} is not a valid number", line);
        return (person, true);
    }else{
        match line.chars().next() {
            None => return (person, true),
            Some('1') => { person.first_name = question(person.first_name.clone(), 1, "First name".to_string()) },
            Some('2') => { person.middle_name = question(person.middle_name.clone(), 2, "Middle name".to_string()) },
            Some('3') => { person.surname = question(person.surname.clone(), 3, "Surname".to_string()) },
            Some('4') => { person.maiden_name = question(person.maiden_name.clone(), 4, "Maiden name".to_string()) },
            Some('5') => { person.gender = question(person.gender.clone(), 5, "Gender".to_string()) },
            Some('6') => { person.birthday = question(person.birthday.clone(), 6, "Birthday".to_string()) },
            Some('7') => { person.deathday = question(person.deathday.clone(), 7, "Deathday".to_string()) },
            Some(_) => {
                line.pop();
                println!("{} is not a valid number", line);
            },
        }
        return (person, true);
    }
}

pub fn create() -> Person{
    let person: Person = Person::new();
    loop {
        println!("\n\nNewly created Person:");
        print(person.clone());
        let (person, boo) = change(person.clone());
        if !boo { return person }
    }
}

pub fn search() -> Person {
    let mut results: Vec<Person>;
    loop {
        let mut line: String = String::new();
        println!("Who are you searching? Please enter the First Name and/or Last Name or their Id");
        let _ = std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        match line.chars().next() {
            Some(z) if z.is_numeric() => { // One can only esacpe this loop by typing a Number/Id 
                let(a, _) = crate::mixed_to_single(line.clone(), 1);
                let id: i32 = a.parse::<i32>().unwrap();
                if id > 0 {
                    match crate::db::id_to_person(id) {
                        None => { println!("there is nobody with that id, please try again"); },
                        Some(z) => {
                            print_all_4_names(z.clone());
                            line.clear();
                            println!("Is that the correct person? Press enter if true or any key if false");
                            let _ = std::io::stdin().read_line(&mut line).unwrap();
                            line.pop();
                            match line.chars().next() {
                                None => { return z; }, // if enter is pressed 
                                Some(_) => {},
                            }
                        },
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
                else{ print_vector_only_names(results); }
            },
            _ => { /* because boo is still true the loop is still going and there is no need for error handling here */ },     
        }
    }
}

pub fn print_vector(all: Vec<Person>) {
    for i in 0..all.len() {
        print(all[i].clone());
    }
}

pub fn print(person: Person) {
    let mut per: String = String::new();
    per.push_str("Person No. ");
    per.push_str(&person.person_id.to_string());

    per.push_str("\n[1] First name: ");
    match person.first_name {
        None => per.push_str("--"),
        Some(z) => {
            if z.is_empty() { per.push_str("--"); }
            else { per.push_str(&z) }
        },
    };

    per.push_str("\n[2] Middle name: ");
    match person.middle_name {
        None => per.push_str("--"),
        Some(z) => {
            if z.is_empty() { per.push_str("--"); }
            else { per.push_str(&z) }
        },
    };

    per.push_str("\n[3] Surname: ");
    match person.surname {
        None => per.push_str("--"),
        Some(z) => {
            if z.is_empty() { per.push_str("--"); }
            else { per.push_str(&z) }
        },
    };

    per.push_str("\n[4] Maiden name: ");
    match person.maiden_name {
        None => per.push_str("--"),
        Some(z) => {
            if z.is_empty() { per.push_str("--"); }
            else { per.push_str(&z) }
        },
    };

    per.push_str("\n[5] Gender: ");
    match person.gender {
        None => per.push_str("--"),
        Some(z) => {
            if z.is_empty() { per.push_str("--"); }
            else { per.push_str(&z) }
        },
    };

    per.push_str("\n[6] Birthday: ");
    match person.birthday {
        None => per.push_str("--"),
        Some(z) => {
            if z.is_empty() { per.push_str("--"); }
            else { per.push_str(&z) }
        },
    };

    per.push_str("\n[7] Deathday: ");
    match person.deathday {
        None => per.push_str("--"),
        Some(z) => {
            if z.is_empty() { per.push_str("--"); }
            else { per.push_str(&z) }
        },
    };
    println!("{}", per);
}

pub fn print_vector_only_names(all: Vec<Person>) {
    for person in all.iter() {
        print_all_4_names(person.clone());
    }
}

pub fn print_all_4_names(person: Person){ 
    println!("{}", get_all_4_names(person)); 
}

pub fn get_all_4_names(person: Person) -> String{
    let mut per : String = String::new();
    per.push('[');
    per.push_str(&person.person_id.to_string());
    per.push_str("] ");

    per.push_str(&get_3_names(person.clone()));
    match person.maiden_name {
        Some(z) => {
            if !z.is_empty() {  
                per.push_str(" (");
                per.push_str(&z);
                per.push(')');
            }
        },
        _ => { per.push(' ') },
    };
    per
}

pub fn get_3_names(person: Person) -> String{
    let mut per: String = String::new();
    match person.first_name {
        None => per.push_str("--"),
        Some(z) => {
            if z.is_empty() { per.push_str("--"); }
            else { per.push_str(&z) }
        },
    };
    match person.middle_name {
        None => {},
        Some(z) => {
            if !z.is_empty() { 
                per.push(' ');
                per.push_str(&z);
            }
        },
    };
    per.push(' ');
    match person.surname {
        None => per.push_str("--"),
        Some(z) => {
            if z.is_empty() { per.push_str("--"); }
            else { per.push_str(&z) }
        },
    };
    per
}

pub fn get_all_children(person: Person) -> Vec<Person> {
    let mut all_c: Vec<Person> = Vec::new();
    match person.clone().gender {
        None=>{},
        Some(z)=>{
            let all_rela: Vec<Relation>;
            if z == "m"{ all_rela = crate::db::person_id_to_relations(person.person_id, 2) }
            else if z == "f"{ all_rela = crate::db::person_id_to_relations(person.person_id, 3) }
            else if z == "um"{ all_rela = crate::db::person_id_to_relations(person.person_id, 3) }
            else if z == "uf"{ all_rela = crate::db::person_id_to_relations(person.person_id, 2) }
            else{ all_rela = Vec::new() }

            for i in 0..all_rela.len() {
                match all_rela[i].clone().child {
                    None => {},
                    Some(z) => {
                        if !all_c.contains(&z) {
                            all_c.push(z);
                        }
                    }
                }
            }
        },
    }
    all_c
}