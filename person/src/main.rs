extern crate mysql;
#[macro_use]
extern crate lazy_static;
use std::env;
use std::io::Write;

pub mod person;
pub mod relation;
pub mod db;

use crate::person::Person;
use crate::relation::Relation;

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn mixed_to_single(line: String, mode: i32) -> (String, Option<String> ){ // 0 -> ABC, 1 -> 123
    let mut first: String = String::new();
    let mut second: String = String::new();
    let mut boo: bool = true;

    for i in 0..line.len() {
        match line.chars().nth(i) {
            None => {},
            Some(a) => {
                if mode == 0 { // text mode
                    if a.is_alphabetic() { 
                        if boo { first.push(a) }
                        else { second.push(a) }
                    }else if a.is_whitespace() { boo = false }
                }else if mode == 1 && a.is_numeric() { first.push(a) }
            },
        }
    }
    if second.is_empty() { return (first, None) }
    else { return (first, Some(second)) }
}

fn form(posx: i32, year: f32) -> String {
    let mut ret: String = String::new();
    ret.push_str("x");
    if posx >= 0 { ret.push_str(&posx.to_string()) }
    else {
        ret.push_str("_");
        ret.push_str(&posx.abs().to_string());
    }
    ret.push_str("y");
    if year >= 0.0 { 
        let year1: i32 = year as i32;
        let year2: i32 = (year % year1 as f32 * 100.0 ) as i32;
        //println!("{} , {}", year1.abs(), year2.abs());

        ret.push_str(&year1.abs().to_string());
        ret.push_str("_");
        ret.push_str(&year2.abs().to_string());
    } else {
        let year1: i32 = year as i32;
        let year2: i32 = (year % year1 as f32 * 100.0 ) as i32;
        //println!("-{} , {}", year1.abs(), year2.abs());
        ret.push_str("_");
        ret.push_str(&year1.abs().to_string());
        ret.push_str("_");
        ret.push_str(&year2.abs().to_string());
    }
    return ret;
}

fn translate(value: i32, leftMin: i32, leftMax: i32, rightMin: i32, rightMax: i32) -> f32 {
    let leftSpan: f32 = (leftMax - leftMin) as f32;
    let rightSpan: f32 = (rightMax - rightMin) as f32;

    let valueScaled: f32 = (value as f32 - leftMin as f32) / leftSpan;

    return rightMin as f32 + (valueScaled * rightSpan);
}

// if pereson has no birthdate -> cild.date + 25
fn get_year(person: Person, child: Person) -> (f32, String) {
    let year: String;
    match person.clone().geburtstag {
        None => {
            match child.geburtstag {
                None => {return (0.0, "Unknown".to_string())},
                Some(mut child_geb) => {
                    match child_geb.len() {
                        4 => {
                            year = child_geb;
                        },
                        7 => {
                            year = child_geb.split_off(3);
                        },
                        10 => {
                            year = child_geb.split_off(6);
                        },
                        _ => {
                            println!("{} has not a correct birthday value", person::get_person_names(Some(person)));
                            return (0.0, "Unknown".to_string());
                        }
                    }
                    let yearI: i32 = year.parse::<i32>().unwrap();
                    return (translate(yearI-30, 1900, 2020, 0, -15), "Unknown".to_string());
                },
            }
        },
        Some(mut geb) => {
            match geb.len() {
                4 => {
                    year = geb;
                },
                7 => {
                    year = geb.split_off(3);
                },
                10 => {
                    year = geb.split_off(6);
                },
                _ => {
                    println!("{} has not a correct birthday value", person::get_person_names(Some(person)));
                    return (0.0, "Unknown".to_string());
                }
            }
            let yearI: i32 = year.parse::<i32>().unwrap();
            return (translate(yearI, 1900, 2020, 0, -15), year);
        },
    }
}

