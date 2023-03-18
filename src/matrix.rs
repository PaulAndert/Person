use crate::Person;
use crate::Family;
use std::collections::HashMap;

// // Width of the matrix (the more generations one has the more space it takes)
// const BREIT: usize = 128;
// // Height of the matrix (the more generations one has the more space it takes)
// const TIEF: usize = 10;

// #[allow(dead_code)]
// pub fn print_matrix(matrix: [[i32;BREIT];TIEF]){ // print the matrix to the console
//     let mut top: String = String::from("    ");
//     let mut ret: String = String::new();
//     for (i, row) in matrix.iter().enumerate() {
//         ret.push_str(&i.to_string());
//         if i < 10  { ret.push_str("  |") }
//         else { ret.push_str(" |") }
//         for (_j, col) in row.iter().enumerate() {
//             ret.push_str(" ");
//             if col >= &0 { 
//                 ret.push_str(&col.to_string());
//                 if col > &9 { ret.push_str(" |") } // take 2 space
//                 else { ret.push_str("  |") } // take 0 space
//             }else{
//                 ret.push_str("   |") // if its a -1 dont write it
//             }
//         }
//         ret.pop(); // delete the last |
//         ret.push_str("\n");
//     }
//     for k in 0..BREIT{ // for the indexes in the first row
//         top.push_str(" ");
//         top.push_str(&k.to_string());
//         if k < 10  { top.push_str("   ") }
//         else { top.push_str("  ") }
//     }
//     println!("{}", top);
//     println!("{}", ret);
// }

pub fn matrix_to_string(max_generation: i32) -> String{
    let mut matrix:[[i32;BREIT];TIEF] = [[-1;BREIT];TIEF]; // create a Matrix full of -1
    let mut years: HashMap<i32, i32> = HashMap::new();

    let mut person: Person = crate::person::search(); // get the starting point
    while person.person_id == -1 { person = crate::person::search(); } // ask what the root person should be
    let mut relation: Vec<i32> = Vec::new();
    
    (matrix, relation, years) = person_into_matrix( person.clone(), matrix, 0, BREIT-1, 0, 1, max_generation, relation, years); // get every person related to p in the matrix

//     //print_matrix(matrix.clone());

//     //println!("{:?}", years);

//     _ = fill_up(years);

//     //println!("{:?}", years);
    
//     let tiefste_person: usize;
//     (matrix, tiefste_person) = test(matrix);
//     if tiefste_person == 0 {
//         if crate::db::person_id_to_relations(person.person_id, 4).len() > 0{
//             let mut map: HashMap<i32, f32> = HashMap::new();
//             let unknown: HashMap<i32, f32> = HashMap::new();
//             map.insert(person.person_id, 0.0);
//             let mut ret: String = crate::graph::graph_node(person.clone(), 0.0);
//             ret.push_str(&format!("
// y1900 [shape=none, fontsize=25, label=\"1900\", pos=\"{x1},{y1900}!\"];
// y2000 [shape=none, fontsize=25, label=\"2000\", pos=\"{x1},{y2000}!\"];
// y0 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"{x2},{y1900}!\"];
// y1 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"{x2},{y2000}!\"];
// y1900 -> y0 [style=dashed, color=\"grey\"] ; y2000 -> y1 [style=dashed, color=\"grey\"]\n\n",
//             x1 = (-10.0 * 2.0 ),
//             x2 = (20.0 * 2.0 ),
//             y1900 = crate::graph::translate(1900),
//             y2000 = crate::graph::translate(2000)));
//             format!("{}\n{}", ret, first_children(person, map, unknown))
//         } else {
//             crate::graph::graph_node(person.clone(), 0.0)
//         } 
//     }else{
//         matrix = reduce_matrix(matrix); // reduce every useless row or column

//         let (map, unknown, relation) = restructure_children(matrix, relation); 
//         // convert the matrix to 2 Hashmaps with person_id and x-positions and a Vector of every connection that needs to be made

//         if crate::db::person_id_to_relations(person.person_id, 4).len() > 0{
//             let ret: String = first_children(person, map.clone(), unknown.clone());
//             format!("{}\n{}", hashmaps_to_dot(map, unknown, relation), ret) // convert the 2 Hashmaps and the Vector to the dot language and return it
//         }else {
//             hashmaps_to_dot(map, unknown, relation)
//         }
//     }
}

