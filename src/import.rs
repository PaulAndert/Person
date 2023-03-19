use std::fs;
use mysql::chrono::NaiveDate;
use substring::Substring;
use crate::family::Family;
use crate::person::Person;
use crate::db;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TempFamily {
    father: String,
    mother: String,
    child: Person,
}
impl TempFamily {
    pub fn new () -> TempFamily {
        return TempFamily{
            father: String::new(),
            mother:String::new(),
            child: Person::new(),
        };
    }
}

pub fn import_obsidian(reset_table: bool) {

    if reset_table { db::reset_db();}

    let mut insert_familys: Vec<TempFamily> = Vec::new();

    let path: String = String::from("/Users/aya/Library/Mobile Documents/iCloud~md~obsidian/Documents/Library/Family/Personen");

    let names = fs::read_dir(path.clone()).unwrap();

    for name in names {

        let name_path: String = format!("{}", name.unwrap().path().display());

        // println!("Name: {}", name_path);

        let content: String = fs::read_to_string(name_path).expect("Should have been able to read the file");

        let mut temp: TempFamily = TempFamily::new();
        (temp.child, temp.father, temp.mother) = get_family_from_content(content);

        temp.child.person_id = db::insert_person(temp.child.clone()) as i32;

        insert_familys.push(temp);

        //break;

    }

    for temp in insert_familys {
        let mut insert: Family = Family::new();

        let mut cnt: i32 = 0;
        insert.male = match db::get_person_by_variable_name(temp.father) {
            None => {cnt += 1; None},
            Some(fath) => {Some(fath)},
        };
        insert.female = match db::get_person_by_variable_name(temp.mother) {
            None => {cnt += 1; None},
            Some(moth) => {Some(moth)},
        };
        insert.children.push(temp.child);

        if cnt == 0 {
            // println!("{}", insert.to_string());
            db::insert_family(insert);
        }
    }


}

fn get_family_from_content(content: String) -> (Person, String, String) {
    let mut ret_person: Person = Person::new();
    let mut father_str: String = String::new();
    let mut mother_str: String = String::new();

    for line in content.split("\n") {
        if line.len() > 0 {
            // println!("{}", line);
            match line.substring(0,6) {
                "#Perso" => {},
                "Vornam" => {
                    let value: String = line.replace("Vorname: ", "");
                    if value.len() > 0 && (value.replace(" ", "") != "--" && value.replace(" ", "") != "??") {
                        ret_person.first_name = Some(value);
                    }
                },
                "Zweitn" => {
                    let value: String = line.replace("Zweitname: ", "");
                    if value.len() > 0 && (value.replace(" ", "") != "--" && value.replace(" ", "") != "??") {
                        ret_person.middle_name = Some(value);
                    }
                },
                "Nachna" => {
                    let value: String = line.replace("Nachname: ", "");
                    if value.len() > 0 && (value.replace(" ", "") != "--" && value.replace(" ", "") != "??") {
                        ret_person.surname = Some(value);
                    }
                },
                "Geburt" => {
                    match line.substring(0,10) {
                        "Geburtsnam" => {
                            let value: String = line.replace("Geburtsname: ", "");
                            if value.len() > 0 && (value.replace(" ", "") != "--" && value.replace(" ", "") != "??") {
                                ret_person.maiden_name = Some(value);
                            }
                        },
                        "Geburtstag" => {
                            let value: String = line.replace("Geburtstag: ", "");
                            if value.len() > 0 && (value.replace(" ", "") != "--" && value.replace(" ", "") != "??") {
                                ret_person.birthday = match NaiveDate::parse_from_str(&value, "%d.%m.%Y") {
                                    Err(_e) => {println!("Warning: Birthday cant be made native {}", value); None},
                                    Ok(date) => {Some(date)},
                                }
                            }
                        },
                        _ => {},
                    }
                },
                "Mutter" => {
                    let mut value: String = line.replace("Mutter: ", "");
                    if value.len() > 0 && (value.replace(" ", "") != "--" && value.replace(" ", "") != "??") {
                        if value.starts_with("[[") {
                            value.pop();
                            value.pop();
                            value.remove(0);
                            value.remove(0);
                        }
                        mother_str = value;
                    }
                },
                "Vater:" => {
                    let mut value: String = line.replace("Vater: ", "");
                    if value.len() > 0 && (value.replace(" ", "") != "--" && value.replace(" ", "") != "??") {
                        if value.starts_with("[[") {
                            value.pop();
                            value.pop();
                            value.remove(0);
                            value.remove(0);
                        }
                        father_str = value;
                    }
                },
                "Bezieh" => {},
                "Kinder" => {},
                "Beruf:" => {},
                "Geschl" => {
                    let value: String = line.replace("Geschlecht: ", "");
                    if value.len() > 0 && (value.replace(" ", "") != "--" && value.replace(" ", "") != "??") {
                        ret_person.gender = Some(value);
                    }
                },
                _ => {println!("Warning: Not Catched: {}", line);},
            }
        }
    }

    // println!("{}", ret_person.to_string());

    (ret_person, father_str, mother_str)
}