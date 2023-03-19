use crate::Person;
use crate::Family;
use mysql::chrono::NaiveDate;

lazy_static! { 
    static ref POOL : mysql::Pool = mysql::Pool::new(
        format!("mysql://{}:{}@{}:{}/{}", 
        std::env::var("DB_USER").expect("DB_IP must be set."),
        std::env::var("DB_PASSWORD").expect("DB_IP must be set."),
        std::env::var("DB_IP").expect("DB_IP must be set."),
        std::env::var("DB_PORT").expect("DB_IP must be set."),
        std::env::var("DB_TABLE").expect("DB_IP must be set.")
    )
    ).unwrap(); 
}

pub fn reset_db() {
// Drop old Tables
    match POOL.prep_exec(
        "drop table children, person, family;"
        , ()) {
        Ok(_) => {},
        Err(z) => println!("{}", z),
    } 
// Create Person
    match POOL.prep_exec(
        "create table person(person_id int auto_increment, first_name varchar(255), middle_name varchar(255), surname varchar(255), maiden_name varchar(255), gender varchar(1), birthday date, deathday date, constraint person_pk primary key (person_id));"
        , ()) {
        Ok(_) => {},
        Err(z) => println!("{}", z),
    } 
// Create Family
    match POOL.prep_exec(
        "create table family (family_id int auto_increment, male_id int, female_id int, constraint family_pk primary key (family_id));"
        , ()) {
        Ok(_) => {},
        Err(z) => println!("{}", z),
    }
// Create Children
    match POOL.prep_exec(
        "create table children (person_id int NOT NULL, family_id int NOT NULL, constraint person_family_pk primary key (person_id, family_id));"
        , ()) {
        Ok(_) => {},
        Err(z) => println!("{}", z),
    } 
}

pub fn get_all_persons() -> Vec<Person>{
    let mut ret: Vec<Person> = Vec::new();
    match POOL.prep_exec("SELECT * from person", ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) 
                : (i32, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<NaiveDate>, Option<NaiveDate>)
                = mysql::from_row(row);
                Person {
                    person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday
                }
            }).collect()
        },
        Err(z) => println!("{}", z),
    } 
    ret
}

pub fn insert_person(person: Person) -> u64 {
    let mut select: String = String::from("SELECT person_id from person where ");
    let mut insert: String = String::from("INSERT INTO person (first_name, middle_name, surname, maiden_name, gender, birthday, deathday) VALUES (");
    match person.first_name {
        None => insert.push_str("\"\", "),
        Some(z) => {
            insert.push_str(&format!("\"{}\", ", &z));
            select.push_str(&format!("first_name = \"{}\"", &z));
        },
    }
    match person.middle_name {
        None => insert.push_str("\"\", "),
        Some(z) => {
            insert.push_str(&format!("\"{}\", ", &z));
            select.push_str(&format!(" and middle_name = \"{}\"", &z));
        },
    }
    match person.surname {
        None => insert.push_str("\"\", "),
        Some(z) => {
            insert.push_str(&format!("\"{}\", ", &z));
            select.push_str(&format!(" and surname = \"{}\"", &z));
        },
    }
    match person.maiden_name {
        None => insert.push_str("\"\", "),
        Some(z) => {
            insert.push_str(&format!("\"{}\", ", &z));
            select.push_str(&format!(" and maiden_name = \"{}\"", &z));
        },
    }
    match person.gender {
        None => insert.push_str("\"\", "),
        Some(z) => {
            insert.push_str(&format!("\"{}\", ", &z));
            select.push_str(&format!(" and gender = \"{}\"", &z));
        },
    }
    match person.birthday {
        None => insert.push_str("NULL, "),
        Some(z) => {
            insert.push_str(&format!("\"{}\", ", &z.to_string()));
            select.push_str(&format!(" and birthday = \"{}\"", &z.to_string()));
        },
    }
    match person.deathday {
        None => insert.push_str("NULL"),
        Some(z) => {
            insert.push_str(&format!("\"{}\", ", &z.to_string()));
            select.push_str(&format!(" and deathday = \"{}\"", &z.to_string()));
        },
    }
    
    insert.push_str(");");
    select.push_str(";");

    let person_ids: Vec<i32> = match POOL.prep_exec(select, ()) {
        Ok(qr) => {
            qr.map(|x| x.unwrap()).map(|row| {
                mysql::from_row(row)
            }).collect()
        },
        Err(z) => { println!("{}", z); Vec::new() },
    };
    if person_ids.len() > 0 {
        return person_ids[0] as u64;
    }else {
        match POOL.prep_exec(insert, ()) {
            Ok(qr) => {
                return qr.last_insert_id();
            },
            Err(z) => { println!("{}", z); 0 },
        }  
    }         
}