// #[allow(dead_code)]
// fn fill_up(mut years: HashMap<i32, i32>) -> HashMap<i32, i32> {
//     for (key, value) in years.clone().drain() {
//         if value == 0 {
//             match crate::db::id_to_person(key) {
//                 None => {},
//                 Some(person) => {
//                     let new_value: i32 = crate::person::find_year(person, years.clone());
//                     if new_value != 0 {
//                         years.insert(key,new_value);
//                     }
//                 }
//             }
//         }
//     }
//     years
// }

// fn first_children(person: Person, map: HashMap<i32, f32>, unknown: HashMap<i32, f32>) -> String {
//     let mut gender: i32 = 0;
//     match person.clone().gender { None => {}, Some(z) => if z == "m" { gender = 1 } else { gender = 2} }
//     let all_relations: Vec<Relation> = crate::db::person_id_to_relations(person.person_id, 4);
//     let mut all_partner: Vec<Person> = Vec::new();
//     // get all partners from the relations
//     for i in 0..all_relations.len() {
//         if gender == 1 {
//             match all_relations[i].clone().female {
//                 None => {},
//                 Some(z) => { if !all_partner.contains(&z) { all_partner.push(z) } }
//             }
//         } 
//         if gender == 2 {
//             match all_relations[i].clone().male {
//                 None => {},
//                 Some(z) => { if !all_partner.contains(&z) { all_partner.push(z) } }
//             }
//         }
//     }

//     if all_partner.len() == 1 {
//         let mut all_children: Vec<Person> = Vec::new();
//         for i in 0..all_relations.len() {
//             match all_relations[i].clone().child {
//                 None => {},
//                 Some(z) => {
//                     if !all_children.contains(&z) { all_children.push(z); }
//                 }
//             }
//         }
//         one_partner_to_string(person, gender, all_partner[0].clone(), all_children, map, unknown, 0.0)

//     }else if all_partner.len() > 0 { // if more partners
//         let mut ret: String = String::new();
//         let mut all_children_of_0: Vec<Person> = Vec::new();
//         let mut all_children_of_1: Vec<Person> = Vec::new();
//         // get all children of partner 0 and 1 
//         for i in 0..all_relations.len() {
//             if gender == 1 {
//                 match all_relations[i].clone().female {
//                     None => {},
//                     Some(z) => { 
//                         if all_partner[0] == z{
//                             match all_relations[i].clone().child {
//                                 None => {},
//                                 Some(child) => {
//                                     if !all_children_of_0.contains(&child) {
//                                         all_children_of_0.push(child);
//                                     }
//                                 }
//                             }
//                         } 
//                         if all_partner[1] == z{
//                             match all_relations[i].clone().child {
//                                 None => {},
//                                 Some(child) => {
//                                     if !all_children_of_1.contains(&child) {
//                                         all_children_of_1.push(child);
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             } 
//             if gender == 2 {
//                 match all_relations[i].clone().male {
//                     None => {},
//                     Some(z) => { 
//                         if all_partner[0] == z{
//                             match all_relations[i].clone().child {
//                                 None => {},
//                                 Some(child) => {
//                                     if !all_children_of_0.contains(&child) {
//                                         all_children_of_0.push(child);
//                                     }
//                                 }
//                             }
//                         } 
//                         if all_partner[1] == z{
//                             match all_relations[i].clone().child {
//                                 None => {},
//                                 Some(child) => {
//                                     if !all_children_of_1.contains(&child) {
//                                         all_children_of_1.push(child);
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
        
//         let mut deepest_parent: f32 = 1000.0;
//         let mut highest_child: f32 = -1000.0;

//         let (year_p, _) = crate::graph::get_year(person.clone());
//         if year_p < deepest_parent { deepest_parent = year_p }
//         let (year_0, _) = crate::graph::get_year(all_partner[0].clone());
//         if year_0 < deepest_parent { deepest_parent = year_0 }
//         let (year_1, _) = crate::graph::get_year(all_partner[1].clone());
//         if year_1 < deepest_parent { deepest_parent = year_1 }

//         for i in 0..all_children_of_0.len() {
//             let (year_c, _) = crate::graph::get_year(all_children_of_0[i].clone());
//             if year_c > highest_child { highest_child = year_c }
//         }
//         for i in 0..all_children_of_1.len() {
//             let (year_c, _) = crate::graph::get_year(all_children_of_1[i].clone());
//             if year_c > highest_child { highest_child = year_c }
//         }

