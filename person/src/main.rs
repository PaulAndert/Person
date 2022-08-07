
extern crate mysql;
#[macro_use]
extern crate lazy_static;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

#[derive(Debug, PartialEq, Eq, Clone)]
struct Person {
    person_id: i32,
    vorname: Option<String>,
    zweitname: Option<String>,
    nachname: Option<String>,
    geburtsname: Option<String>, 
    geburtstag: Option<String>,
}

impl Person {
    fn new () -> Person {
        return Person{
            person_id: -1,
            vorname: Some(String::new()),
            zweitname: Some(String::new()),
            nachname: Some(String::new()),
            geburtsname: Some(String::new()),
            geburtstag: Some(String::new()),
        };
    }
}

lazy_static! {
    static ref POOL : mysql::Pool = mysql::Pool::new("mysql://root:Gravure1247@localhost:3306/person").unwrap();
}

fn get_all_persons() -> Vec<Person>{

    return POOL.prep_exec("SELECT * from person", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (person_id, vorname, zweitname, nachname, geburtsname, geburtstag) = mysql::from_row(row);

                Person {
                    person_id,
                    vorname,
                    zweitname,
                    nachname,
                    geburtsname,
                    geburtstag
                }
            }).collect()
        }).unwrap(); // Unwrap `Vec<Person>`
}

fn print_vector_person(all: Vec::<Person>) {
    for person in all.iter() {
        print_person(Some(person.clone()));
    }
}

fn print_person(person: Option<Person>) {
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
        },
    }
    println!("{}", per);
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

fn ask_for_changes(mut p: Person) -> (Person, bool) {
    let mut line: String = String::new();
    println!("for changes type the number you want to change, else press enter");
    let n = std::io::stdin().read_line(&mut line).unwrap();
    
    if n <= 1 {
        return (p, true);
    } else if n >= 3 {
        line.pop();
        println!("{} is not a valid number", line);
        return (p, false);
    }else{
        match line.chars().next() {
            None => return (p, false),
            Some('1') => p.vorname = question(p.vorname, 1, "Vorname".to_string()),
            Some('2') => p.zweitname = question(p.zweitname, 2, "Zweitname".to_string()),
            Some('3') => p.nachname = question(p.nachname, 3, "Nachname".to_string()),
            Some('4') => p.geburtsname = question(p.geburtsname, 4, "Geburtsname".to_string()),
            Some('5') => p.geburtstag = question(p.geburtstag, 5, "Geburtstag".to_string()),
            Some(_) => {
                line.pop();
                println!("{} is not a valid number", line);
            },
        }
        return (p, false);
    }
}

fn create_new_person() -> Option<Person>{
    let mut new_person: Person = Person::new();
    
    new_person.vorname = question(None, 1, "Vorname".to_string());
    new_person.zweitname = question(None, 2, "Zweitname".to_string());
    new_person.nachname = question(None, 3, "Nachname".to_string());
    new_person.geburtsname = question(None, 4, "Geburtsname".to_string());
    new_person.geburtstag = question(None, 5, "Geburtstag".to_string());

    let mut boo: bool = false;
    while boo == false{
        println!("\n\nNewly created Person:");
        print_person(Some(new_person.clone()));
        (new_person, boo) = ask_for_changes(new_person.clone());
    }

    return Some(new_person);
}

#[allow(unused_must_use)]
fn insert_new_person(person: Option<Person>){
    match person {
        None => println!("No person found"),
        Some(person) => {
            let mut insert: String = String::from("INSERT INTO person (vorname, zweitname, nachname, geburtsname, geburtstag) VALUES (\"");

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
            insert.push_str("\");");
            
            POOL.prep_exec(insert, ());
        },
    };

}

fn main(){

    let p1: Option<Person> = create_new_person();

    // Person{
    //     person_id: -1, 
    //     vorname: Some("Peter".to_string()), 
    //     zweitname: Some("zwei".to_string()), 
    //     nachname: Some("Neu".to_string()), 
    //     geburtsname: Some("Alt".to_string()),
    //     geburtstag: Some("01.01.2000".to_string()),
    // };

    print_person(p1);


    // let p: Option<Person> = create_new_person();

    // insert_new_person(p);

    // let all: Vec<Person> = get_all_persons();

    // print_vector_person(all);

    //print_person(Some(all[0].clone()));

}
