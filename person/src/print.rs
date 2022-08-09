use crate::Person;

pub fn print_vector_person(all: Vec<Person>) {
    for person in all.iter() {
        print_person(Some(person.clone()));
    }
}

pub fn print_person(person: Option<Person>) {
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

            per.push_str("\n[5] Geburtstag: ");
            match &person.geburtstag {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push_str("\n[6] Todestag: ");
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

pub fn print_vector_person_names(all: Vec<Person>) {
    for person in all.iter() {
        print_person_names(Some(person.clone()));
    }
}

pub fn print_person_names(person: Option<Person>){
    let mut per : String = String::new();
    match person {
        None => per.push_str("No person found"),
        Some(person) => {
            per.push('[');
            per.push_str(&person.person_id.to_string());
            per.push_str("] ");

            match &person.vorname {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
                },
            };

            per.push(' ');
            match &person.zweitname {
                None => per.push_str("--"),
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else { per.push_str(z) }
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
            match &person.geburtsname {
                Some(z) => {
                    if z.is_empty() { per.push_str("--"); }
                    else{  
                        per.push_str(" (");
                        per.push_str(z);
                        per.push(')');
                    }
                },
                _ => { per.push(' ') },
            };
        },
    }
    println!("{}", per);
}