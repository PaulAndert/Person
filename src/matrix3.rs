use crate::Person;
use crate::Family;
use crate::db;

// Width of the matrix (the more generations one has the more space it takes)
const WIDTH: usize = 32;
// Height of the matrix (the more generations one has the more space it takes)
const DEPTH: usize = 4;

// oldest year for scaling
pub const OLDEST: f32 = 1850.0;
// newest year for scaling
pub const NEWEST: f32 = 2050.0;
// upper value to scaling to
const SCALE_TO_UPPER: f32 = 30.0;
// lower value to scaling to
const SCALE_TO_LOWER: f32 = 0.0;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix {
    pub data: [[i32;WIDTH];DEPTH],
}

impl Matrix {
    pub fn new () -> Matrix {
        return Matrix{
            data: [[0;WIDTH];DEPTH],
        };
    }
    pub fn to_string(&self) -> String {
        let mut mat: String = String::new();
        mat.push_str("Matrix\n");

        for i in 0..self.data.len() {
            for value in self.data[self.data.len()-(1+i)] {
                if value == 0 {
                    mat.push_str("   |")
                }else if value > 9 {
                    mat.push_str(&format!(" {}|", value))
                }else {
                    mat.push_str(&format!(" {} |", value))
                }
            }
            mat.push_str("\n")
        }
        mat
    }
}

pub fn create_dot_string(id: i32) -> String{
    let mut person: Person;
    if id != -1 {
        match db::get_person_by_id(id) {
            None => {
                person = crate::person::search(); 
                while person.person_id == -1 { 
                    person = crate::person::search(); 
                }
            },
            Some(per) => {person = per},
        }
    } else {
        person = crate::person::search(); 
        while person.person_id == -1 { 
            person = crate::person::search(); 
        }
    }
    
    let all_familys: Vec<Vec<Family>> = init_all_familys(Vec::new(), person, 0);

    let mut matrix: Matrix = Matrix::new();

    matrix = all_familys_to_matrix(all_familys.clone(), matrix);

    //println!("{}", matrix.to_string());

    // for i in 0..all_familys.len() {
    //     println!("I: {}", i);
    //     for j in all_familys[i].clone() {
    //         println!("{}", j.to_string());
    //     }
    // }

    let stri: String = matrix_to_dot_string(matrix, all_familys);

    stri
}

fn matrix_to_dot_string(matrix: Matrix, all_familys: Vec<Vec<Family>>) -> String {
    let persons: String = insert_dot_persons(matrix.clone());
    //println!("{}", persons);
    let connections: String = insert_dot_connections(matrix, all_familys);
    //println!("{}", connections);
    format!("{}\n\n{}", persons, connections)
}

fn insert_dot_connections(matrix: Matrix, all_familys: Vec<Vec<Family>>) -> String {
    let mut ret: String = String::new();
    let last_gen: usize = all_familys.len();

    for i in 1..=all_familys.len() { // all familys von oben durch gehen 
        for family in all_familys[last_gen-i].clone() {
            // println!("{}", family.to_string());
// Male
            let male = match family.male {
                Some(per) => {per},
                None => {Person::new()},
            };
            // println!("{}", male.three_names());
            let male_year: String = match male.birthday {
                None => {
                    search_birth_year(male.clone())
                },
                Some(z) => {z.format("%Y").to_string()},
            };
            let male_y: f32 = year_to_y(male_year.clone());

            // println!("{:?}", matrix.data[DEPTH-i]);
            let male_x: f32 = 3.0 * match matrix.data[DEPTH-i].iter().position(|&e| e == male.person_id) {
                Some(idx) => {(idx) as f32},
                None => {println!("Error male_x is not in row"); 0.0},
            };

// Children
            let mut children_y: Vec<f32> = Vec::new();
            let mut children_x: Vec<f32> = Vec::new();

            for child in family.children.clone() {
                let child_year: String = match child.birthday {
                    None => {
                        search_birth_year(child.clone())
                    },
                    Some(z) => {z.format("%Y").to_string()},
                };
                children_y.push(year_to_y(child_year.clone()));
                children_x.push(3.0 * match matrix.data[DEPTH-(i+1)].iter().position(|&e| e == child.person_id) {
                    Some(idx) => {(idx) as f32},
                    None => {println!("Error male_x is not in row"); 0.0},
                });
            }
// Female
            let female = match family.female {
                Some(per) => {per},
                None => {Person::new()},
            };
            let female_year: String = match female.birthday {
                None => {
                    search_birth_year(female.clone())
                },
                Some(z) => {z.format("%Y").to_string()},
            };
            let female_y: f32 = year_to_y(female_year.clone());
            let female_x: f32 = 3.0 * match matrix.data[DEPTH-i].iter().position(|&e| e == female.person_id) {
                Some(idx) => {(idx) as f32},
                None => {println!("Error male_x is not in row"); 0.0},
            };

            let middle_y: f32 = translate(((average_vector(vec![male, female]) + average_vector(family.children.clone())) / 2.0) as i32);
// insert the lines
// x22_0y_13_75 [shape=circle,label="",height=0.01,width=0.01, pos="22,-13.75!"];
            // println!("MX: {}", male_x);
            ret.push_str(
                &format!("x{}y{} [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"{}, {}!\"];\n", 
                male_x, format!("{:.2}", middle_y).replace("-", "_").replace(".", "_"), male_x, middle_y.to_string())
            );

            for child_idx in 0..children_x.len() {
                ret.push_str(
                    &format!("x{}y{} [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"{}, {}!\"];\n", 
                    children_x[child_idx], format!("{:.2}", middle_y).replace("-", "_").replace(".", "_"), children_x[child_idx], middle_y.to_string())
                );
            }

            ret.push_str(
                &format!("x{}y{} [shape=circle,label=\"\",height=0.01,width=0.01, pos=\"{}, {}!\"];\n", 
                female_x, format!("{:.2}", middle_y).replace("-", "_").replace(".", "_"), female_x, middle_y.to_string())
            );


            let mut conn: String = String::new();
            conn.push_str(&format!("x{}y{} -> ", male_x, format!("{:.2}", male_y).replace("-", "_").replace(".", "_")));

            conn.push_str(&format!("x{}y{} -> ", male_x, format!("{:.2}", middle_y).replace("-", "_").replace(".", "_")));

            for child_idx in 0..children_x.len() {
                conn.push_str(&format!("x{}y{} -> ", children_x[child_idx], format!("{:.2}", middle_y).replace("-", "_").replace(".", "_")));
            }

            conn.push_str(&format!("x{}y{} -> ", female_x, format!("{:.2}", middle_y).replace("-", "_").replace(".", "_")));

            conn.push_str(&format!("x{}y{};\n", female_x, format!("{:.2}", female_y).replace("-", "_").replace(".", "_")));

            ret.push_str(&conn);

            for child_idx in 0..children_x.len() {
                ret.push_str(
                    &format!("x{}y{} -> x{}y{};\n",
                    children_x[child_idx], format!("{:.2}", middle_y).replace("-", "_").replace(".", "_"),
                    children_x[child_idx], format!("{:.2}", children_y[child_idx]).replace("-", "_").replace(".", "_")
                ));
            }
        }
    }
    ret
}

