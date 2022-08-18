use crate::Person;
use crate::Relation;

lazy_static! { static ref POOL : mysql::Pool = mysql::Pool::new("mysql://root:Gravure1247@localhost:3306/person").unwrap(); }

#[allow(unused_must_use)]
pub fn get_all_persons() -> Vec<Person>{
    return POOL.prep_exec("SELECT * from person", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) = mysql::from_row(row);

                Person {
                    person_id,
                    first_name,
                    middle_name,
                    surname,
                    maiden_name,
                    gender,
                    birthday,
                    deathday,
                }
            }).collect()
        }).unwrap();
}

#[allow(unused_must_use)]
pub fn insert_person(person: Option<Person>){
    match person {
        None => println!("No person found"),
        Some(person) => {
            let mut insert: String = String::from("INSERT INTO person (first_name, middle_name, surname, maiden_name, gender, birthday, deathday) VALUES (\"");
            
            match person.first_name {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\", \"");
            match person.middle_name {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\", \"");
            match person.surname {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\", \"");
            match person.maiden_name {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\", \"");
            match person.gender {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\", \"");
            match person.birthday {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\", \"");
            match person.deathday {
                None => insert.push_str(""),
                Some(z) => insert.push_str(&z),
            }
            insert.push_str("\");");
            
            match POOL.prep_exec(insert, ()) {
                Ok(_) => {},
                Err(z) => println!("{}", z),
            }
        },
    };
}

#[allow(unused_must_use)]
pub fn update_person(person: Option<Person>){
    match person {
        None => println!("No person found"),
        Some(person) => {
            let mut update: String = String::from("UPDATE person SET first_name = \"");

            match person.first_name {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\", middle_name = \"");
            match person.middle_name {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\", surname = \"");
            match person.surname {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\", maiden_name = \"");
            match person.maiden_name {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\", gender = \"");
            match person.gender {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\", birthday = \"");
            match person.birthday {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\", deathday = \"");
            match person.deathday {
                None => update.push_str(""),
                Some(z) => update.push_str(&z),
            }
            update.push_str("\"WHERE person_id = ");
            update.push_str(&person.person_id.to_string());
            update.push_str(";");
            
            match POOL.prep_exec(update, ()) {
                Ok(_) => {},
                Err(z) => println!("{}", z),
            }
        },
    };
}

#[allow(unused_must_use)]
pub fn single_name_person(name: String) -> Vec<Person>{
    let mut selec: String = String::from("SELECT * from person where first_name like '%");
    selec.push_str(&name);
    selec.push_str("%' or surname = '%;");
    selec.push_str(&name);
    selec.push_str("%';");

    return POOL.prep_exec(selec, ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) = mysql::from_row(row);

                Person {
                    person_id,
                    first_name,
                    middle_name,
                    surname,
                    maiden_name,
                    gender,
                    birthday,
                    deathday,
                }
            }).collect()
        }).unwrap();
}

pub fn double_name_person(a: String, b: String) -> Vec<Person>{
    let mut selec: String = String::from("SELECT * from person where first_name like '%");
    selec.push_str(&a);
    selec.push_str("%' or surname = '%;");
    selec.push_str(&b);
    selec.push_str("%';");

    return POOL.prep_exec(selec, ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) = mysql::from_row(row);

                Person {
                    person_id,
                    first_name,
                    middle_name,
                    surname,
                    maiden_name,
                    gender,
                    birthday,
                    deathday,
                }
            }).collect()
        }).unwrap();
}

#[allow(unused_must_use)]
pub fn id_to_person(id: i32) -> Vec<Person>{
    let mut selec: String = String::from("SELECT * from person where person_id = ");
    selec.push_str(&id.to_string());
    selec.push(';');
    return POOL.prep_exec(selec, ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) = mysql::from_row(row);

                Person {
                    person_id,
                    first_name,
                    middle_name,
                    surname,
                    maiden_name,
                    gender,
                    birthday,
                    deathday,
                }
            }).collect()
        }).unwrap();
}

// Relation

