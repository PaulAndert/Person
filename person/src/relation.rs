use crate::Person;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Relation {
    pub male: Option<Person>,
    pub female: Option<Person>,
    pub child: Option<Person>,
} 

impl Relation {
    fn new () -> Relation {
        return Relation{
            male: None,
            female: None,
            child: None,
        };
    }
}

pub fn change(mut relation: Relation) -> (Relation, bool) {
    let mut line: String = String::new();
    println!("for changes type the number you want to change, else press enter");
    let n = std::io::stdin().read_line(&mut line).unwrap();
    if n <= 1 {
        return (relation, false);
    } else if n >= 3 {
        line.pop();
        println!("{} is not a valid number", line);
        return (relation, true);
    }else{
        match line.chars().next() {
            None => return (relation, true),
            Some('1') => { relation.male = Some(crate::person::search()) },
            Some('2') => { relation.female = Some(crate::person::search()) },
            Some('3') => { relation.child = Some(crate::person::search()) },
            Some(_) => {
                line.pop();
                println!("{} is not a valid number", line);
            },
        }
        return (relation, true);
    }
}

fn check(relation: Relation) -> bool{
    let mut cnt: i32 = 0;
    if relation.male.is_some() { cnt += 1 }
    if relation.female.is_some() { cnt += 1 }
    if relation.child.is_some() { cnt += 1 }
    if cnt >= 2 { return true }
    else{ return false }
}

pub fn create() -> Relation{
    let mut new_relation: Relation = Relation::new();
    loop {
        println!("\n\nNewly created Relation:");
        print(new_relation.clone());
        let boo: bool;
        (new_relation, boo) = change(new_relation);
        if !boo {
            if !check(new_relation.clone()) { println!("!! You need at least 2 persons for a relation") }
            else { return new_relation }
        }
    }
}

pub fn print(relation: Relation) {
    let mut per : String = String::new();
    per.push_str("[1] Male: ");
    match relation.male {
        None => per.push_str("--"),
        Some(z) => per.push_str(&crate::person::get_all_4_names(z)),
    };

    per.push_str("\n[2] Female: ");
    match relation.female {
        None => per.push_str("--"),
        Some(z) => per.push_str(&crate::person::get_all_4_names(z)),
    };

    per.push_str("\n[3] Child: ");
    match relation.child {
        None => per.push_str("--"),
        Some(z) => per.push_str(&crate::person::get_all_4_names(z)),
    };
    println!("{}", per);
}

pub fn search() -> Relation {
    let mut results: Vec<Relation>;
    loop {
        println!("Please enter a person who is in that relation (until there is only one relation left)");
        results = crate::db::person_id_to_relations(crate::person::search().person_id, 0);
        if results.clone().len() == 1 { return results[0].clone() }
        else{ crate::relation::print_vector(results) }
    }
}

pub fn print_vector(all: Vec<Relation>) {
    let mut cnt: i32 = 0;
    for relation in all.iter() {
        cnt += 1;
        println!("Relation no. {}", cnt);
        print(relation.clone());
    }
}