pub fn update_person(person: Person){
    let mut update: String = String::from("UPDATE person SET first_name = \"");

    match person.first_name {
        None => {},
        Some(z) => update.push_str(&z),
    }
    update.push_str("\", middle_name = \"");
    match person.middle_name {
        None => {},
        Some(z) => update.push_str(&z),
    }
    update.push_str("\", surname = \"");
    match person.surname {
        None => {},
        Some(z) => update.push_str(&z),
    }
    update.push_str("\", maiden_name = \"");
    match person.maiden_name {
        None => {},
        Some(z) => update.push_str(&z),
    }
    update.push_str("\", gender = \"");
    match person.gender {
        None => {},
        Some(z) => update.push_str(&z),
    }
    update.push_str("\"");
    match person.birthday {
        None => update.push_str(", birthday = NULL"),
        Some(z) => update.push_str(&format!(", birthday = \"{}\"", &z.to_string())),
    }
    match person.deathday {
        None => update.push_str(", deathday = NULL"),
        Some(z) => update.push_str(&format!(", deathday = \"{}\"", &z.to_string())),
    }
    update.push_str(&format!(" WHERE person_id = {};", person.person_id));

    match POOL.prep_exec(update, ()) {
        Ok(_) => {},
        Err(z) => println!("{}", z),
    }
}

pub fn get_person_by_single_name(name: String) -> Vec<Person>{
    let mut selec: String = String::from("SELECT * from person where first_name like '%");
    selec.push_str(&name);
    selec.push_str("%' or surname like '%");
    selec.push_str(&name);
    selec.push_str("%';");

    let mut ret: Vec<Person> = Vec::new();
    match POOL.prep_exec(selec, ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                    let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) = mysql::from_row(row);
                    Person {
                        person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday,
                    }
                }).collect()
        },
        Err(z) => println!("{}", z),
    } 
    ret
}

pub fn get_person_by_double_name(a: String, b: String) -> Vec<Person>{
    let mut selec: String = String::from("SELECT * from person where first_name like '%");
    selec.push_str(&a);
    selec.push_str("%' or surname like '%");
    selec.push_str(&b);
    selec.push_str("%';");

    let mut ret: Vec<Person> = Vec::new();
    match POOL.prep_exec(selec, ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                    let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) = mysql::from_row(row);
                    Person {
                        person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday,
                    }
                }).collect()
        },
        Err(z) => println!("{}", z),
    } 
    ret
}

pub fn get_person_by_id(id: i32) -> Option<Person>{
    let mut selec: String = String::from("SELECT * from person where person_id = ");
    selec.push_str(&id.to_string());
    selec.push(';');
    let mut ret: Vec<Person> = Vec::new();
    match POOL.prep_exec(selec, ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                    let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) = mysql::from_row(row);
                    Person {
                        person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday,
                    }
                }).collect()
        },
        Err(z) => println!("{}", z),
    } 
    if ret.len() == 1 {
        Some(ret[0].clone())
    }else {
        None
    }
}

pub fn get_children_by_family_id(family_id: i32) -> Vec<Person>{
    let mut ret: Vec<Person> = Vec::new();
    match POOL.prep_exec(format!("select p.* from person p join children c on p.person_id = c.person_id where c.family_id = {};", family_id), ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) 
                : (i32, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<NaiveDate>, Option<NaiveDate>)
                = mysql::from_row(row);
                Person {
                    person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday
                }
            }).collect()
        },
        Err(z) => println!("{}", z),
    } 
    ret
}

pub fn get_all_children(male_id: i32, female_id: i32, child_id: i32) -> Vec<Person> {
    let mut ret: Vec<Person> = Vec::new();
    match POOL.prep_exec(format!("select p.* from family f join children c on f.family_id = c.family_id join person p on p.person_id = c.person_id where f.male_id = {} and f.female_id = {} ORDER BY CASE WHEN p.person_id = {} THEN 0 ELSE 1 END, p.person_id DESC;", male_id, female_id, child_id), ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) 
                : (i32, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<NaiveDate>, Option<NaiveDate>)
                = mysql::from_row(row);
                Person {
                    person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday
                }
            }).collect()
        },
        Err(z) => println!("{}", z),
    };
    ret 
}