fn average_vector(persons: Vec<Person>) -> f32 {

    let mut add: f32 = 0.0;

    for person in persons.clone() {
        let year_string: String  = match person.birthday {
            None => {
                search_birth_year(person.clone())
            },
            Some(z) => {z.format("%Y").to_string()},
        };
        add += match year_string.parse::<i32>() {
            Ok(year) => {year as f32},
            Err(e) => {println!("{}", e); 0.0} 
        };
    }
    add / (persons.len() as f32)
}

fn insert_dot_persons(matrix: Matrix) -> String {
    // x25_61y_19_25 [shape=square, color="blue",label="Ben Rudi Andert 2004", pos="25.61111,-19.25!"];

    let mut ret: String = String::new();

    for i in 1..=matrix.data.len() {
        for j in 0..matrix.data[matrix.data.len()-i].len() {
            let data: i32 = matrix.data[matrix.data.len()-i][j];
            if data != 0 {
                match db::get_person_by_id(data) {
                    None => {println!("ERROR id {} returns no person", data); Person::new()},
                    Some(person) => {
                        let x: usize = j * 3;
                        let year: String = match person.birthday {
                            None => {
                                search_birth_year(person.clone())
                            },
                            Some(z) => {z.format("%Y").to_string()},
                        };
                        let y: f32 = year_to_y(year.clone());
                        let shape: String; 
                        let color: String;
                        match person.clone().gender {
                            None => {
                                shape = String::from("triangle");
                                color = String::from("gray");
                            },
                            Some(gend) => {
                                if gend == "m" {
                                    shape = String::from("square");
                                    color = String::from("blue");
                                }else if gend == "f" {
                                    shape = String::from("circle");
                                    color = String::from("pink");
                                }else {
                                    shape = String::from("triangle");
                                    color = String::from("gray");
                                }
                            },
                        }
                        let label: String = format!("{}\n{}", person.three_names(), year);
                        ret.push_str(
                        &format!(
                                "x{}y{} [shape={}, color={}, label=\"{}\", pos=\"{}, {}!\"];\n", 
                                x, format!("{:.2}", y).replace("-", "_").replace(".", "_"), shape, color, label, x, y.to_string()
                            )
                        );
                        person
                    },
                };
            }
        }
    }

    ret
}

fn year_to_y(year: String) -> f32 {
    let year_i32 : i32 = match year.parse::<i32>() {
        Ok(year) => {year},
        Err(e) => {println!("{}", e); 0} 
    };

    if year_i32 != 0 {
        return translate(year_i32);
    }else{
        return 0.0;
    }
}

pub fn translate(value: i32) -> f32 {
    let left_span: f32 = NEWEST - OLDEST;
    let right_span: f32 = SCALE_TO_LOWER - SCALE_TO_UPPER;
    let value_scaled: f32 = (value as f32 - OLDEST) / left_span;
    return SCALE_TO_UPPER + (value_scaled * right_span);
}

