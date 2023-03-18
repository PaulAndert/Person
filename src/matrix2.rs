// use crate::person::Person;
// use crate::Relation;
// use std::collections::HashMap;

// pub const OLDEST: usize = crate::graph::OLDEST as usize;
// pub const NEWEST: usize = crate::graph::NEWEST as usize;
// const BREIT: usize = 32;
// const TIEF: usize = NEWEST - OLDEST;

// pub fn matrix_to_string() -> String {
    
//     // let mut matrix:[[i32;BREIT];TIEF] = [[-1;BREIT];TIEF]; // create a Matrix full of -1

//     let mut person: Person = crate::person::search(); 
//     while person.person_id == -1 { person = crate::person::search(); }

//     let mut years: HashMap<i32, i32> = HashMap::new();
//     let mut family_structure: Vec<Vec<Person>> = Vec::new();
//     family_structure.push(Vec::new());
//     family_structure[0].push(person.clone());

//     match person.clone().birthday {
//         None => {
//             let year: i32 = crate::person::find_year(person.clone(), years.clone());
//             if year != 0 { years.insert(person.person_id, year); }
//         },
//         Some(bir) => {
//             if !bir.is_empty() {
//                 let mut year: i32 = crate::graph::get_year_from_birthday(bir);
//                 if year != 0 { years.insert(person.person_id, year); }
//                 else {
//                     year = crate::person::find_year(person.clone(), years.clone());
//                     if year != 0 { years.insert(person.person_id, year); }
//                 }
//             }else { 
//                 let year: i32 = crate::person::find_year(person.clone(), years.clone());
//                 if year != 0 { years.insert(person.person_id, year); }
//             }
//         },
//     }
    
//     let generation: usize = 1;
//     (years, family_structure) = get_family_heads(person, years, family_structure, generation);

//     for i in 0..family_structure.len() {
//         println!("Gen {}", i);
//         for j in 0..family_structure[i].len() {
//             println!("{}", crate::person::get_3_names(family_structure[i][j].clone()));
//         }
//     }

//     //let pair_matrix = pair_to_matrix(family_structure[4][0], family_structure[4][1]);



//     //(matrix, years) = person_into_matrix(matrix, person, years);
//     // print_matrix(matrix);
    
//     "".to_string()
// }

// // fn pair_to_matrix(female: Person, male: Person) {
// //     let all_children: Vec<Relation> = crate::person::male_and_female_id_to_relations(male.person_id, female.person_id);

// //     let breite: i32 = 2 + all_children.len();
// //     let matrix

// // }

// fn get_family_heads(person: Person, mut years: HashMap<i32, i32>, mut family_structure: Vec<Vec<Person>>, generation: usize) -> (HashMap<i32, i32>, Vec<Vec<Person>>) {
//     if family_structure.len() <= generation { family_structure.push(Vec::new()) }

//     let relation: Vec<Relation> = crate::db::person_id_to_relations(person.person_id, 1);
//     let mut no_female: bool = false;

//     let mut female: Option<Person> = None;
//     let mut male: Option<Person> = None;
//     if relation.len() > 0 {
//         match relation[0].clone().female {
//             None => { no_female = true },
//             Some(f) => { 
//                 family_structure[generation].push(f.clone()); 
//                 female = Some(f);
//             },
//         }
//         match relation[0].clone().male {
//             None => {
//                 if no_female == true { println!("There are no parents in the relation with {}", person.person_id) }
//                 else {
//                     let mut unkn: Person = Person::new();
//                     let female: Person = family_structure[generation][family_structure[generation].len()-1].clone() ;
//                     unkn.person_id = 0;
//                     unkn.first_name = Some(String::from("Unknown"));
//                     unkn.birthday = female.birthday;
//                     unkn.gender = Some(String::from("um"));
//                     family_structure[generation].push( unkn );
//                 }
//             },
//             Some(m) => {
//                 if no_female == true {
//                     let mut unkn: Person = Person::new();
//                     unkn.person_id = 0;
//                     unkn.first_name = Some(String::from("Unknown"));
//                     unkn.birthday = m.clone().birthday;
//                     unkn.gender = Some(String::from("uf"));
//                     family_structure[generation].push( unkn );
//                 }
//                 family_structure[generation].push(m.clone());
//                 male = Some(m);
//             }
//         }
//     }else {
//         let matrix: [i32, 1] = [person.person_id, 1];
        

//     }


//     // Connect the matrixes
//     let all_children: Vec<Relation>;
//     match female{
//         None => {
//             match male{
//                 None => { //  NO Female And NO Male 
//                     all_children = Vec::new();
//                 },
//                 Some(m) => {  // NO Female And YES Male 
//                     (years, family_structure) = get_family_heads(m.clone(), years, family_structure, generation+1);
//                     all_children = crate::db::male_and_female_id_to_relations(m.person_id, -1);
//                 }
//             }
//         },
//         Some(f) => {
//             (years, family_structure) = get_family_heads(f.clone(), years, family_structure, generation+1);
//             match male{
//                 None => {  // YES Female And NO Male 
//                     all_children = crate::db::male_and_female_id_to_relations(-1, f.person_id);
//                 },
//                 Some(m) => {  // YES Female And YES Male 
//                     (years, family_structure) = get_family_heads(m.clone(), years, family_structure, generation+1);
//                     all_children = crate::db::male_and_female_id_to_relations(m.person_id, f.person_id);
//                 }
//             }
//         }
//     }

//     (years, family_structure)
// }

// // fn person_into_matrix(mut matrix: [[i32;BREIT];TIEF], person: Person, mut years: HashMap<i32, i32>) -> ([[i32;BREIT];TIEF], HashMap<i32, i32>) {

// //     let all_children: Vec<Person> = crate::person::get_all_children(person.clone());
// //     let all_parents: Vec<Person> = crate::person::get_all_parents(person.clone());

// //     // match years.get(&person.person_id) {
// //     //     None => {
// //     //         match person.birthday {
// //     //             None => {
// //     //                 let year = crate::person::find_year(person.clone(), years.clone());
// //     //                 years.insert(person.person_id, year);
// //     //                 matrix[year as usize - OLDEST][BREIT/2] = person.person_id; 
// //     //             }
// //     //             Some(z) => {
// //     //                 let year: i32 = crate::graph::get_year_from_birthday(z);
// //     //                 years.insert(person.person_id, year);
// //     //                 matrix[year as usize - OLDEST][BREIT/2] = person.person_id; 
// //     //             }
// //     //         }
// //     //     },
// //     //     Some(year) => {
// //     //         matrix[*year as usize - OLDEST][BREIT/2] = person.person_id; 
// //     //     }
// //     // }

// //     // child of child

// //     // parent of parent


// //     (matrix, years)
// // }

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