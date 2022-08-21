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
            first_name: Some(String::new()),
            middle_name: Some(String::new()),
            surname: Some(String::new()),
            maiden_name: Some(String::new()),
            gender: Some(String::new()),
            birthday: Some(String::new()),
            deathday: Some(String::new()),
        };
    }
    pub fn first_name(&mut self, first_name: Option<String>) -> &mut Self {
        self.first_name = first_name;
        self
    }
    pub fn middle_name(&mut self, middle_name: Option<String>) -> &mut Self {
        self.middle_name = middle_name;
        self
    }
    pub fn surname(&mut self, surname: Option<String>) -> &mut Self {
        self.surname = surname;
        self
    }
    pub fn maiden_name(&mut self, maiden_name: Option<String>) -> &mut Self {
        self.maiden_name = maiden_name;
        self
    }
    pub fn gender(&mut self, gender: Option<String>) -> &mut Self {
        self.gender = gender;
        self
    }
    pub fn birthday(&mut self, birthday: Option<String>) -> &mut Self {
        self.birthday = birthday;
        self
    }
    pub fn deathday(&mut self, deathday: Option<String>) -> &mut Self {
        self.deathday = deathday;
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
                    Some(ref mut z) => z.first_name = question(z.first_name.clone(), 1, "First name".to_string()),
                }
            },
            Some('2') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.middle_name = question(z.middle_name.clone(), 2, "Middle name".to_string()),
                }
            },
            Some('3') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.surname = question(z.surname.clone(), 3, "Surname".to_string()),
                }
            },
            Some('4') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.maiden_name = question(z.maiden_name.clone(), 4, "Maiden name".to_string()),
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
                    Some(ref mut z) => z.birthday = question(z.birthday.clone(), 6, "Birthday".to_string()),
                }
            },
            Some('7') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.deathday = question(z.deathday.clone(), 7, "Deathday".to_string()),
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

    // new_person.clone().map(|mut s| { 
    //     s.first_name( question(None, 1, "First name".to_string())); 
    //     s.middle_name( question(None, 2, "Middle name".to_string()));
    //     s.surname( question(None, 3, "Surname".to_string()));
    //     s.maiden_name( question(None, 4, "Maiden name".to_string()));
    //     s.gender( question(None, 5, "Gender".to_string()));
    //     s.birthday( question(None, 6, "Birthday".to_string()));
    //     s.deathday( question(None, 7, "Deathday".to_string()));
    //     s});

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

            per.push_str("\n[1] First name: ");
            match &person.first_name {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push_str("\n[2] Middle name: ");
            match &person.middle_name {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push_str("\n[3] Surname: ");
            match &person.surname {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push_str("\n[4] Maiden name: ");
            match &person.maiden_name {
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

            per.push_str("\n[6] Birthday: ");
            match &person.birthday {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push_str("\n[7] Deathday: ");
            match &person.deathday {
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
            match &person.maiden_name {
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
        
            match &person.first_name {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };
            match &person.middle_name {
                None => {},
                Some(z) => {
                    if !z.is_empty() { 
                        per.push(' ');
                        per.push_str(z);
                    }
                },
            };

            per.push(' ');
            match &person.surname {
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

pub fn get_all_children(person: Person) -> Vec<Person> {
    let mut all_c: Vec<Person> = Vec::new();
    match person.clone().gender {
        None=>{},
        Some(z)=>{
            let all_rela: Vec<Relation>;
            if z == "m"{
                all_rela = crate::db::person_to_relations(Some(person.clone()), 2);
            }else if z == "f"{
                all_rela = crate::db::person_to_relations(Some(person.clone()), 3);
            }else{
                all_rela = Vec::new();
            }
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