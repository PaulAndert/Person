use crate::person::Person;
use crate::Relation;
use std::collections::HashMap;

const BREIT: usize = 16;
const TIEF: usize = 10;


fn print_matrix(matrix: [[i32;BREIT];TIEF]){
    let mut top: String = String::from("    ");
    let mut row_string: String = String::new();
    for (i, row) in matrix.iter().enumerate() {
        row_string.push_str(&i.to_string());
        if i < 10  { row_string.push_str("  |") }
        else { row_string.push_str(" |") }
        for (_j, col) in row.iter().enumerate() {
            row_string.push_str(" ");
            if col >= &0 { 
                row_string.push_str(&col.to_string());
                if col > &9 { row_string.push_str(" |") } // take 2 space
                else { row_string.push_str("  |") } // take 0 space
            }else{
                row_string.push_str("   |")
            }
        }
        row_string.pop();
        row_string.push_str("\n");
    }
    for k in 0..BREIT{
        top.push_str(" ");
        top.push_str(&k.to_string());
        if k < 10  { top.push_str("   ") }
        else { top.push_str("  ") }
    }
    println!("{}", top);
    println!("{}", row_string);
}

pub fn matrix_to_string(max_generation: i32) -> String{
            // [[DT; Breite ]; Tiefe]
    let matrix:[[i32;BREIT];TIEF] = [[-1;BREIT];TIEF];

    // Display value given the indexes
    //println!("{}", my_int_matrix[0][0]);

    let p: Option<Person> = crate::person::search();
    //crate::person::print(p);
    
    let (map, unknown, relation) = restructure_children( reduce_matrix( person_into_matrix( p, matrix, 0, BREIT-1, 0, 0, max_generation) ) );
    Hashmaps_to_dot(map, unknown, relation)
}

fn restructure_children(matrix: [[i32;BREIT];TIEF]) -> (HashMap<i32, f32>, HashMap<i32, f32>, Vec<i32>) { 
    // die chilren in die mitte der eltern stecken 

    let mut tiefste_person = 0;
    for i in 1..=TIEF {
        if tiefste_person == 0 { for j in 0..matrix[TIEF-i].len() { if matrix[TIEF-i][j] != -1 { tiefste_person = TIEF - i; break} } }
        else { break }
    }    
    // count the deepest person
    
    let mut map: HashMap<i32, f32> = HashMap::new();
    //                  Partner Id, x-position
    let mut unknown: HashMap<i32, f32> = HashMap::new();

    let mut relation: Vec<i32> = Vec::new();

    let mut first: i32 = -1;
    let mut dict_first: f32 = -1.0;
    let mut second: i32 = -1;
    let mut dict_second: f32 = -1.0;
    let mut unknown_insert: bool = false;
    for k in 0..tiefste_person {
        for i in 0..matrix[tiefste_person-k].len() {
            if matrix[tiefste_person-k][i] != -1 {
                let num: i32 = matrix[tiefste_person-k][i];

                match map.get(&num) {
                    None => {
                        if first == -1 {
                            relation.push(num);
                            first = i as i32;
                            if num == 0{
                                unknown_insert = true;
                            }else{
                                map.insert(num, i as f32);
                            }
                        }else if second == -1 {
                            relation.push(num);
                            second = i as i32;
                            if num == 0{
                                unknown.insert(matrix[tiefste_person-k][first as usize], i as f32);
                            }else{
                                map.insert(num, i as f32);
                            }
                            if unknown_insert {
                                unknown.insert(matrix[tiefste_person-k][second as usize], i as f32);
                            }
        
                            let parent_distance: i32 = second - first;
        
                            let mut cnt: i32 = 0;
                            for j in first..=second {
                                if matrix[tiefste_person-k-1][j as usize] != -1 { cnt += 1 }
                            }
        
                            let space: f32 = parent_distance as f32 / (cnt+1) as f32;
                            let mut children: f32 = 1.0;
                            for j in first..=second {
                                if matrix[tiefste_person-k-1][j as usize] != -1 { 
                                    relation.push(matrix[tiefste_person-k-1][j as usize]);
                                    map.insert(matrix[tiefste_person-k-1][j as usize], children * space + first as f32);
                                    children += 1.0;
                                }
                            }
                            relation.push(-1);
                            //println!("PD: {} = {} - {}", parent_distance, second, first);
                            first = -1;
                            second = -1;
                        }
                    },
                    Some(z) => {
                        if first == -1 {
                            first = i as i32;
                            dict_first = *z;
                            relation.push(num);
                        }else if second == -1 {
                            relation.push(num);
                            second = i as i32;
                            dict_second = *z;
                            if dict_first == -1.0 {
                                dict_first = first as f32;
                            }
        
                            let parent_distance: f32 = dict_second - dict_first;
        
                            let mut cnt: i32 = 0;
                            for j in first..=second {
                                if matrix[tiefste_person-k-1][j as usize] != -1 { cnt += 1 }
                            }
        
                            let space: f32 = parent_distance / (cnt+1) as f32;
                            let mut children: f32 = 1.0;
                            for j in first..=second {
                                if matrix[tiefste_person-k-1][j as usize] != -1 { 
                                    relation.push(matrix[tiefste_person-k-1][j as usize]);
                                    map.insert(matrix[tiefste_person-k-1][j as usize], children * space + dict_first);
                                    children += 1.0;
                                }
                            }
                            relation.push(-1);
                            //println!("PD: {} = {} - {}", parent_distance, second, first);
                            first = -1;
                            second = -1;
                        }
                    },
                }
            }
        }
    }

    println!("{:?}", map);
    println!("{:?}", unknown);
    println!("{:?}", relation);

    (map, unknown, relation)
}