//         let own_edge_höhe: f32 = (deepest_parent + highest_child ) / 2.0; 
        
//         ret.push_str(&one_partner_to_string(person.clone(), gender, all_partner[0].clone(), all_children_of_0, map.clone(), unknown.clone(), own_edge_höhe));
//         ret.push_str(&one_partner_to_string(person, if gender == 1 { 0 } else { 1 }, all_partner[1].clone(), all_children_of_1, map, unknown, own_edge_höhe));
        
//         ret
//     }else {
//         let mut all_children: Vec<Person> = Vec::new();
//         for i in 0..all_relations.len() {
//             match all_relations[i].clone().child {
//                 None => {},
//                 Some(z) => {
//                     if !all_children.contains(&z) { all_children.push(z); }
//                 }
//             }
//         }
//         let mut unk = person.clone();
//         unk.first_name = Some(String::from("Unknown"));
//         unk.middle_name = None;
//         unk.surname = None;
//         unk.maiden_name = None;
//         if gender == 1 {
//             unk.gender = Some(String::from("uf"));
//         }else {
//             unk.gender = Some(String::from("um"));
//         }
//         one_partner_to_string(person, gender, unk, all_children, map, unknown, 0.0)
//     }
// }

// fn one_partner_to_string(person: Person, gender: i32, partner: Person, all_children: Vec<Person>, mut map: HashMap<i32, f32>, unknown: HashMap<i32, f32>, own_edge_höhe: f32) -> String {
//     let mut ret: String = String::new();
//     let parent_relation: Vec<Relation> = crate::db::person_id_to_relations(person.person_id, 1);
//     let mut parent_x: f32 = -1.0;
//     // get the x position of the mother / father of person so that the partner can be under them 
//     if parent_relation.len() > 0 {
//         if gender == 1 {
//             match parent_relation[0].clone().female {
//                 None => {
//                     parent_x = 0.0;
//                 },
//                 Some(z) => {
//                     match map.get(&z.person_id) {
//                         None => {
//                             parent_x = 0.0;
//                         },
//                         Some(x) => {
//                             parent_x = *x;
//                         }
//                     }
//                 }
//             }
//         }
//         if gender == 2 {
//             match parent_relation[0].clone().male {
//                 None => {
//                     parent_x = 0.0;
//                 },
//                 Some(z) => {
//                     match map.get(&z.person_id) {
//                         None => {
//                             parent_x = BREIT as f32;
//                         },
//                         Some(x) => {
//                             parent_x = *x;
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     // get persons x position and change the parent x if the amount of new children is to high
//     let mut person_x: f32 = -1.0;
//     match map.get(&person.person_id){
//         None => {},
//         Some(z) => {
//             person_x = *z;
//             if gender == 1 {
//                 if *z - (all_children.len() as f32) < parent_x {
//                     parent_x = *z - 1.0 - all_children.len() as f32;
//                 }else {
//                     parent_x -= 1.0;
//                 }
//             }else if gender == 2 {
//                 if *z + (all_children.len() as f32) > parent_x {
//                     parent_x = *z + 1.0 + all_children.len() as f32;
//                 }else {
//                     parent_x += 1.0;
//                 }
//             }
//         }
//     }
//     // calculate the distance between parents and the distance between children
//     let parent_distance: f32 = (parent_x - person_x).abs();
//     let start: f32;
//     let sector: f32;
//     if all_children.len() == 1 {
//         start = parent_distance / 2.0;
//         sector = 0.0;
//     }else if parent_distance - 1.0 - (all_children.len() - 1) as f32 == 0.0 {
//         start = 0.5;
//         sector = 1.0;
//     }else {
//         start = parent_distance / (all_children.len() as f32 + 1.0);
//         sector = start;
//     }
//     let mut children: f32 = 0.0;
//     let first: f32 = if parent_x < person_x { parent_x } else { person_x };
//     let mut one_family: Vec<i32> = Vec::new();
//     one_family.push(person.person_id);
//     if person.person_id == partner.person_id {
//         map.insert(-1, parent_x);
//         one_family.push(-1);
//     }else {
//         map.insert(partner.person_id, parent_x);
//         one_family.push(partner.person_id);
//     }
//     ret.push_str(&crate::graph::graph_node(partner.clone(), parent_x*2.0));
//     for j in 0..all_children.len() {