pub fn insert_children(children: Vec<Person>, family_id: u64) {
    let mut insert: Vec<i32> = Vec::new();
    let mut values: String = String::new();

    for child in children.clone() {
        match POOL.prep_exec(format!("Select person_id from children where person_id = {} and family_id = {};", child.person_id, family_id), ()) {
            Ok(qr) => {
                _ = qr.map(|x| x.unwrap()).map(|row| {
                    insert.push(mysql::from_row(row));
                });
            },
            Err(z) => println!("{}", z),
        }
    }
    if insert.len() != children.len() {
        for item in children {
            if !insert.contains(&item.person_id) {
                values.push_str(&format!("({}, {}),", item.person_id, family_id));
            }
        }
        values.pop();
        match POOL.prep_exec(format!("INSERT INTO children (person_id, family_id) values {};", values), ()) {
            Ok(_) => {},
            Err(z) => println!("{}", z),
        }   
    }
}

pub fn get_person_by_child_id(child_id: i32) -> Vec<Person> {
    let mut ret: Vec<Person> = Vec::new();
    match POOL.prep_exec(format!("
        select * from person where person_id = any(
        select male_id from family f 
        join children c on f.family_id = c.family_id 
        join person p on p.person_id = c.person_id 
        where c.person_id = {}) or person_id = any(
        select female_id from family f 
        join children c on f.family_id = c.family_id 
        join person p on p.person_id = c.person_id 
        where c.person_id = {}
        );", child_id, child_id), ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) 
                : (i32, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<NaiveDate>, Option<NaiveDate>)
                = mysql::from_row(row);
                Person {
                    person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday
                }
            }).collect()
        },
        Err(z) => println!("{}", z),
    } 
    ret
}

pub fn get_person_by_variable_name(full_name: String) -> Option<Person> {
    let mut select: String = String::new();

    //println!("!! Name: |{}|", full_name);
    let name_vec: Vec<&str> = full_name.split(" ").collect();
    if name_vec.len() == 0 || full_name == "" {
        // one name
        return None;
    }else if name_vec.len() == 1 {
        // one name
        //println!("Warning: person has only one name: |{}|", name_vec[0]);
        select.push_str(&format!("first_name = \"{}\"", name_vec[0]))
    }else if name_vec.len() == 2 {
        // 2 names
        select.push_str(&format!("first_name = \"{}\" and surname = \"{}\"", name_vec[0], name_vec[1]))
    }else if name_vec.len() > 2 {
        // multiple names
        select.push_str(&format!("first_name = \"{}\" and surname = \"{}\" and middle_name = \"", name_vec[0], name_vec[name_vec.len()-1]));

        for i in 1..name_vec.len() - 1 {
            //println!("I {}", i);
            select.push_str(&format!("{} ", name_vec[i]));
        }
        select.pop();
        select.push_str("\"");
    }
    // println!("{}", select);
    let mut ret: Vec<Person> = Vec::new();
    match POOL.prep_exec(format!("SELECT * from person where {};", select), ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) = mysql::from_row(row);
                Person {
                    person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday,
                }
            }).collect()
        },
        Err(z) => println!("{}", z),
    } 
    if ret.len() == 1 {
        Some(ret[0].clone())
    }else {
        None
    }
}

pub fn get_siblings_by_person_id(person_id: i32) -> Vec<Person> {
    let mut ret: Vec<Person> = Vec::new();
    match POOL.prep_exec(format!("select p.* from person p join children c on p.person_id = c.person_id join family f on c.family_id = f.family_id where f.family_id = (select family_id from children where person_id = {});", person_id), ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                let (person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday) 
                : (i32, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<NaiveDate>, Option<NaiveDate>)
                = mysql::from_row(row);
                Person {
                    person_id, first_name, middle_name, surname, maiden_name, gender, birthday, deathday
                }
            }).collect()
        },
        Err(z) => println!("{}", z),
    } 
    ret
}

//// #############

