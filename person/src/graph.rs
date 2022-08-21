use crate::Person;
use crate::relation::Relation;
use std::io::{self, Write};
use std::process::Command;

pub fn form(posx: f32, year: f32) -> String {
    //println!("Form {}", year);
    let mut ret: String = String::new();
    ret.push_str("x");
    // if posx >= 0.0 { ret.push_str(&posx.to_string()) }
    // else {
    //     ret.push_str("_");
    //     ret.push_str(&posx.abs().to_string());
    // }
    if posx >= 0.0 { 
        let posx_vorkomma: i32 = posx as i32;
        let posx_nachkomma: i32;
        if posx.abs() < 1.0 {
            //println!("{}", year);
            posx_nachkomma = ( posx * 100.0 ) as i32;
        }else {
            //println!("{} % {} = {}", year, year_vorkomma, year % year_vorkomma as f32);
            posx_nachkomma = (posx % posx_vorkomma as f32 * 100.0 ) as i32;
        }
        //println!("{} , {}", year_vorkomma, year_nachkomma);
        ret.push_str(&posx_vorkomma.abs().to_string());
        ret.push_str("_");
        ret.push_str(&posx_nachkomma.abs().to_string());
    } else {
        let posx_vorkomma: i32 = posx as i32;
        let posx_nachkomma: i32;
        if posx.abs() < 1.0 {
            //println!("{}", year);
            posx_nachkomma = ( posx * 100.0 ) as i32;
        }else {
            //println!("{} % {} = {}", year, year_vorkomma, year % year_vorkomma as f32);
            posx_nachkomma = (posx % posx_vorkomma as f32 * 100.0 ) as i32;
        }
        //println!("{} , {}", year_vorkomma, year_nachkomma);
        ret.push_str("_");
        ret.push_str(&posx_vorkomma.abs().to_string());
        ret.push_str("_");
        ret.push_str(&posx_nachkomma.abs().to_string());
    }

    ///// --> 
    ret.push_str("y");
    if year >= 0.0 { 
        let year_vorkomma: i32 = year as i32;
        let year_nachkomma: i32 = (year % year_vorkomma as f32 * 100.0 ) as i32;
        ret.push_str(&year_vorkomma.abs().to_string());
        ret.push_str("_");
        ret.push_str(&year_nachkomma.abs().to_string());
    } else {
        let year_vorkomma: i32 = year as i32;
        let year_nachkomma: i32;
        if year.abs() < 1.0 {
            //println!("{}", year);
            year_nachkomma = ( year * 100.0 ) as i32;
        }else {
            //println!("{} % {} = {}", year, year_vorkomma, year % year_vorkomma as f32);
            year_nachkomma = (year % year_vorkomma as f32 * 100.0 ) as i32;
        }
        //println!("{} , {}", year_vorkomma, year_nachkomma);
        ret.push_str("_");
        ret.push_str(&year_vorkomma.abs().to_string());
        ret.push_str("_");
        ret.push_str(&year_nachkomma.abs().to_string());
    }
    //println!("{}", ret);
    return ret;
}

fn translate(value: i32, left_min: i32, left_max: i32, right_min: i32, right_max: i32) -> f32 {
    let left_span: f32 = (left_max - left_min) as f32;
    let right_span: f32 = (right_max - right_min) as f32;
    let value_scaled: f32 = (value as f32 - left_min as f32) / left_span;
    return right_min as f32 + (value_scaled * right_span);
}

fn get_year_from_birthday(mut birthday: String) -> i32 {
    match birthday.len() {
        4 => { return birthday.parse::<i32>().unwrap() },
        7 => { return birthday.split_off(3).parse::<i32>().unwrap() },
        10 => { return birthday.split_off(6).parse::<i32>().unwrap() },
        _ => { return 0 }
    }
}

fn get_year_of_child(person: Person) -> (f32, String) {
    let all_children: Vec<Person> = crate::person::get_all_children(person.clone());
    if all_children.len() > 0 {
        let mut child: Person = all_children[0].clone();
        let mut children_year: i32 = 10000;

        for i in 0..all_children.len() {
            match all_children[i].birthday.clone() {
                None => {},
                Some(z) => {
                    if !z.is_empty() {
                        let year_i32: i32 = get_year_from_birthday(z);
                        if year_i32 == 0 {
                            println!("{} has no correct birthday value", crate::person::get_person_names(Some(all_children[i].clone())));
                        }else if year_i32 < children_year{
                            child = all_children[i].clone();
                            children_year = year_i32;
                        }
                    }
                }
            }
        }
        return (translate(children_year-25, 1850, 2050, 0, -20), (children_year-25).to_string());
    }else { return (0.0, (0.0).to_string()) }
}

pub fn get_year(person: Person) -> (f32, String) {
    match person.clone().birthday {
        None => { return get_year_of_child(person.clone()) },
        Some(z) => {
            if z.is_empty() { 
                return get_year_of_child(person.clone())
            }else {
                let year: i32 = get_year_from_birthday(z);
                return (translate(year, 1850, 2050, 0, -20), year.to_string()) 
            }
        },
    }
}

