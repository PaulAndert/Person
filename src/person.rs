// use crate::Relation;
// use std::collections::HashMap;
use mysql::chrono::NaiveDate;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Person {
    pub person_id: i32,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub surname: Option<String>,
    pub maiden_name: Option<String>, 
    pub gender: Option<String>, 
    pub birthday: Option<NaiveDate>,
    pub deathday: Option<NaiveDate>,
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
    pub fn to_string(&self) -> String {
        let mut per: String = String::new();
        per.push_str("Person No. ");
        per.push_str(&self.person_id.to_string());

        per.push_str("\n[1] First name: ");
        match self.first_name.clone() {
            None => per.push_str("--"),
            Some(z) => {
                if z.is_empty() { per.push_str("--"); }
                else { per.push_str(&z) }
            },
        };

        per.push_str("\n[2] Middle name: ");
        match self.middle_name.clone() {
            None => per.push_str("--"),
            Some(z) => {
                if z.is_empty() { per.push_str("--"); }
                else { per.push_str(&z) }
            },
        };

        per.push_str("\n[3] Surname: ");
        match self.surname.clone() {
            None => per.push_str("--"),
            Some(z) => {
                if z.is_empty() { per.push_str("--"); }
                else { per.push_str(&z) }
            },
        };

        per.push_str("\n[4] Maiden name: ");
        match self.maiden_name.clone() {
            None => per.push_str("--"),
            Some(z) => {
                if z.is_empty() { per.push_str("--"); }
                else { per.push_str(&z) }
            },
        };

        per.push_str("\n[5] Gender: ");
        match self.gender.clone() {
            None => per.push_str("--"),
            Some(z) => {
                if z.is_empty() { per.push_str("--"); }
                else { per.push_str(&z) }
            },
        };

        per.push_str("\n[6] Birthday: ");
        match self.birthday {
            None => per.push_str("--"),
            Some(z) => {
                per.push_str(&z.format("%d.%m.%Y").to_string())
            },
        };

        per.push_str("\n[7] Deathday: ");
        match self.deathday {
            None => per.push_str("--"),
            Some(z) => {
                per.push_str(&z.format("%d.%m.%Y").to_string())
            },
        };
        per 
    }

    pub fn two_names(&self) -> String {
        let mut per: String = String::new();

        match self.first_name.clone() {
            None => per.push_str("-- "),
            Some(z) => {
                if z.is_empty() { per.push_str("-- "); }
                else { per.push_str(&z) }
            },
        };
        match self.surname.clone() {
            None => per.push_str(" --"),
            Some(z) => {
                if z.is_empty() { per.push_str(" --"); }
                else { per.push_str(&format!(" {}", &z)) }
            },
        };

        per
    }

    pub fn three_names(&self) -> String {
        let mut per: String = String::new();

        match self.first_name.clone() {
            None => per.push_str("-- "),
            Some(z) => {
                if z.is_empty() { per.push_str("-- "); }
                else { per.push_str(&format!("{} ", &z)) }
            },
        };

        match self.middle_name.clone() {
            None => {},
            Some(z) => {
                if !z.is_empty() {
                    per.push_str(&format!("{} ", &z))
                }
            },
        };

        match self.surname.clone() {
            None => per.push_str("--"),
            Some(z) => {
                if z.is_empty() { per.push_str("--"); }
                else { per.push_str(&format!("{}", &z)) }
            },
        };

        per
    }

    pub fn four_names(&self) -> String {
        let mut per: String = self.three_names();

        match self.maiden_name.clone() {
            None => {},
            Some(z) => {
                if !z.is_empty() {
                    per.push_str(&format!(" ({})", &z))
                }
            },
        };
        per
    }
 

}

pub fn op_naive_to_op_string(naive: Option<NaiveDate>) -> Option<String> {
    let mut movestr: Option<String> = None;
    match naive {
        None => {},
        Some(z) => { movestr = Some(z.to_string()) },
    }
    return movestr;
}

// give a question to the user and return the line as String
fn question_string(number: i32, old_string: Option<String>) -> Option<String> {
    let question: Vec<&str> = vec!("First name", "Middle name", "Surname", "Maiden name", "Gender");
    match old_string{
        None =>  println!("[{}] {}:", number, question[(number - 1) as usize]),
        Some(z) =>  println!("[{}] {}: {}", number, question[(number - 1) as usize], z),
    }
    let mut line: String = String::new();
    let n = std::io::stdin().read_line(&mut line).unwrap();
    if n <= 1 {
        return None;
    }else{ 
        line.pop();
        return Some(line.clone());
    }
}

fn question_naive(number: i32, old_string: Option<NaiveDate>, error: bool) -> Option<NaiveDate> {
    let question: Vec<&str> = vec!("Birthday", "Deathday");
    if error {
        println!("Warning: Use the following format for datetypes: dd.mm.yyyy, ex: 15.01.2002");
    }
    match old_string{
        None =>  println!("[{}] {}:", number, question[(number - 6) as usize]),
        Some(z) =>  println!("[{}] {}: {}", number, question[(number - 6) as usize], z),
    }
    let mut line: String = String::new();
    let n = std::io::stdin().read_line(&mut line).unwrap();
    if n <= 1 {
        return None;
    }else{ 
        let naive_date: Option<NaiveDate>;
        line.pop();
        match line.len() {
            8..=10 => { // 01.01.2000 , 1.01.2000 , 01.1.2000 , 1.1.2000
                naive_date = Some(NaiveDate::parse_from_str(&line, "%d.%m.%Y").unwrap());
            },
            6..=7 => { // 01.2000 , 1.2000
                line = format!("01.{}", line);
                naive_date = Some(NaiveDate::parse_from_str(&line, "%d.%m.%Y").unwrap());
            },
            4 => { // 2000
                line = format!("01.01.{}", line);
                naive_date = Some(NaiveDate::parse_from_str(&line, "%d.%m.%Y").unwrap());
            },
            _ => {
                naive_date = question_naive(number, old_string, true);
            },
        }
        return naive_date;
    }
}

