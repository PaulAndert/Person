use crate::Person;
use crate::relation::Relation;
use std::io::{self, Write};
use std::process::Command;

const OLDEST: f32 = 1850.0;
const NEWEST: f32 = 2050.0;
const SCALE_TO_UPPER: f32 = 0.0;
const SCALE_TO_LOWER: f32 = -25.0;

pub fn form(posx: f32, year: f32) -> String {
    let mut ret: String = String::new();
    ret.push_str("x");
    if posx >= 0.0 { 
        ret.push_str(&float_to_string(posx));
    } else {
        ret.push_str("_");
        ret.push_str(&float_to_string(posx));
    }
    ret.push_str("y");
    if year >= 0.0 { 
        ret.push_str(&float_to_string(year));
    } else {
        ret.push_str("_");
        ret.push_str(&float_to_string(year));
    }
    ret
}

fn float_to_string(float: f32) -> String {
    let vorkomma: i32 = float as i32;
    let nachkomma: i32;
    if float.abs() < 1.0 { nachkomma = ( float * 100.0 ) as i32 }
    else { nachkomma = (float % vorkomma as f32 * 100.0 ) as i32 }
    format!("{}_{}", vorkomma.abs().to_string(), nachkomma.abs().to_string())
}
pub fn translate(value: i32) -> f32 {
    let left_span: f32 = NEWEST - OLDEST;
    let right_span: f32 = SCALE_TO_LOWER - SCALE_TO_UPPER;
    let value_scaled: f32 = (value as f32 - OLDEST) / left_span;
    return SCALE_TO_UPPER + (value_scaled * right_span);
}

fn get_year_from_birthday(mut birthday: String) -> i32 {
    match birthday.len() {
        4 => { return birthday.parse::<i32>().unwrap() },
        7 => { return birthday.split_off(3).parse::<i32>().unwrap() },
        10 => { return birthday.split_off(6).parse::<i32>().unwrap() },
        _ => { return 0 }
    }
}

fn get_year_of_child(person: Person) -> i32 {
    let all_children: Vec<Person> = crate::person::get_all_children(person.clone());
    if all_children.len() > 0 {
        let mut children_year: i32 = 10000;

        for i in 0..all_children.len() {
            match all_children[i].birthday.clone() {
                None => {},
                Some(z) => {
                    if !z.is_empty() {
                        let year_i32: i32 = get_year_from_birthday(z);
                        if year_i32 == 0 {
                            println!("{} has no correct birthday value", crate::person::get_all_4_names(all_children[i].clone()));
                        }else if year_i32 < children_year{
                            children_year = year_i32;
                        }
                    }
                }
            }
        }
        return children_year;
    }else { return 0 }
}

fn get_year_of_parent(person: Person) -> i32 {
    let all_parents: Vec<Relation> = crate::db::person_id_to_relations(person.person_id, 1);
    if all_parents.len() > 0 {
        let mut parent_year: i32 = 0;
        let mut parents_have_no_bd_cnt: i32 = 0;
        match all_parents[0].male.clone() {
            None => {parents_have_no_bd_cnt += 1},
            Some(z) => {
                match z.clone().birthday{
                    None => {parents_have_no_bd_cnt += 1},
                    Some(geb) => {
                        if !geb.is_empty() {
                            let year_i32: i32 = get_year_from_birthday(geb);
                            if year_i32 == 0 {
                                println!("{} has no correct birthday value", crate::person::get_all_4_names(z.clone()));
                            }else {
                                parent_year = year_i32;
                            }
                        }else { parents_have_no_bd_cnt += 1 }
                    },
                }
            }
        }
        match all_parents[0].female.clone() {
            None => {parents_have_no_bd_cnt += 1},
            Some(z) => {
                match z.clone().birthday{
                    None => {parents_have_no_bd_cnt += 1},
                    Some(geb) => {
                        if !geb.is_empty() {
                            let year_i32: i32 = get_year_from_birthday(geb);
                            if year_i32 == 0 {
                                println!("{} has no correct birthday value", crate::person::get_all_4_names(z.clone()));
                            }else if year_i32 > parent_year {
                                parent_year = year_i32;
                            }
                        }else { parents_have_no_bd_cnt += 1 }
                    },
                }
            }
        }
        if parents_have_no_bd_cnt == 2 || parent_year == 0{
            match all_parents[0].male.clone() {
                None => {},
                Some(z) => {
                    parent_year = get_year_of_child(z.clone());
                }
            }
            match all_parents[0].female.clone() {
                None => {},
                Some(z) => {
                    let ret_i32 = get_year_of_child(z.clone());
                    if ret_i32 > parent_year {
                        parent_year = ret_i32;
                    }
                }
            }
            return parent_year-25;
        }
        return parent_year;
    }else { return 0 }
}

pub fn get_year(person: Person) -> (f32, String) {
    match person.clone().birthday {
        None => { 
            let ret_i32 = get_year_of_child(person.clone());
            if ret_i32 != 0 { 
                return (translate(ret_i32-25), String::from("Unknown")) ; 
            } else { 
                let ret_i32 = get_year_of_parent(person.clone()); 
                if ret_i32 != 0 { 
                    return (translate(ret_i32+25), String::from("Unknown")) ; 
                }else{
                    return (SCALE_TO_UPPER, String::from("Unknown")) ; // Worst case if person has no children or parents with bd
                }
            }
        },
        Some(z) => {
            if z.is_empty() { 
                let ret_i32 = get_year_of_child(person.clone());
                if ret_i32 != 0 { 
                    return (translate(ret_i32-25), String::from("Unknown")) ; 
                } else { 
                    let ret_i32 = get_year_of_parent(person.clone()); 
                    if ret_i32 != 0 { 
                        return (translate(ret_i32+25), String::from("Unknown")) ; 
                    }else{
                        return (SCALE_TO_UPPER, String::from("Unknown")) ; // Worst case if person has no children or parents with bd
                    }
                }
            }else {
                let year: i32 = get_year_from_birthday(z);
                if year != 0 {
                    return (translate(year), year.to_string()) ;
                }else{
                    let ret_i32 = get_year_of_child(person.clone());
                    if ret_i32 != 0 { 
                        return (translate(ret_i32-25), String::from("Unknown")) ; 
                    } else { 
                        let ret_i32 = get_year_of_parent(person.clone()); 
                        if ret_i32 != 0 { 
                            return (translate(ret_i32+25), String::from("Unknown")) ; 
                        }else{
                            return (SCALE_TO_UPPER, String::from("Unknown")) ; // Worst case if person has no children or parents with bd
                        }
                    }
                }
            }
        },
    }
}

pub fn graph_node(person: Person, posx: f32) -> String{
    let (year_float, year_string) = get_year(person.clone());
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
    let name: String = crate::person::get_3_names(person.clone());
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
    let mut ret: String = String::new();
    ret.push_str(&form(posx, edge_höhe));
    ret.push_str(" [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"");
    ret.push_str(&posx.to_string());
    ret.push(',');
    ret.push_str(&edge_höhe.to_string());
    ret.push_str("!\"];\n");
    return ret;
}

pub fn graph(mut cur_gen: i32){
    if cur_gen == -1 { cur_gen = 4 }

    let gra: String = String::from(format!("digraph P {{
    edge [dir=forward, arrowhead=none];
    node [fontsize=11, fixedsize=true, height=1.5, width=1.5];\n\n"));
    let mut file = std::fs::File::create("data.dot").expect("create failed");

    file.write_all(gra.as_bytes()).expect("write failed");
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