pub fn graph_node(person: Person, posx: f32, year_float: f32, year_string: String) -> String{
    let mut ret: String = String::new();
    ret.push_str(&form(posx, year_float));
    ret.push_str(" [shape=");
    match person.gender{
        None => {},
        Some(ref g) => {
            if g == "m" { ret.push_str("square, color=\"blue\",") }
            else if g == "f" { ret.push_str("circle, color=\"pink\",") }
            else if g == "um" { ret.push_str("square, color=\"grey\",") }
            else if g == "uf" { ret.push_str("circle, color=\"grey\",") }
            else { ret.push_str("star, color=\"yellow\",") }
        },
    }
    ret.push_str("label=\"");
    let name: String = crate::person::vor_zweit_nach(Some(person.clone()));
    if name.len() > 20 {
        let mut split = name.split_whitespace();
        match split.next() { None => {println!("There is a naming error in person {}", person.person_id)},
            Some(a) => { 
                match split.next() { None => {println!("There is a naming error in person {}", person.person_id)},
                    Some(b) => {
                        if a.len() + b.len() < 20{
                            ret.push_str(a);
                            ret.push_str(" ");
                            ret.push_str(b);
        } } } } }

        ret.push_str("\n");
        for _ in 0..split.clone().count() {
            ret.push_str(match split.next() { None => {&""}, Some(z) => z},);
            ret.push_str(" ");
        }
    }else{ ret.push_str(&name) }
    ret.push_str("\n");
    ret.push_str(&year_string);
    ret.push_str("\", pos=\"");
    ret.push_str(&posx.to_string());
    ret.push(',');
    ret.push_str(&year_float.to_string());
    ret.push_str("!\"];\n");
    return ret;
}

