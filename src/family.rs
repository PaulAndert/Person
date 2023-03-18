use crate::Person;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Family {
    pub family_id: i32,
    pub male: Option<Person>,
    pub female: Option<Person>,
    pub children: Vec<Person>,
} 

impl Family {
    pub fn new () -> Family {
        return Family{
            family_id: -1,
            male: None,
            female: None,
            children: Vec::new(),
        };
    }
    pub fn to_string(&self) -> String {
        let mut per: String = String::new();
        per.push_str(&format!("Family No. {}\n", self.family_id.to_string()));
        match self.male.clone() {
            None => {per.push_str("[1] Male: --\n")},
            Some(male) => {per.push_str(&format!("[1] Male: {}\n", male.four_names()))},
        }
        match self.female.clone() {
            None => {per.push_str("[2] Female: --\n")},
            Some(female) => {per.push_str(&format!("[2] Female: {}\n", female.four_names()))},
        }
        if self.children.len() > 0 {
            per.push_str("[3] Children:");
            for child in self.children.clone() {
                per.push_str(&format!(" {},", child.four_names()));
            }
            per.pop();
        }else {
            per.push_str("[3] Children: --\n");
        }
        per
    }
}

pub fn change(mut family: Family) -> (Family, bool) {
    let mut line: String = String::new();
    println!("for changes type the number you want to change, else press enter");
    let n = std::io::stdin().read_line(&mut line).unwrap();
    if n <= 1 {
        return (family, false);
    } else if n >= 3 {
        line.pop();
        println!("{} is not a valid number", line);
        return (family, true);
    }else{
        match line.chars().next() {
            None => return (family, true),
            Some('1') => { 
                let person = crate::person::search();
                if person.person_id == -1 { family.male = None }
                else { family.male = Some(person) }
            },
            Some('2') => { 
                let person = crate::person::search();
                if person.person_id == -1 { family.female = None }
                else { family.female = Some(person) }
            },
            Some('3') => { 
                let person = crate::person::search();
                if person.person_id != -1 { family.children.push(person) }
            },
            Some(_) => {
                line.pop();
                println!("{} is not a valid number", line);
            },
        }
        return (family, true);
    }
}

fn check(family: Family) -> bool{
    let mut cnt: i32 = 0;
    if family.male.is_some() {
        cnt += 1;
    }
    if family.female.is_some() {
        cnt += 1;
    }
    if family.children.len() > 0 { 
        cnt += family.children.len() as i32;
    }
    if cnt >= 2 { 
        true 
    }else{ 
        false 
    }
}

pub fn create() -> Family {
    let mut new_family: Family = Family::new();
    loop {
        println!("\n\nNewly created Family:");
        println!("{}", new_family.to_string());
        let boo: bool;
        (new_family, boo) = change(new_family);
        if !boo {
            if !check(new_family.clone()) { println!("!! You need at least 2 persons for a Family") }
            else { return new_family }
        }
    }
}

pub fn search() -> Family {
    let mut results: Vec<Family>;
    loop {
        println!("Please enter a person who is in that family (until there is only one family left)");
        let person = crate::person::search();
        if person.person_id != -1 {
            results = crate::db::get_family_by_person_id(person.person_id);
            if results.clone().len() == 1 { return results[0].clone() }
            else{ 
                for res in results.clone() { println!("{}", res.to_string()) };
                println!("if the correct relation is among them please enter the number to select");
                let mut line: String = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let (number , _) = crate::mixed_to_single(line.clone(), 1);
                let index = number.parse::<i32>().unwrap();
                if index > 0 && results.clone().into_iter().any(|res| res.family_id == index ) {
                    match results.into_iter().find(|res| res.family_id == index) {
                        None => {
                            line.pop();
                            println!("{} is not a valid index", line);
                        },
                        Some(ret) => {
                            return ret;
                        },
                    }
                }else {
                    line.pop();
                    println!("{} is not a valid index", line);
                }
            }
        }
    }
}