//         if children == 0.0 {
//             map.insert(all_children[j].person_id, start + first); // and to the hashmap  
//             one_family.push(all_children[j].person_id);
//             ret.push_str(&crate::graph::graph_node(all_children[j].clone(), (start + first ) *2.0));
//         }else {
//             map.insert(all_children[j].person_id, start + children*sector + first); // and to the hashmap
//             one_family.push(all_children[j].person_id);
//             ret.push_str(&crate::graph::graph_node(all_children[j].clone(), (start + children*sector + first) *2.0));
//         }
//         children += 1.0;
//     }

//     let (year_float_first, _) = crate::graph::get_year(person.clone());
//     let (year_float_second, _) = crate::graph::get_year(partner.clone());
//     ret.push_str(&get_string_of_one_family(one_family, year_float_first, year_float_second, map, unknown, own_edge_höhe));
//     ret
// }

// fn test(matrix: [[i32;BREIT];TIEF]) -> ([[i32;BREIT];TIEF], usize) {
//     let mut tiefste_person: usize = 0;
//     for i in 1..=TIEF { // row count of the deepest person
//         if tiefste_person == 0 { for j in 0..matrix[TIEF-i].len() { if matrix[TIEF-i][j] != -1 { tiefste_person = TIEF - i; break} } }
//         else { break }
//     } 

//     let mut temp_matrix:[[i32;BREIT];TIEF] = [[-1;BREIT];TIEF];
//     temp_matrix[tiefste_person] = matrix[tiefste_person];
//     let mut temp_j: usize = 1;
//     for i in 1..=tiefste_person {
//         //println!("{:?}", matrix[tiefste_person-i]);
//         for j in 0..matrix[tiefste_person-i].len() - temp_j {
//             temp_matrix[tiefste_person-i][temp_j+j] = matrix[tiefste_person-i][j];
//         }
//         temp_j += 1;
//     }
//     (temp_matrix, tiefste_person)
// }

// fn restructure_children(matrix: [[i32;BREIT];TIEF], relation: Vec<i32>) -> (HashMap<i32, f32>, HashMap<i32, f32>, Vec<i32>) { 

//     let mut tiefste_person = 0;
//     for i in 1..=TIEF { // row count of the deepest person
//         if tiefste_person == 0 { for j in 0..matrix[TIEF-i].len() { if matrix[TIEF-i][j] != -1 { tiefste_person = TIEF - i; break} } }
//         else { break }
//     }    
    
//     let mut map: HashMap<i32, f32> = HashMap::new(); // normal person_id ; x-position
//     let mut unknown: HashMap<i32, f32> = HashMap::new(); // partners person_id ; x-position
//     //let mut relation: Vec<i32> = Vec::new(); // person_id`s with -1 as seperators

//     let mut first: i32 = -1;
//     let mut dict_first: f32 = -1.0;
//     let mut second: i32 = -1;
//     let mut dict_second: f32;
//     let mut unknown_insert: bool = false;

//     // walking the matrix from the bottom and the two persons next to each other (partners)
//     for k in 0..tiefste_person {
//         for i in 0..matrix[tiefste_person-k].len() {
//             if matrix[tiefste_person-k][i] != -1 {
//                 let person_id: i32 = matrix[tiefste_person-k][i]; // id of person from the matrix
//                 let index = relation.iter().position(|&r| r == person_id);
//                 match index {
//                     None => {},
//                     Some(ind) => {
//                         if ind == 0 || ind == 1 || relation[ind-1 as usize] == -1 || relation[ind-2 as usize] == -1{
//                             match map.get(&person_id) {
//                                 None => {                       // if the person is not in the map
//                                     if first == -1 {            // if there is no first partner saved 
//                                         //relation.push(person_id);
//                                         first = i as i32;       // save the matrix position 
//                                         if person_id == 0 { unknown_insert = true; } // if id = 0, raise flag so it can later be addet to the unknown hashmap
//                                         else{ map.insert(person_id, i as f32); }
//                                     }else if second == -1 {     // if there is a first partner saved but no second
//                                         //relation.push(person_id);
//                                         second = i as i32;      // save the matrix position 
//                                         // if id = 0, add id of partner and matrix position to unknown hashmap
//                                         if person_id == 0 { unknown.insert(matrix[tiefste_person-k][first as usize], second as f32); }
//                                         else{ map.insert(person_id, i as f32); }
//                                         if unknown_insert {     // if the flag is risen add id of partner of first and matrix position to unknown hashmap
//                                             unknown.insert(matrix[tiefste_person-k][second as usize], first as f32);
//                                             unknown_insert = false; // remove flag
//                                         }
                                        