pub fn graph_edge(posx: f32, edge_höhe: f32) -> String {
    //println!("{} {}", posx, edge_höhe);
    let mut ret: String = String::new();
    ret.push_str(&form(posx, edge_höhe));
    ret.push_str(" [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"");
    ret.push_str(&posx.to_string());
    ret.push(',');
    ret.push_str(&edge_höhe.to_string());
    ret.push_str("!\"];\n");
    return ret;
}

fn connect(x: i32, year: f32, breit: i32, edge_höhe: f32, f_höhe: f32, m_höhe: f32) -> String {
    let mut ret: String = String::new();
    ret.push_str(&form((x-breit) as f32, f_höhe));
    ret.push_str(" -> ");
    ret.push_str(&form((x-breit) as f32, edge_höhe));
    ret.push_str(" -> ");
    ret.push_str(&form(x as f32, edge_höhe));
    ret.push_str(" -> ");
    ret.push_str(&form((x+breit) as f32, edge_höhe));
    ret.push_str(" -> ");
    ret.push_str(&form((x+breit) as f32, m_höhe));
    ret.push_str(" ; ");
    ret.push_str(&form(x as f32, edge_höhe));
    ret.push_str(" -> ");
    ret.push_str(&form(x as f32, year));
    return ret;
}

fn one_relation(x: i32, main: Option<Person>, cur_gen:i32, mode: i32) -> String{
    if cur_gen < 2 { return "".to_string() }
    let mut ret: String = String::new();
    match main.clone(){
        None => {println!("There is no person in one_relation")},
        Some(z) => { 
            let all_rela: Vec<Relation> = crate::db::person_to_relations(main.clone(), mode);
            let rela: Relation;

            // mehr platz zwischen elternteile und mach pro kind ein mehr edge die connected werden muss
// Das war mal ein test
            // let parent_rela: Vec<Relation>;
            // if z.gender.as_ref().unwrap() == "m"{
            //     parent_rela = crate::db::person_to_relations(main.clone(), 2);
            // }else if z.gender.as_ref().unwrap() == "f"{
            //     parent_rela = crate::db::person_to_relations(main.clone(), 3);
            // }else{
            //     parent_rela = Vec::new();
            // }
            //println!("{}: {}", crate::person::get_person_names(main.clone()), parent_rela.len());
            // let breite_add: i32 = 0;
            // if parent_rela.len() > 1 {
            //     //breite_add = 
            // }

            if all_rela.len() > 0 {
                let (year_float_child, year_string_child) = get_year(z.clone());
                ret.push_str(&graph_node(z.clone(), x as f32, year_float_child, year_string_child));

                rela = all_rela[0].clone();
                // cur_gen -> breite zwischen nodes, 1 -> 0, 2 -> 1, 3 -> 2, 4 -> 4, 5 -> 8, 6 -> 16 :: (2^cur_gen-2)
                let breit: i32 = 2_i32.pow((cur_gen-2).try_into().unwrap());
                
                let mut year_float_female = 0.0;
                let mut year_string_female = String::new();
                let mut female_not_there = false;
                let mut male_not_there = false;
                let mut female_copy: Person = Person::new();
                match rela.female.clone() {
                    None => { female_not_there = true },
                    Some(f) => { 
                        female_copy = f.clone();
                        (year_float_female, year_string_female) = get_year(f.clone());
                        ret.push_str("\n");
                        ret.push_str(&graph_node(f.clone(), (x-breit) as f32, year_float_female, year_string_female.clone()));
                    },
                }
                let mut year_float_male = 0.0;
                match rela.male.clone() {
                    None => { // if there is no male u need a Unknown placeholder
                        male_not_there = true;
                        ret.push_str("\n");
                        female_copy.first_name = Some(String::from("Unknown"));
                        female_copy.middle_name = None;
                        female_copy.surname = Some(String::from(" "));
                        female_copy.maiden_name = None;
                        female_copy.gender = Some(String::from("um"));
                        ret.push_str(&graph_node(female_copy, (x+breit) as f32, year_float_female, year_string_female));
                    },
                    Some(m) => { 
                        let year_string_male;
                        (year_float_male, year_string_male) = get_year(m.clone());
                        ret.push_str("\n");
                        ret.push_str(&graph_node(m.clone(), (x+breit) as f32, year_float_male, year_string_male.clone()));
                        if female_not_there { // if there is no female u need a Unknown placeholder
                            ret.push_str("\n");
                            let mut male_copy: Person = m.clone();
                            male_copy.first_name = Some(String::from("Unknown"));
                            male_copy.middle_name = None;
                            male_copy.surname = Some(String::from(" "));
                            male_copy.maiden_name = None;
                            male_copy.gender = Some(String::from("uf"));
                            ret.push_str(&graph_node(male_copy, (x-breit) as f32, year_float_male, year_string_male));
                        }
                    },
                }
                // // Edge höhe = tiefsten parent nehmen und dann + child / 2 
                let edge_höhe: f32;
                if year_float_male <= year_float_female { edge_höhe = (year_float_male + year_float_child ) / 2.0 } 
                else { edge_höhe = (year_float_female + year_float_child ) / 2.0 }
                ret.push_str(&graph_edge((x-breit) as f32, edge_höhe));
                ret.push_str(&graph_edge(x as f32, edge_höhe));
                ret.push_str(&graph_edge((x+breit) as f32, edge_höhe));

                ret.push_str("\n");
                if female_not_there {
                    ret.push_str(&connect(x, year_float_child, breit, edge_höhe, year_float_male, year_float_male)) 
                } else if male_not_there {
                    ret.push_str(&connect(x, year_float_child, breit, edge_höhe, year_float_female, year_float_female)) 
                } else { 
                    ret.push_str(&connect(x, year_float_child, breit, edge_höhe, year_float_female, year_float_male)) 
                }

                ret.push_str("\n");
                ret.push_str(&one_relation(x-breit, rela.female, cur_gen-1, 1));

                ret.push_str("\n");
                ret.push_str(&one_relation(x+breit, rela.male, cur_gen-1, 1));

            }
        },
    }
    return ret;
}

pub fn graph(mut cur_gen: i32){
    // how high is the next node from the node under it
    // how many nodes over each other, needed to calculate how much space between nodes is needed
    if cur_gen == -1 { cur_gen = 4 }

    let gra: String = String::from(format!("digraph P {{
    edge [dir=forward, arrowhead=none];
    node [fontsize=11, fixedsize=true, height=1.5, width=1.5];"));
    // y1900 [shape=none, label=\"1900\", pos=\"-{h},{y1900}!\"];
    // y2000 [shape=none, label=\"2000\", pos=\"-{h},{y2000}!\"];
    // y0 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"{h},{y1900}!\"];
    // y1 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"{h},{y2000}!\"];
    // y1900 -> y0 [style=dashed] ; y2000 -> y1 [style=dashed]\n\n", 
    // h = (2_i32.pow((cur_gen - 1).try_into().unwrap()) + 4),
    // y1900 = translate(1900, 1850, 2050, 0, -20),
    // y2000 = translate(2000, 1850, 2050, 0, -20)));
    let mut file = std::fs::File::create("data.dot").expect("create failed");

    // 1 -> 0, 2 -> 1, 3 -> 2, 4 -> 4, 5 -> 8, 6 -> 16 :: (2^cur_gen-2)
    let mode:i32 = 1; // 0: no mode, 1: child, 2: male, 3: female

    file.write_all(gra.as_bytes()).expect("write failed");
    //file.write_all(one_relation(0, crate::person::search(), cur_gen, mode).as_bytes()).expect("write failed");
    file.write_all(crate::matrix::matrix_to_string(cur_gen).as_bytes()).expect("write failed");
    file.write_all("\n}".as_bytes()).expect("write failed");
}

pub fn dot_to_svg() {
    let output = Command::new("dot")
        .args(["-Kfdp", "-n", "-Tsvg", "-o", "data.svg", "data.dot"])
        .output()
        .expect("Failed to execute command");
    io::stdout().write_all(&output.stdout).unwrap();
}