fn graph_node(person: Person, posx: i32, yearF: f32, yearS: String) -> String{
    let mut ret: String = String::new();
    ret.push_str(&form(posx, yearF));
    ret.push_str(" [shape=");
    match person.gender{
        None => {},
        Some(ref g) => {
            if g == "m" {
                ret.push_str("square, color=\"blue\",");
            }else if g == "f" {
                ret.push_str("circle, color=\"pink\",");
            }else {
                ret.push_str("star, color=\"yellow\",");
            }
        },
    }
    ret.push_str("label=\"");
    let name: String = person::vor_zweit_nach(Some(person.clone()));
    if name.len() > 20 {
        let mut split = name.split_whitespace();
        match split.next() { None => {},
            Some(a) => { 
                match split.next() { None => {},
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
    }else{
        ret.push_str(&name);
    }
    ret.push_str("\n");
    ret.push_str(&yearS);
    ret.push_str("\", pos=\"");
    ret.push_str(&posx.to_string());
    ret.push(',');
    ret.push_str(&yearF.to_string());
    ret.push_str("!\"];\n");
    return ret;
}

fn graph_edge(posx: i32, edge_höhe: f32) -> String {
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
    //println!("{}, {}, {}", edge_höhe, f_höhe, m_höhe);
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
        None => {},
        Some(z) => { 
            let all_rela: Vec<Relation> = db::person_to_relations(main.clone(), mode);
            let rela: Relation;
            if all_rela.len() > 0 {
                let (yearF, yearS) = get_year(z.clone(), z.clone());
                ret.push_str(&graph_node(z.clone(), x, yearF, yearS));

                rela = all_rela[0].clone();
                let breit: i32 = 2_i32.pow((cur_gen-2).try_into().unwrap());
                
                let (mut yearF_f, mut yearS_f) = (0.0, String::new());
                match rela.female.clone() {
                    None => {},
                    Some(f) => { 
                        (yearF_f, yearS_f) = get_year(f.clone(), z.clone());
                        ret.push_str("\n");
                        ret.push_str(&graph_node(f.clone(), x-breit, yearF_f, yearS_f));
                    },
                }
                let (mut yearF_m, mut yearS_m) = (0.0, String::new());
                match rela.male.clone() {
                    None => {},
                    Some(m) => { 
                        (yearF_m, yearS_m) = get_year(m.clone(), z.clone());
                        ret.push_str("\n");
                        ret.push_str(&graph_node(m, x+breit, yearF_m, yearS_m));
                    },
                }
                // // Edge höhe = tiefsten parent nehmen und dann + child / 2 
                let edge_höhe: f32;
                if yearF_m < yearF_m { 
                    edge_höhe = (yearF_m + yearF ) / 2.0;
                } else { 
                    edge_höhe = (yearF_f + yearF ) / 2.0;
                }
                ret.push_str(&graph_edge(x, edge_höhe));
                ret.push_str(&graph_edge(x-breit, edge_höhe));
                ret.push_str(&graph_edge(x+breit, edge_höhe));

                ret.push_str("\n");
                ret.push_str(&connect(x, yearF, breit, edge_höhe, yearF_f, yearF_m));

                ret.push_str("\n");
                ret.push_str(&one_relation(x-breit, rela.female, cur_gen-1, 1));

                ret.push_str("\n");
                ret.push_str(&one_relation(x+breit, rela.male, cur_gen-1, 1));

            }
        },
    }
    return ret;
}

fn graph(){
    let gra: String = String::from("digraph P {
    edge [dir=forward, arrowhead=none];
    node [fontsize=11, fixedsize=true, height=1.5, width=1.5];
    y1900 [shape=none, label=\"1900\", pos=\"-15,0!\"];
    y2000 [shape=none, label=\"2000\", pos=\"-15,-12.5!\"];
    y0 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"15,0!\"];
    y1 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"15,-12.5!\"];
    y1900 -> y0 [style=dashed] ; y2000 -> y1 [style=dashed]\n\n");
    let mut file = std::fs::File::create("data.dot").expect("create failed");

    //let gen_höhe = 2; // how high is the next node from the node under it
    let cur_gen = 2; // how many nodes over each other, needed to calculate how much space between nodes is needed
    // 1 -> 0, 2 -> 1, 3 -> 2, 4 -> 4, 5 -> 8, 6 -> 16 :: (2^cur_gen-2)
    let mode:i32 = 1; // 0: no mode, 1: child, 2: male, 3: female

    file.write_all(gra.as_bytes()).expect("write failed");
    file.write_all(one_relation(0, person::search(), cur_gen, mode).as_bytes()).expect("write failed");
    file.write_all("\n}".as_bytes()).expect("write failed");
}

fn main(){
    let args: Vec<_> = env::args().collect();
    let mut update: bool = false;
    if args.len() > 1 {
        if args.len() > 2 && args[2] == "-u" { update = true; }
        match args[1].as_str() {
            "-a" => {
                for p in db::get_all_persons() { person::print_names(Some(p)) }
            }
            "-g" => {
                graph();
            }
            "-p" => {
                if update { 
                    let mut p: Option<Person> = person::search(); 
                    let mut boo: bool = true;
                    while boo{
                        person::print(p.clone());
                        (p, boo) = person::change(p.clone());
                    }
                    db::update_person(p.clone());
                }else { db::insert_person(person::create()); }
            },
            "-r" => {
                if update { 
                    let original: Option<Relation> = relation::search();
                    let mut update: Option<Relation> = original.clone();
                    let mut boo: bool = true;
                    while boo{
                        relation::print(update.clone());
                        (update, boo) = relation::change(update.clone());
                    }
                    db::update_relation(original.clone(), update.clone());
                }else { db::insert_relation(relation::create()) }
            },
            "-s" => {
                person::print(person::search());
            }
            "-h" => {
                println!("TODO print help message");
            },
            z => println!("{} is not a valid parameter", z),
        }
    } else { println!("No Args found, please look up the documentation for how to use this tool"); }


    // let p1: Option<Person> = create_new_person();
    // print_person(p1);
    // Person{
    //     person_id: -1, 
    //     vorname: Some("Peter".to_string()), 
    //     zweitname: Some("zwei".to_string()), 
    //     nachname: Some("Neu".to_string()), 
    //     geburtsname: Some("Alt".to_string()),
    //     geburtstag: Some("01.01.2000".to_string()),
    //     todestag: None,
    // };
    // let p: Option<Person> = create_new_person();
    // insert_person(p);
    // let all: Vec<Person> = get_all_persons();
    // print_vector_person(all);
    //print_person(Some(all[0].clone()));
}

/* Usage:
cargo run 
    -p : create a new person -> all 6 parameter
    -r : create a new relation
    -u : update a given thing (only in combination with another mode)
    -s : search a person
    -h : help -> shows this and / or more
    
Examples:
    cargo run -p : new person
    cargo run -p -u : update a person
    cargo run -h : show help
    ...
*/