#[allow(unused_must_use)]
pub fn insert_relation(relation: Option<Relation>){
    match relation {
        None => println!("No relation found"),
        Some(relation) => {
            let mut insert: String = String::from("INSERT INTO relation (");
            let mut rows: String = String::new();
            let mut values: String = String::new();
            match relation.male {
                None => {},
                Some(z) => {
                    rows.push_str("male_id,");
                    values.push_str(&z.person_id.to_string());
                    values.push(',');
                },
            }
            match relation.female {
                None => {},
                Some(z) => {
                    rows.push_str("female_id,"); 
                    values.push_str(&z.person_id.to_string());
                    values.push(',');
                },
            }
            match relation.child {
                None => {},
                Some(z) => {
                    rows.push_str("child_id,"); 
                    values.push_str(&z.person_id.to_string());
                    values.push(',');
                },
            }
            if !rows.is_empty(){ 
                rows.pop();
                values.pop();

                insert.push_str(&rows);
                insert.push_str(") values (");
                insert.push_str(&values);
                insert.push_str(");");

                //println!("{}", insert.clone());
                match POOL.prep_exec(insert, ()) {
                    Ok(_) => {},
                    Err(z) => println!("{}", z),
                }
            }
        },
    };
}

pub fn update_relation(original: Option<Relation>, updated: Option<Relation>) {
    match updated {
        None => println!("No relation found"),
        Some(up) => {
            let mut update: String = String::from("UPDATE relation SET ");
            let mut first: String = String::new();
            let mut second: String = String::new();
            match up.male {
                None => {},
                Some(z) => {
                    first.push_str("male_id = ");
                    first.push_str(&z.person_id.to_string())
                },
            }
            match up.female {
                None => {},
                Some(z) => {
                    if !first.is_empty() { first.push_str(", female_id = ") }
                    else { first.push_str("female_id = ") }
                    first.push_str(&z.person_id.to_string())
                },
            }
            match up.child {
                None => {},
                Some(z) => {
                    if !first.is_empty() { first.push_str(", child_id = ") }
                    else { first.push_str("child_id = ") }
                    first.push_str(&z.person_id.to_string())
                },
            }
            first.push_str(" WHERE ");
            match original {
                None => println!("No relation found"),
                Some(ori) => {
                    match ori.male {
                        None => {},
                        Some(z) => {
                            second.push_str("male_id = ");
                            second.push_str(&z.person_id.to_string())
                        },
                    }
                    match ori.female {
                        None => {},
                        Some(z) => {
                            if !second.is_empty() { second.push_str(" and female_id = "); }
                            else { second.push_str("female_id = "); }
                            second.push_str(&z.person_id.to_string())
                        },
                    }
                    match ori.child {
                        None => {},
                        Some(z) => {
                            if !second.is_empty() { second.push_str(" and child_id = "); }
                            else { second.push_str("child_id = "); }
                            second.push_str(&z.person_id.to_string())
                        },
                    }
                    update.push_str(&first);
                    update.push_str(&second);
                    update.push_str(";");
                    println!("{}", update.clone());
                    match POOL.prep_exec(update, ()) {
                        Ok(_) => {println!("DONE")},
                        Err(z) => println!("{}", z),
                    }
                },
            }
        },
    };
}

pub fn person_to_relations(person: Option<Person>, mode: i32) -> Vec<Relation> {
    let mut selec: String = String::from("SELECT * from relation where ");
 
    match person {
        None => {},
        Some(z) => {
            if mode == 0 || mode == 1 {
                selec.push_str(" child_id = ");
                selec.push_str(&z.person_id.to_string());
            }
            if mode == 0 { selec.push_str(" or ") }
            if mode == 0 || mode == 2 {
                selec.push_str("male_id = ");
                selec.push_str(&z.person_id.to_string());
            }
            if mode == 0 { selec.push_str(" or ") }
            if mode == 0 || mode == 3 {
                selec.push_str("female_id = ");
                selec.push_str(&z.person_id.to_string());
            }        
            selec.push_str(";");
        }
    }
    return POOL.prep_exec(selec, ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (male_id, female_id, child_id) = mysql::from_row::<(Option<i32>, Option<i32>, Option<i32>)>(row);

                Relation {
                    male: {
                        match male_id{
                            None => { None },
                            Some(z) => { Some(id_to_person(z)[0].clone()) }
                        }
                    },
                    female: {
                        match female_id{
                            None => { None },
                            Some(z) => { Some(id_to_person(z)[0].clone()) }
                        }
                    },
                    child: {
                        match child_id{
                            None => { None },
                            Some(z) => { Some(id_to_person(z)[0].clone()) }
                        }
                    },
                }
            }).collect()
        }).unwrap();
}
