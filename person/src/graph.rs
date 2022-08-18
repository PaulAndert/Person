use crate::Person;
use crate::relation::Relation;
use std::io::{self, Write};
use std::process::Command;

fn form(posx: i32, year: f32) -> String {
    //println!("Form {}", year);
    let mut ret: String = String::new();
    ret.push_str("x");
    if posx >= 0 { ret.push_str(&posx.to_string()) }
    else {
        ret.push_str("_");
        ret.push_str(&posx.abs().to_string());
    }
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
        if year < 1.0 {
            year_nachkomma = ( year * 100.0 ) as i32;
        }else {
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

// if person has no birthdate -> cild.date + 25
fn get_year(person: Person, child: Person) -> (f32, String) {
    let year: String;
    match person.clone().birthday {
        None => {
            match child.birthday {
                None => {return (0.0, "Unknown".to_string())},
                Some(mut child_geb) => {
                    match child_geb.len() {
                        4 => { year = child_geb },
                        7 => { year = child_geb.split_off(3) },
                        10 => { year = child_geb.split_off(6) },
                        _ => {
                            println!("{} has not a correct birthday value", crate::person::get_person_names(Some(person)));
                            return (0.0, "Unknown".to_string());
                        }
                    }
                    return (translate(year.parse::<i32>().unwrap()-30, 1850, 2050, 0, -20), "Unknown".to_string());
                },
            }
        },
        Some(mut geb) => {
            if geb.is_empty() {
                match child.birthday {
                    None => {return (0.0, "Unknown".to_string())},
                    Some(mut child_geb) => {
                        match child_geb.len() {
                            4 => { year = child_geb },
                            7 => { year = child_geb.split_off(3) },
                            10 => { year = child_geb.split_off(6) },
                            _ => {
                                println!("{} has not a correct birthday value", crate::person::get_person_names(Some(person)));
                                return (0.0, "Unknown".to_string());
                            }
                        }
                        return (translate(year.parse::<i32>().unwrap()-30, 1850, 2050, 0, -20), "Unknown".to_string());
                    },
                }
            }else {
                match geb.len() {
                    4 => { year = geb },
                    7 => { year = geb.split_off(3) },
                    10 => { year = geb.split_off(6) },
                    _ => {
                        println!("{} has not a correct birthday value", crate::person::get_person_names(Some(person)));
                        return (0.0, "Unknown".to_string());
                    }
                }
                return (translate(year.parse::<i32>().unwrap(), 1850, 2050, 0, -20), year);
            }
        },
    }
}

fn graph_node(person: Person, posx: i32, year_float: f32, year_string: String) -> String{
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

fn graph_edge(posx: i32, edge_höhe: f32) -> String {
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
    ret.push_str(&form(x-breit, f_höhe));
    ret.push_str(" -> ");
    ret.push_str(&form(x-breit, edge_höhe));
    ret.push_str(" -> ");
    ret.push_str(&form(x, edge_höhe));
    ret.push_str(" -> ");
    ret.push_str(&form(x+breit, edge_höhe));
    ret.push_str(" -> ");
    ret.push_str(&form(x+breit, m_höhe));
    ret.push_str(" ; ");
    ret.push_str(&form(x, edge_höhe));
    ret.push_str(" -> ");
    ret.push_str(&form(x, year));
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
            if all_rela.len() > 0 {
                let (year_float_child, year_string_child) = get_year(z.clone(), z.clone());
                ret.push_str(&graph_node(z.clone(), x, year_float_child, year_string_child));

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
                        (year_float_female, year_string_female) = get_year(f.clone(), z.clone());
                        ret.push_str("\n");
                        ret.push_str(&graph_node(f.clone(), x-breit, year_float_female, year_string_female.clone()));
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
                        ret.push_str(&graph_node(female_copy, x+breit, year_float_female, year_string_female));
                    },
                    Some(m) => { 
                        let year_string_male;
                        (year_float_male, year_string_male) = get_year(m.clone(), z.clone());
                        ret.push_str("\n");
                        ret.push_str(&graph_node(m.clone(), x+breit, year_float_male, year_string_male.clone()));
                        if female_not_there { // if there is no female u need a Unknown placeholder
                            ret.push_str("\n");
                            let mut male_copy: Person = m.clone();
                            male_copy.first_name = Some(String::from("Unknown"));
                            male_copy.middle_name = None;
                            male_copy.surname = Some(String::from(" "));
                            male_copy.maiden_name = None;
                            male_copy.gender = Some(String::from("uf"));
                            ret.push_str(&graph_node(male_copy, x-breit, year_float_male, year_string_male));
                        }
                    },
                }
                // // Edge höhe = tiefsten parent nehmen und dann + child / 2 
                let edge_höhe: f32;
                if year_float_male <= year_float_female { edge_höhe = (year_float_male + year_float_child ) / 2.0 } 
                else { edge_höhe = (year_float_female + year_float_child ) / 2.0 }
                ret.push_str(&graph_edge(x-breit, edge_höhe));
                ret.push_str(&graph_edge(x, edge_höhe));
                ret.push_str(&graph_edge(x+breit, edge_höhe));

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
    node [fontsize=11, fixedsize=true, height=1.5, width=1.5];
    y1900 [shape=none, label=\"1900\", pos=\"-{h},{y1900}!\"];
    y2000 [shape=none, label=\"2000\", pos=\"-{h},{y2000}!\"];
    y0 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"{h},{y1900}!\"];
    y1 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"{h},{y2000}!\"];
    y1900 -> y0 [style=dashed] ; y2000 -> y1 [style=dashed]\n\n", 
    h = (2_i32.pow((cur_gen - 1).try_into().unwrap()) + 4),
    y1900 = translate(1900, 1850, 2050, 0, -20),
    y2000 = translate(2000, 1850, 2050, 0, -20)));
    let mut file = std::fs::File::create("data.dot").expect("create failed");

    // 1 -> 0, 2 -> 1, 3 -> 2, 4 -> 4, 5 -> 8, 6 -> 16 :: (2^cur_gen-2)
    let mode:i32 = 1; // 0: no mode, 1: child, 2: male, 3: female

    file.write_all(gra.as_bytes()).expect("write failed");
    file.write_all(one_relation(0, crate::person::search(), cur_gen, mode).as_bytes()).expect("write failed");
    file.write_all("\n}".as_bytes()).expect("write failed");
}

pub fn dot_to_svg() {
    let output = Command::new("dot")
        .args(["-Kfdp", "-n", "-Tsvg", "-o", "data.svg", "data.dot"])
        .output()
        .expect("Failed to execute command");
    io::stdout().write_all(&output.stdout).unwrap();
}