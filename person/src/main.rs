
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

fn question(old_string: String, number: i32, question: String) -> Option<String> {
    println!("[{}] {}: {}", number, question, old_string);
    let mut line: String = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    if line.is_empty() { return None; }
    else{ return Some(line.clone()); }
}

fn create_new_person() -> Option<Person>{
    let mut new_person: Person = Person::new();
    
    new_person.vorname = question("".to_string(), 1, "Vorname".to_string());
    new_person.zweitname = question("".to_string(), 2, "Zweitname".to_string());
    new_person.nachname = question("".to_string(), 3, "Nachname".to_string());
    new_person.geburtsname = question("".to_string(), 4, "Geburtsname".to_string());
    new_person.geburtstag = question("".to_string(), 5, "Geburtstag".to_string());

    println!("Newly created Person:");
    print_person(Some(new_person.clone()));
    println!("for changes type the number you want to change, else type y");

    // feature to change value if number is typed or go on and return if y is typed

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