pub fn insert_family(family: Family){
    let male: String;
    let female: String;
    match family.male {
        None => { male = String::from("NULL") },
        Some(z) => { male = z.person_id.to_string() },
    }
    match family.female {
        None => { female = String::from("NULL") },
        Some(z) => { female = z.person_id.to_string() },
    }

    let mut family_id : u64 = 0;
    match POOL.prep_exec(format!("select family_id from family where male_id = {} and female_id = {} LIMIT 1;", male, female), ()) {
        Ok(qr) => {
            let ret : Vec<u64> = qr.map(|x| x.unwrap()).map(|row| {
                mysql::from_row(row)
            }).collect();
            if ret.len() > 0 {
                family_id = ret[0];
            }
        },
        Err(z) => println!("{}", z),
    }
    if family_id == 0 {
        match POOL.prep_exec(format!("INSERT INTO family (male_id, female_id) values ({}, {});", male, female), ()) {
            Ok(qr) => {
                family_id = qr.last_insert_id();
            },
            Err(z) => println!("{}", z),
        }
    }

    if family.children.len() > 0 {
        insert_children(family.children, family_id);
    }
}

pub fn update_family(original: Family, updated: Family) {
    let mut update: String = String::from("UPDATE family SET ");
    match updated.male {
        None => {
            update.push_str("male_id = NULL");
        },
        Some(z) => {
            update.push_str(&format!("male_id = {}", &z.person_id.to_string()));
        },
    }
    match updated.female {
        None => {
            update.push_str(", female_id = NULL")
        },
        Some(z) => {
            update.push_str(&format!(", female_id = {}", &z.person_id.to_string()));
        },
    }
    
    match POOL.prep_exec(format!("{} where family_id = {};", update, original.family_id.to_string()), ()) {
        Ok(_) => {},
        Err(z) => println!("{}", z),
    }

    let children_db: Vec<Person>= get_children_by_family_id(original.family_id);

    for child_db in children_db.clone() { // old children list
        if !updated.children.contains(&child_db) {
            // remove child_db
            match POOL.prep_exec(format!("delete from children where id = {};", child_db.person_id), ()) {
                Ok(_) => {},
                Err(z) => println!("{}", z),
            } 
        }
    }
    for child in updated.children.clone() {
        if children_db.contains(&child) {
            // new child adden 
            match POOL.prep_exec(format!("INSERT INTO children (person_id, family_id) values ({}, {});", child.person_id, original.family_id), ()) {
                Ok(_) => {},
                Err(z) => println!("{}", z),
            } 
        }
    }
}

pub fn get_family_by_person_id(person_id: i32) -> Vec<Family>{
    let mut ret: Vec<Family> = Vec::new();
    match POOL.prep_exec(format!("select f.* from family f join children c on f.family_id = c.family_id where male_id = {person_id} or female_id = {person_id} or c.person_id = {person_id};", person_id = person_id), ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                let (family_id, male_id, female_id) 
                : (i32, i32, i32)
                = mysql::from_row(row);
                build_a_family(family_id, male_id, female_id, person_id)
            }).collect()
        },
        Err(z) => println!("{}", z),
    } 
    ret
}

fn build_a_family(family_id: i32, male_id: i32, female_id: i32, child_id: i32) -> Family {
    let male: Option<Person> = get_person_by_id(male_id);
    let female: Option<Person> = get_person_by_id(female_id);
    let children: Vec<Person> = get_all_children(male_id, female_id, child_id);
    return Family { family_id: family_id, male: male, female: female, children: children };
}

pub fn get_family_by_child_id(child_id: i32) -> Vec<Family> {
    let mut ret: Vec<Family> = Vec::new();
    match POOL.prep_exec(format!("select f.* from family f join children c on f.family_id = c.family_id where c.person_id = {};", child_id), ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                let (family_id, male_id, female_id) 
                : (i32, i32, i32)
                = mysql::from_row(row);
                build_a_family(family_id, male_id, female_id, child_id)
            }).collect()
        },
        Err(z) => println!("{}", z),
    } 
    ret
}

pub fn get_family_by_parent_id(parent_id: i32) -> Vec<Family> {
    let mut ret: Vec<Family> = Vec::new();
    match POOL.prep_exec(format!("select * from family where male_id = {} or female_id = {};", parent_id, parent_id), ()) {
        Ok(z) => {
            ret = z.map(|x| x.unwrap()).map(|row| {
                let (family_id, male_id, female_id) 
                : (i32, i32, i32)
                = mysql::from_row(row);
                build_a_family(family_id, male_id, female_id, parent_id)
            }).collect()
        },
        Err(z) => println!("{}", z),
    } 
    ret
}