use super::person::*;

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

pub fn change(mut p: Option<Relation>) -> (Option<Relation>, bool) {
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
                    Some(ref mut z) => z.male = search(),
                }
            },
            Some('2') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.female = search(),
                }
            },
            Some('3') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.child = search(),
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

fn check(relation: Option<Relation>) -> bool{
    match relation{
        None => return false,
        Some(z) => {
            let mut cnt: i32 = 0;
            if z.male.is_some() { cnt += 1 }
            if z.female.is_some() { cnt += 1 }
            if z.child.is_some() { cnt += 1 }
            if cnt >= 2 { return true }
            else{ return false }
        }
    }
}

pub fn create() -> Option<Relation>{
    let mut new_relation: Option<Relation> = Some(Relation::new());

    let mut boo: bool = true;
    while boo{
        println!("\n\nNewly created Relation:");
        print(new_relation.clone());
        (new_relation, boo) = change(new_relation.clone());
        if !boo && !check(new_relation.clone()) {
            println!("!! You need at least 2 persons for a relation");
            boo = true;
        }
    }
    return new_relation;
}

pub fn print(relation: Option<Relation>) {
    let mut per : String = String::new();
    match relation {
        None => per.push_str("No relation found"),
        Some(relation) => {
            
            per.push_str("[1] Male: ");
            match &relation.male {
                None => per.push_str("--"),
                Some(_) => per.push_str(&get_person_names(relation.male)),
            };

            per.push_str("\n[2] Female: ");
            match &relation.female {
                None => per.push_str("--"),
                Some(_) => per.push_str(&get_person_names(relation.female)),
            };

            per.push_str("\n[3] Child: ");
            match &relation.child {
                None => per.push_str("--"),
                Some(_) => per.push_str(&get_person_names(relation.child)),
            };
        },
    }
    println!("{}", per);
}