fn reduce_matrix(matrix: [[i32;BREIT];TIEF]) -> [[i32;BREIT];TIEF] {

    let mut tiefste_person = 0;
    for i in 1..=TIEF {
        if tiefste_person == 0 { for j in 0..matrix[TIEF-i].len() { if matrix[TIEF-i][j] != -1 { tiefste_person = TIEF - i; break} } }
        else { break }
    }
    // count the deepest person

    let mut temp_matrix:[[i32;BREIT];TIEF] = [[-1;BREIT];TIEF];
    let mut temp_j: usize = 0;
    for j in 0..=tiefste_person { if j % 2 == 1 || j == 0 { temp_matrix[temp_j] = matrix[j]; temp_j += 1 } }
    // clear out all useless rows

    let mut kill_column: [bool;BREIT] = [false;BREIT];
    for i in 0..temp_matrix[tiefste_person].len() {
        if temp_matrix[tiefste_person][i] == -1 {
            let mut cnt: i32 = 0;
            for j in 0..=tiefste_person { if temp_matrix[j][i] == -1 { cnt += 1 } } // zählen ob die spalte weg kann
            if cnt == (tiefste_person + 1) as i32 { kill_column[i] = true } // alle die weg können bleiben false
        }   
    }

    let mut ret_matrix:[[i32;BREIT];TIEF] = [[-1;BREIT];TIEF];
    let mut new_i: usize = 0;
    for i in 0..BREIT {
        if !kill_column[i] {
            for j in 0..=tiefste_person { ret_matrix[j][new_i] = temp_matrix[j][i] }
            new_i += 1;
        } 
    }
    // move only the usefull columns ino the returning matrix
    ret_matrix
}