//                                         let parent_distance: f32 = (second - first) as f32; // calculate the distance of both partners
//                                         let mut cnt: i32 = 0; // count the number of cildren in the matrix by going a step down and counting in the parent_distance
//                                         for j in first..=second { if matrix[tiefste_person-k-1][j as usize] != -1 { cnt += 1; } }

//                                         let start: f32;
//                                         let sector: f32;
//                                         if cnt == 1 {
//                                             start = parent_distance / 2.0;
//                                             sector = 0.0;
//                                         }else if parent_distance - 1.0 - (cnt - 1) as f32 == 0.0 {
//                                             start = 0.5;
//                                             sector = 1.0;
//                                         }else {
//                                             start = parent_distance / (cnt as f32 + 1.0);
//                                             sector = start;
//                                         }
//                                         let mut children: f32 = 0.0;
//                                         //println!("{} -- {} ({}); Kids: {} Start: {} Sector: {}", first, second, parent_distance, cnt, start, sector);

//                                         for j in first..=second {
//                                             if matrix[tiefste_person-k-1][j as usize] != -1 { // if there is a child
//                                                 //relation.push(matrix[tiefste_person-k-1][j as usize]); // push it to the vector
//                                                 if children == 0.0 {
//                                                     //println!("{} -> {} + {}", matrix[tiefste_person-k-1][j as usize], dist, first as f32);
//                                                     map.insert(matrix[tiefste_person-k-1][j as usize], start + first as f32); // and to the hashmap  
//                                                 }else {
//                                                     //println!("{} -> {} + {} + {}", matrix[tiefste_person-k-1][j as usize], dist, children/2.0, first as f32);
//                                                     map.insert(matrix[tiefste_person-k-1][j as usize], start + children*sector + first as f32); // and to the hashmap
//                                                 }
//                                                 children += 1.0;
//                                             }
//                                         }
//                                         //relation.push(-1); // push the seperator because on family is over
//                                         first = -1; // reset both partner
//                                         second = -1;
//                                     }
//                                 },
//                                 Some(z) => { 
//                                 // if the person is already in the map its the same as in None exept the now parents x-positions are used not the matrix positions
//                                     if first == -1 {
//                                         first = i as i32;
//                                         dict_first = *z;
//                                         //relation.push(person_id);
//                                     }else if second == -1 {
//                                         //relation.push(person_id);
//                                         second = i as i32;
//                                         dict_second = *z;
//                                         if dict_first == -1.0 { dict_first = first as f32; }
                    
//                                         let parent_distance: f32 = dict_second - dict_first;
//                                         let mut cnt: i32 = 0;
//                                         for j in first..=second { if matrix[tiefste_person-k-1][j as usize] != -1 { cnt += 1; } }
                    
//                                         let start: f32;
//                                         let sector: f32;
//                                         if cnt == 1 {
//                                             start = parent_distance / 2.0;
//                                             sector = 0.0;
//                                         }else if parent_distance - 1.0 - (cnt - 1) as f32 == 0.0 {
//                                             start = 0.5;
//                                             sector = 1.0;
//                                         }else {
//                                             start = parent_distance / (cnt as f32 + 1.0);
//                                             sector = start;
//                                         }
//                                         let mut children: f32 = 0.0;
//                                         //println!("{} -- {} ({}); kids: {} Start: {} Sector: {}", dict_first, dict_second, parent_distance, cnt, start, sector);

//                                         for j in first..=second {
//                                             if matrix[tiefste_person-k-1][j as usize] != -1 { // if there is a child
//                                                 //relation.push(matrix[tiefste_person-k-1][j as usize]); // push it to the vector
//                                                 if children == 0.0 {
//                                                     //println!("{} -> {} + {}", matrix[tiefste_person-k-1][j as usize], dist, first as f32);
//                                                     map.insert(matrix[tiefste_person-k-1][j as usize], start + dict_first); // and to the hashmap  
//                                                 }else {
//                                                     //println!("{} -> {} + {} + {}", matrix[tiefste_person-k-1][j as usize], dist, children/2.0, first as f32);
//                                                     map.insert(matrix[tiefste_person-k-1][j as usize], start + children*sector + dict_first); // and to the hashmap
//                                                 }
//                                                 children += 1.0;
//                                             }
//                                         }
//                                         //relation.push(-1);
//                                         first = -1;
//                                         second = -1;
//                                     }
//                                 },
//                             }
//                         }
//                     },
//                 }
//             }
//         }
//     }
//     //println!("{:?}", map);
//     //println!("{:?}", unknown);
//     //println!("{:?}", relation);
//     (map, unknown, relation)
// }