// function to ask the user if there is a change to be made in the person object
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
            Some('1') => { person.first_name = question_string(1, person.first_name.clone()) },
            Some('2') => { person.middle_name = question_string(2, person.middle_name.clone()) },
            Some('3') => { person.surname = question_string(3, person.surname.clone()) },
            Some('4') => { person.maiden_name = question_string(4, person.maiden_name.clone()) },
            Some('5') => { person.gender = question_string(5, person.gender.clone()) },
            Some('6') => { person.birthday = question_naive(6, person.birthday.clone(), false) },
            Some('7') => { person.deathday = question_naive(7, person.deathday.clone(), false) },
            Some(_) => {
                line.pop();
                println!("{} is not a valid number", line);
            },
        }
        return (person, true);
    }
}

// create a person new person and display it to user
// the loop call ask user if something needs to be changed
pub fn create() -> Person{
    let mut person: Person = Person::new();
    loop {
        println!("\n\nNewly created Person:");
        println!("{}", person.to_string());
        let boo: bool;
        (person, boo) = change(person.clone());
        if !boo { return person }
    }
}

// ask the user to get a specific person object
pub fn search() -> Person {
    let mut results: Vec<Person>;
    loop {
        let mut line: String = String::new();
        println!("Who are you searching? Please enter the First Name and/or Last Name or their Id");
        let _ = std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        match line.chars().next() {
            None => { return Person::new(); }
            Some(z) if z.is_numeric() => { // One can only esacpe this loop by typing a Number/Id 
                let(a, _) = crate::mixed_to_single(line.clone(), 1);
                let id: i32 = a.parse::<i32>().unwrap();
                if id > 0 {
                    match crate::db::get_person_by_id(id) {
                        None => { println!("there is nobody with that id, please try again"); },
                        Some(z) => {
                            println!("[{}] {}", z.person_id, z.four_names());
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
                    None => results = crate::db::get_person_by_single_name(a),
                    Some(z) => results = crate::db::get_person_by_double_name(a, z),
                }
                if results.len() < 1 { println!("there is nobody with that name, please try again"); }
                else{ for res in results { println!("[{}] {}", res.person_id, res.four_names()); } }
            },
            _ => { /* because boo is still true the loop is still going and there is no need for error handling here */ },     
        }
    }
}



// get all children of a person 
// pub fn get_all_children(person: Person) -> Vec<Person> {
//     let mut all_c: Vec<Person> = Vec::new();
//     match person.clone().gender {
//         None=>{},
//         Some(z)=>{
//             let all_rela: Vec<Relation>;
//             if z == "m"{ all_rela = crate::db::person_id_to_relations(person.person_id, 2) }
//             else if z == "f"{ all_rela = crate::db::person_id_to_relations(person.person_id, 3) }
//             else if z == "um"{ all_rela = crate::db::person_id_to_relations(person.person_id, 3) }
//             else if z == "uf"{ all_rela = crate::db::person_id_to_relations(person.person_id, 2) }
//             else{ all_rela = Vec::new() }

//             for i in 0..all_rela.len() {
//                 match all_rela[i].clone().child {
//                     None => {},
//                     Some(z) => {
//                         if !all_c.contains(&z) {
//                             all_c.push(z);
//                         }
//                     }
//                 }
//             }
//         },
//     }
//     all_c
// }

// get all parents of a person
// pub fn get_all_parents(person: Person) -> Vec<Person> {
//     let mut all_p: Vec<Person> = Vec::new();
//     let all_rela: Vec<Relation> = crate::db::person_id_to_relations(person.person_id, 1);

//     for i in 0..all_rela.len() {
//         match all_rela[i].clone().male {
//             None => {},
//             Some(z) => {
//                 if !all_p.contains(&z) {
//                     all_p.push(z);
//                 }
//             }
//         }
//         match all_rela[i].clone().female {
//             None => {},
//             Some(z) => {
//                 if !all_p.contains(&z) {
//                     all_p.push(z);
//                 }
//             }
//         }
//     }

//     all_p
// }

// find a year to display
// pub fn find_year(person: Person, years: HashMap<i32, i32>) -> i32 {
//     let all_children: Vec<Person> = get_all_children(person.clone());
//     let all_parents: Vec<Person> = get_all_parents(person.clone());

//     let mut lowest_child_year: i32 = 10000;
//     let mut highest_parent_year: i32 = 0;

//     for i in 0..all_children.len() {
//         match &years.get(&all_children[i].person_id) {
//             None => {},
//             Some(year) => { if **year < lowest_child_year { lowest_child_year = **year } }
//         }
//     }

//     if lowest_child_year != 10000 { return lowest_child_year+25 }

//     for i in 0..all_parents.len() {
//         match &years.get(&all_parents[i].person_id) {
//             None => {},
//             Some(year) => { if **year > highest_parent_year { highest_parent_year = **year } }
//         }
//     }

//     if highest_parent_year != 0 { return highest_parent_year-25 }
//     else {
//         println!("NO LUCK on {}", person.person_id);
//         return 0;
//     }
// }