fn Hashmaps_to_dot(mut map: HashMap<i32, f32>, mut unknown: HashMap<i32, f32>, relation: Vec<i32>) -> String {
    let mut ret: String = String::new();

    for (key, value) in map.clone().drain() {
        let p: Vec<Person> = crate::db::id_to_person(key);
        let (year_float, year_string) = crate::graph::get_year(p[0].clone());
        ret.push_str(&crate::graph::graph_node(p[0].clone(), value*2.0, year_float, year_string));
        ret.push_str("\n");
    }
    for (key, value) in unknown.clone().drain() {
        let mut p: Vec<Person> = crate::db::id_to_person(key);
        let (year_float, year_string) = crate::graph::get_year(p[0].clone());
        p[0].first_name = Some(String::from("Unknown"));
        p[0].middle_name = None;
        p[0].surname = Some(String::from(" "));
        p[0].maiden_name = None;
        p[0].gender = match p[0].clone().gender {
            None => {
                println!("{} has no gender", p[0].person_id);
                Some(String::from(" "))
            },
            Some(z) => {
                if z == "f" { Some(String::from("um")) }
                else { Some(String::from("uf")) }
            },
        };
        ret.push_str(&crate::graph::graph_node(p[0].clone(), value*2.0, year_float, year_string));
        ret.push_str("\n");
    }

    let mut one_family: Vec<i32> = Vec::new();
    for i in 0..relation.len(){
        if relation[i] != -1 {
            println!("{}", relation[i]);
            one_family.push(relation[i]);
        } else { 
            println!("One family finished");
            if one_family[0] != 0 && one_family[1] != 0 {
                let first: Vec<Person> = crate::db::id_to_person(one_family[0]);
                let (year_float_first, _) = crate::graph::get_year(first[0].clone());
                let second: Vec<Person> = crate::db::id_to_person(one_family[1]);
                let (year_float_second, _) = crate::graph::get_year(second[0].clone());

                let mut parent_year: f32;
                if year_float_first < year_float_second {
                    parent_year = year_float_first
                }else {
                    parent_year = year_float_second;
                }

                let mut children_year: f32 = 10000.0;
                let mut year_float_child: Vec<f32> = Vec::new();
                for j in 2..one_family.len() {
                    println!("{}", one_family[j]);
                    let child: Vec<Person> = crate::db::id_to_person(one_family[j]);
                    let (year, _) = crate::graph::get_year(child[0].clone());
                    println!("Year: {}", year);
                    year_float_child.push(year);
                    if year < children_year {
                        children_year = year;
                    }
                }

                let edge_höhe: f32 = (parent_year + children_year ) / 2.0; 
                println!("{} = {} + {} / 2", edge_höhe, parent_year, children_year);
                for k in 0..one_family.len() {
                    match map.get(&one_family[k]) {
                        None => {},
                        Some(z) => {
                            println!("Edge: {}", (*z)*2.0);
                            ret.push_str(&crate::graph::graph_edge((*z)*2.0, edge_höhe));
                        },
                    }
                }

                ret.push_str("\n");
                // ret.push_str(&form((x-breit) as f32, f_höhe));
                // ret.push_str(" -> ");
                // ret.push_str(&form((x-breit) as f32, edge_höhe));
                match map.get(&one_family[0]) { 
                    None=>{break}, 
                    Some(z) => {
                        ret.push_str(&crate::graph::form((*z)*2.0, year_float_first));
                        ret.push_str(" -> ");
                        ret.push_str(&crate::graph::form((*z)*2.0, edge_höhe));
                        ret.push_str(" -> ");
                    }
                }
                let mut child_ret: String = String::new();
                for l in 2..one_family.len() {
                    match map.get(&one_family[l]) { 
                        None => {break},
                        Some(z) => {
                            ret.push_str(&crate::graph::form((*z)*2.0, edge_höhe));
                            ret.push_str(" -> ");
                            child_ret.push_str(&crate::graph::form((*z)*2.0, edge_höhe));
                            child_ret.push_str(" -> ");
                            child_ret.push_str(&crate::graph::form((*z)*2.0, year_float_child[l-2]));
                            child_ret.push_str("; ");
                        },
                    }
                }
                match map.get(&one_family[1]) { 
                    None => {break},
                    Some(z) => {
                        ret.push_str(&crate::graph::form((*z)*2.0, edge_höhe));
                        ret.push_str(" -> ");
                        ret.push_str(&crate::graph::form((*z)*2.0, year_float_second));
                        ret.push_str("\n");
                    }
                }
                
                ret.push_str(&child_ret);
                ret.push_str("\n");
                ret.push_str("\n");

                // year_float_child[l-2]
            }else {
                if one_family[0] != 0 {

                }else {

                }
            }
            one_family.clear();
        }
    }


    ret
}

fn person_into_matrix(child: Option<Person>, mut matrix: [[i32;BREIT];TIEF], min_breite: usize, max_breite: usize, tiefe: usize, generation: i32, max_generation: i32) -> [[i32;BREIT];TIEF]{    
    let all_rela: Vec<Relation> = crate::db::person_to_relations(child.clone(), 1);
    let mut has_children: bool = false;
    let mut all_children: Vec<Person> = Vec::new();
    if all_rela.len() > 0{
        match &all_rela[0].male {
            None => {
                matrix[tiefe+1][max_breite] = 0;
            },
            Some(z) => {
                all_children = crate::person::get_all_children(z.clone());
                has_children = true;
                matrix[tiefe+1][max_breite] = z.person_id;
            }
        }
        match &all_rela[0].female {
            None => {
                matrix[tiefe+1][min_breite] = 0;
            },
            Some(z) => {
                if !has_children { all_children = crate::person::get_all_children(z.clone())}
                matrix[tiefe+1][min_breite] = z.person_id;
            }
        }
    }

    //crate::person::print_vector(all_children);

    for i in 0..all_children.len() {
        matrix[tiefe][i+1+min_breite] = all_children[i].person_id;
    }

    if all_rela.len() > 0 && generation + 1 < max_generation{
        //println!("{}: {} - {} ", match all_rela[0].female.clone() { Some(z) => z.person_id, None=>{0},}, min_breite, (max_breite-1)/2);
        matrix = person_into_matrix(all_rela[0].female.clone(), matrix, min_breite, (max_breite-min_breite-1)/2+min_breite, tiefe+2, generation + 1, max_generation);
        
        //println!("{}: {} - {} ", match all_rela[0].male.clone() { Some(z) => z.person_id, None=>{0},}, (max_breite+1)/2, max_breite);
        matrix = person_into_matrix(all_rela[0].male.clone(), matrix, (max_breite-min_breite+1)/2+min_breite, max_breite, tiefe+2, generation + 1, max_generation);
    }
    return matrix;
}