// fn reduce_matrix(matrix: [[i32;BREIT];TIEF]) -> [[i32;BREIT];TIEF] {

//     let mut tiefste_person = 0;
//     for i in 1..=TIEF { // row count of the deepest person
//         if tiefste_person == 0 { for j in 0..matrix[TIEF-i].len() { if matrix[TIEF-i][j] != -1 { tiefste_person = TIEF - i; break} } }
//         else { break }
//     }

//     let mut temp_matrix:[[i32;BREIT];TIEF] = [[-1;BREIT];TIEF];
//     let mut temp_j: usize = 0;
//     for j in 0..=tiefste_person { 
//         if j % 2 == 1 || j == 0 { 

//             temp_matrix[temp_j] = matrix[j];
//             temp_j += 1 ;
//             // if j + 1 <= tiefste_person {
//             //     for i in 0..matrix[].len() {

//             //     }
//             // }
//         } 
//     }
//     // clear out all useless rows = all straigt

//     //matrix = temp_matrix;

//     let mut kill_column: [bool;BREIT] = [false;BREIT];
//     for i in 0..matrix[tiefste_person].len() {
//         if matrix[tiefste_person][i] == -1 {
//             let mut cnt: i32 = 0;
//             for j in 0..=tiefste_person { if matrix[j][i] == -1 { cnt += 1 } } // zählen ob die spalte weg kann
//             if cnt == (tiefste_person + 1) as i32 { kill_column[i] = true } // alle die weg können bleiben false
//         }   
//     }

//     let mut ret_matrix:[[i32;BREIT];TIEF] = [[-1;BREIT];TIEF];
//     let mut new_i: usize = 0;
//     for i in 0..BREIT {
//         if !kill_column[i] {
//             for j in 0..=tiefste_person { ret_matrix[j][new_i] = matrix[j][i] }
//             new_i += 1;
//         } 
//     }
//     // move only the usefull columns ino the returning matrix
//     ret_matrix
// }

// fn hashmaps_to_dot(map: HashMap<i32, f32>, unknown: HashMap<i32, f32>, relation: Vec<i32>) -> String {
//     let mut ret: String = String::new();

//     //println!("{:?}", relation.clone());
//     let mut biggest: f32 = -1000.0;
//     let mut smallest: f32 = 1000.0;

//     for (key, value) in map.clone().drain() {
//         if value > biggest { biggest = value }
//         if value < smallest { smallest = value }
//         match crate::db::id_to_person(key) {
//             None => { println!("There is no person with the id: {}", key); },
//             Some(z) => {
//                 ret.push_str(&crate::graph::graph_node(z, value*2.0));
//                 ret.push_str("\n");
//             },
//         }
//     }
//     for (key, value) in unknown.clone().drain() {
//         if value > biggest { biggest = value }
//         if value < smallest { smallest = value }
//         match crate::db::id_to_person(key) {
//             None => { println!("There is no person with the id: {}", key); },
//             Some(mut z) => {
//                 z.first_name = Some(String::from("Unknown"));
//                 z.middle_name = None;
//                 z.surname = Some(String::from(" "));
//                 z.maiden_name = None;
//                 z.gender = match z.clone().gender {
//                     None => {
//                         println!("{} has no gender", z.person_id);
//                         Some(String::from(" "))
//                     },
//                     Some(gender) => {
//                         if gender == "f" { Some(String::from("um")) }
//                         else { Some(String::from("uf")) }
//                     },
//                 };
//                 ret.push_str(&crate::graph::graph_node(z, value*2.0));
//                 ret.push_str("\n");
//             }
//         }
//     }
//     ret.push_str(&format!("
// y1900 [shape=none, fontsize=25, label=\"1900\", pos=\"{x1},{y1900}!\"];
// y2000 [shape=none, fontsize=25, label=\"2000\", pos=\"{x1},{y2000}!\"];
// y0 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"{x2},{y1900}!\"];
// y1 [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"{x2},{y2000}!\"];
// y1900 -> y0 [style=dashed, color=\"grey\"] ; y2000 -> y1 [style=dashed, color=\"grey\"]\n\n",
//     x1 = (smallest * 2.0 ) - 8.0,
//     x2 = (biggest * 2.0 ) + 8.0,
//     y1900 = crate::graph::translate(1900),
//     y2000 = crate::graph::translate(2000)));