fn search_birth_year(person: Person) -> String {
    let gender: String = match person.gender {
        None => {String::new()},
        Some(z) => {z},
    };
    let mut possibilitys: Vec<String> = Vec::new();

    // search within marriage partners
    let familys: Vec<Family> = db::get_family_by_parent_id(person.person_id);

    for family in familys.clone() {
        let date: String;
        if gender == "m" {
            match family.female {
                None => { date = String::new(); },
                Some(male) => {
                    match male.birthday {
                        None => { date = String::new(); },
                        Some(birth) => {date = birth.format("%Y").to_string(); },
                    }
                }
            }
        }else if gender == "f" {
            match family.male {
                None => { date = String::new(); },
                Some(female) => {
                    match female.birthday {
                        None => { date = String::new(); },
                        Some(birth) => {date = birth.format("%Y").to_string(); },
                    }
                }
            }
        }else {
            date = String::new();
        }
        if date != String::new() {
            possibilitys.push(date);
        }
    }

    if possibilitys.len() >= 1 {
        if possibilitys.len() == 1 {
            return possibilitys[0].clone();
        }else{
            let mut add: i32 = 0;
            for item in possibilitys.clone() {
                match item.parse::<i32>() {
                    Ok(year) => {add += year},
                    Err(e) => {println!("{}", e);} 
                }
            }
            return ((add / possibilitys.len() as i32) as i32).to_string();
        }
    }else {
        // search within children
        for family in familys {

            for child in family.children {
                match child.birthday {
                    None => {},
                    Some(birth) => {
                        possibilitys.push(birth.format("%Y").to_string());
                    },
                }
            }
        }
        if possibilitys.len() >= 1 {
            if possibilitys.len() == 1 {
                return match possibilitys[0].clone().parse::<i32>() {
                    Ok(year) => {(year - 30).to_string()},
                    Err(e) => {println!("{}", e); String::new() } 
                }
            }else{
                let mut add: i32 = 0;
                for item in possibilitys.clone() {
                    match item.parse::<i32>() {
                        Ok(year) => {add += year},
                        Err(e) => {println!("{}", e);} 
                    }
                }
                return (((add / possibilitys.len() as i32) as i32) - 30).to_string();
            }
        }else{
            // search within parents
            let parents: Vec<Person> = db::get_person_by_child_id(person.person_id);

            for parent in parents {

                match parent.birthday {
                    None => {},
                    Some(birth) => {
                        possibilitys.push(birth.format("%Y").to_string());
                    },
                }

            }
            if possibilitys.len() >= 1 {
                if possibilitys.len() == 1 {
                    return match possibilitys[0].clone().parse::<i32>() {
                        Ok(year) => {(year + 30).to_string()},
                        Err(e) => {println!("{}", e); String::new() } 
                    }
                }else{
                    let mut add: i32 = 0;
                    for item in possibilitys.clone() {
                        match item.parse::<i32>() {
                            Ok(year) => {add += year},
                            Err(e) => {println!("{}", e);} 
                        }
                    }
                    return (((add / possibilitys.len() as i32) as i32) + 30).to_string();
                }
            }
        }
    }
    String::new()
}

fn all_familys_to_matrix(all_familys: Vec<Vec<Family>>, mut matrix: Matrix) -> Matrix {
    let last_gen: usize = all_familys.len();

    for i in 1..=all_familys.len() {
        let mut current_width: usize = 0;
        for family in all_familys[last_gen-i].clone() {
            //println!("{}", family.to_string());

// Male
            let male_id = match family.male {
                Some(per) => {per.person_id},
                None => {0},
            };
            if !matrix.data[DEPTH-i].contains(&male_id) {
                matrix.data[DEPTH-i][current_width] = male_id;
            }
            
            current_width = match matrix.data[DEPTH-i].iter().position(|&e| e == male_id) {
                Some(idx) => {idx + 1},
                None => {current_width + 1},
            };
// Children
            for child in family.children {
                matrix.data[DEPTH-(i+1)][current_width] = child.person_id;
                current_width += 1;
            }
// Female
            let female_id = match family.female {
                Some(per) => {per.person_id},
                None => {0},
            };  
            if !matrix.data[DEPTH-i].contains(&female_id) {
                matrix.data[DEPTH-i][current_width] = female_id;
            }
            current_width += 1;
        }
    }

    return matrix;
}

fn init_all_familys(mut all_familys: Vec<Vec<Family>>, person: Person, gen: usize) -> Vec<Vec<Family>> {
    let all_connected_familys: Vec<Family> = db::get_family_by_child_id(person.person_id);

    for family in all_connected_familys.clone() {
        if all_familys.len() > gen && !all_familys[gen].contains(&family) {
            all_familys[gen].push(family);
        }else {
            all_familys.push(Vec::new());
            all_familys[gen].push(family);
        }
    }

    for family in all_connected_familys {
        match family.male {
            None => {},
            Some(male) => {
                all_familys = init_all_familys(all_familys, male, gen + 1);
            },
        }
        match family.female {
            None => {},
            Some(female) => {
                all_familys = init_all_familys(all_familys, female, gen + 1);
            },
        }
        
    }

    all_familys
}
