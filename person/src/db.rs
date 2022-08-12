use super::person::Person;
//use super::parent::Parent;

lazy_static! { static ref POOL : mysql::Pool = mysql::Pool::new("mysql://root:Gravure1247@localhost:3306/person").unwrap(); }

#[allow(unused_must_use)]
pub fn get_all_persons() -> Vec<Person>{
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
        }).unwrap();
}

#[allow(unused_must_use)]
pub fn insert_person(person: Option<Person>){
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

#[allow(unused_must_use)]
pub fn update_person(person: Option<Person>){
    match person {
        None => println!("No person found"),
        Some(person) => {
            let mut update: String = String::from("UPDATE person SET vorname = \"");

            match person.vorname {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\", zweitname = \"");
            match person.zweitname {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\", nachname = \"");
            match person.nachname {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\", geburtsname = \"");
            match person.geburtsname {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\", geburtstag = \"");
            match person.geburtstag {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\", todestag = \"");
            match person.todestag {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\"WHERE person_id = ");
            update.push_str(&person.person_id.to_string());
            update.push_str(";");
            
            POOL.prep_exec(update, ());
        },
    };
}

#[allow(unused_must_use)]
pub fn single_name_person(name: String) -> Vec<Person>{
    let mut selec: String = String::from("SELECT * from person where vorname like '%");
    selec.push_str(&name);
    selec.push_str("%' or nachname = '%;");
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

pub fn double_name_person(a: String, b: String) -> Vec<Person>{
    let mut selec: String = String::from("SELECT * from person where vorname like '%");
    selec.push_str(&a);
    selec.push_str("%' or nachname = '%;");
    selec.push_str(&b);
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

#[allow(unused_must_use)]
pub fn id_person(id: i32) -> Vec<Person>{
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