//     let mut one_family: Vec<i32> = Vec::new();
//     for i in 0..relation.len(){
//         // if we encounter a real id we write it in the one_family vector
//         if relation[i] != -1 { one_family.push(relation[i]) } 
//         else if one_family.len() > 0 {
//             let mut year_float_first: f32 = 0.0;
//             let mut year_float_second: f32 = 0.0;
//             // get the first and second element from one_family and get the year in a transformed float 
//             // if a partner is 0, we take the year from the other partner
//             let mut first_is_null: bool = false;
//             if one_family[0] != 0 {
//                 match crate::db::id_to_person( one_family[0] ) {
//                     None => { println!("There is no person with the id: {}", one_family[0] ) },
//                     Some(z) => { (year_float_first, _) = crate::graph::get_year( z ) },
//                 }
//             }else { first_is_null = true }
//             if one_family[1] != 0 {
//                 match crate::db::id_to_person( one_family[1] ) {
//                     None => { println!("There is no person with the id: {}", one_family[1] ) },
//                     Some(z) => { (year_float_second, _) = crate::graph::get_year( z ) },
//                 }
//                 if first_is_null { year_float_first = year_float_second.clone() }
//             }else { year_float_second = year_float_first.clone() }

//             ret.push_str(&get_string_of_one_family(one_family.clone(), year_float_first, year_float_second, map.clone(), unknown.clone(), 0.0));
//             one_family.clear();
//         }
//     }
//     ret
// }

// fn get_string_of_one_family(one_family: Vec<i32>, year_float_first: f32, year_float_second: f32, map: HashMap<i32, f32>, unknown: HashMap<i32, f32>, mut edge_höhe: f32) -> String {
//     let mut ret: String = String::new();
//     let parent_year: f32;
//     if year_float_first < year_float_second { parent_year = year_float_first }
//     else { parent_year = year_float_second }

//     let mut children_year: f32 = -10000.0;
//     let mut year_float_child: Vec<f32> = Vec::new();
//     for j in 2..one_family.len() {
//         match crate::db::id_to_person(one_family[j]) {
//             None => { println!("There is no person with the id: {}", one_family[j] ); },
//             Some(z) => {
//                 let (year, _) = crate::graph::get_year(z);
//                 year_float_child.push(year);
//                 if year > children_year {
//                     children_year = year;
//                 }
//             },
//         }
//     }

//     if edge_höhe == 0.0 {
//         edge_höhe = (parent_year + children_year ) / 2.0; 
//     }
//     let mut first_x: f32 = 0.0;
//     if one_family[0] == 0 {
//         match unknown.get(&one_family[1]) {
//             None => {},
//             Some(z) => {
//                 first_x = (*z)*2.0;
//             },
//         }
//     }else {
//         match map.get(&one_family[0]) {
//             None => {},
//             Some(z) => {
//                 first_x = (*z)*2.0;
//             },
//         }
//     }
//     let mut second_x: f32 = 0.0;
//     if one_family[1] == 0 {
//         match unknown.get(&one_family[0]) {
//             None => {},
//             Some(z) => {
//                 second_x = (*z)*2.0;
//             },
//         }
//     }else {
//         match map.get(&one_family[1]) {
//             None => {},
//             Some(z) => {
//                 second_x = (*z)*2.0;
//             },
//         }
//     }
//     ret.push_str(&crate::graph::graph_edge(first_x, edge_höhe));
//     ret.push_str(&crate::graph::graph_edge(second_x, edge_höhe));
//     for k in 2..one_family.len() {
//         match map.get(&one_family[k]) {
//             None => {},
//             Some(z) => {
//                 ret.push_str(&crate::graph::graph_edge((*z)*2.0, edge_höhe));
//             },
//         }
//     }

//     ret.push_str("\n");
//     ret.push_str(&crate::graph::form(first_x, year_float_first));
//     ret.push_str(" -> ");
//     ret.push_str(&crate::graph::form(first_x, edge_höhe));
//     ret.push_str(" -> ");

//     let mut child_ret: String = String::new();
//     for l in 2..one_family.len() {
//         match map.get(&one_family[l]) { 
//             None => {break},
//             Some(z) => {
//                 ret.push_str(&crate::graph::form((*z)*2.0, edge_höhe));
//                 ret.push_str(" -> ");
//                 child_ret.push_str(&crate::graph::form((*z)*2.0, edge_höhe));
//                 child_ret.push_str(" -> ");
//                 child_ret.push_str(&crate::graph::form((*z)*2.0, year_float_child[l-2]));
//                 child_ret.push_str("; ");
//             },
//         }
//     }
//     ret.push_str(&crate::graph::form(second_x, edge_höhe));
//     ret.push_str(" -> ");
//     ret.push_str(&crate::graph::form(second_x, year_float_second));
//     ret.push_str("\n");

//     ret.push_str(&child_ret);
//     ret.push_str("\n");
//     ret.push_str("\n");

//     ret
// }

fn person_into_matrix(child: Person, mut matrix: [[i32;BREIT];TIEF], min_breite: usize, max_breite: usize, tiefe: usize, generation: i32, max_generation: i32, mut relation: Vec<i32>, mut years: HashMap<i32, i32>) -> ([[i32;BREIT];TIEF], Vec<i32>, HashMap<i32, i32>) {    
    
    let all_rela: Vec<Relation> = crate::db::person_id_to_relations(child.person_id, 1);
    let mut has_children: bool = false;
    let mut all_children: Vec<Person> = Vec::new();
    if all_rela.len() > 0{
        match &all_rela[0].male {
            None => {
                matrix[tiefe+1][(max_breite-min_breite+1)/2+min_breite] = 0;
                relation.push(0);
            },
            Some(z) => {
                all_children = crate::person::get_all_children(z.clone());
                has_children = true;
                matrix[tiefe+1][(max_breite-min_breite+1)/2+min_breite] = z.person_id;
                relation.push(z.person_id);
                match z.clone().birthday {
                    None => {},
                    Some(bir) => { if !bir.is_empty() { years.insert(z.person_id, crate::graph::get_year_from_birthday(bir) ); } },
                }
            }
        }
        match &all_rela[0].female {
            None => {
                matrix[tiefe+1][min_breite] = 0;
                relation.push(0);
            },
            Some(z) => {
                if !has_children { all_children = crate::person::get_all_children(z.clone())}
                matrix[tiefe+1][min_breite] = z.person_id;
                relation.push(z.person_id);
                match z.clone().birthday {
                    None => {},
                    Some(bir) => { if !bir.is_empty() { years.insert(z.person_id, crate::graph::get_year_from_birthday(bir) ); } },
                }
            }
        }
    }else if generation == 1 {
        matrix[tiefe][BREIT/2] = child.person_id;
        relation.push(child.person_id);
        match child.clone().birthday {
            None => {},
            Some(bir) => { if !bir.is_empty() { years.insert(child.person_id, crate::graph::get_year_from_birthday(bir) ); } },
        }
    }
    // TODO hier wenn gender m or f dann unterschiedlich annordnen so das grapf besser assieht
    for i in 0..all_children.len() {
        matrix[tiefe][i+min_breite] = all_children[i].person_id;
        relation.push(all_children[i].person_id);
        match all_children[i].clone().birthday {
            None => {},
            Some(bir) => { if !bir.is_empty() { years.insert(all_children[i].person_id, crate::graph::get_year_from_birthday(bir) ); } },
        }
    }
    relation.push(-1);

    if all_rela.len() > 0 && generation + 1 <= max_generation{
        match all_rela[0].female.clone() {
            None => {},
            Some(z) => { (matrix, relation, years) = person_into_matrix(z, matrix, min_breite, (max_breite-min_breite-1)/2+min_breite, tiefe+1, generation + 1, max_generation, relation.clone(), years) },
        }
        match all_rela[0].male.clone() {
            None => {},
            Some(z) => { (matrix, relation, years) = person_into_matrix(z, matrix, (max_breite-min_breite+1)/2+min_breite, max_breite, tiefe+1, generation + 1, max_generation, relation.clone(), years) },
        }
    }
    
    (matrix, relation, years)
}