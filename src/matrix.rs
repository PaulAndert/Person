use crate::Person;
use crate::Family;
use crate::db;

// Width of the matrix (the more generations one has the more space it takes)
const WIDTH: usize = 32;
// Height of the matrix (the more generations one has the more space it takes)
const DEPTH: usize = 10;

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
    
    let mut all_familys: Vec<Vec<Family>> = init_all_familys(Vec::new(), person, 0, "");

    all_familys.pop();

    let mut matrix: Matrix = Matrix::new();

    matrix = all_familys_to_matrix(all_familys.clone(), matrix);

    // println!("{}", matrix.to_string());

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
            if family.family_id != -1 {
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
                if male_y == -1.0 {
                    println!("{}", male.two_names());
                }
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
                    if year_to_y(child_year.clone()) == -1.0 {
                        println!("{}", child.two_names());
                    }
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
                if female_y == -1.0 {
                    println!("{}", female.two_names());
                }
                let female_x: f32 = 3.0 * match matrix.data[DEPTH-i].iter().position(|&e| e == female.person_id) {
                    Some(idx) => {(idx) as f32},
                    None => {println!("Error male_x is not in row"); 0.0},
                };
                _ = average_vector(vec![male.clone(), female.clone()]) + average_vector(family.children.clone());
                let middle_y: f32 = translate(((max_vector(vec![male, female]) + min_vector(family.children.clone())) / 2) as i32);

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
    }
    ret
}

fn min_vector(persons: Vec<Person>) -> i32 {
    let mut ret: Vec<i32> = Vec::new();

    for person in persons.clone() {
        let year_string: String  = match person.birthday {
            None => {
                search_birth_year(person.clone())
            },
            Some(z) => {z.format("%Y").to_string()},
        };
        match year_string.parse::<i32>() {
            Ok(year) => {
                ret.push(year);
            },
            Err(_e) => {println!("AV: {}", person.two_names());} 
        };
    }
    match ret.iter().min() {
        None => {0},
        Some(min) => {
            // println!("MIN: {:?} -> {}", persons.iter().map(|p| p.two_names()).collect::<String>(), min);
            *min
        }
    }
}

fn max_vector(persons: Vec<Person>) -> i32 {
    let mut ret: i32 = 0;

    for person in persons.clone() {
        let year_string: String  = match person.birthday {
            None => {
                search_birth_year(person.clone())
            },
            Some(z) => {z.format("%Y").to_string()},
        };
        match year_string.parse::<i32>() {
            Ok(year) => {
                if ret < year { ret = year; }
            },
            Err(_e) => {println!("AV: {}", person.two_names());} 
        };
    }
    // println!("MAX: {:?} -> {}", persons.iter().map(|p| p.two_names()).collect::<String>(), ret);
    ret
}

#[allow(dead_code)]
fn average_vector(persons: Vec<Person>) -> i32 {
    let mut add: i32 = 0;

    for person in persons.clone() {
        let year_string: String  = match person.birthday {
            None => {
                search_birth_year(person.clone())
            },
            Some(z) => {z.format("%Y").to_string()},
        };
        add += match year_string.parse::<i32>() {
            Ok(year) => {year},
            Err(_e) => {println!("AV: {}", person.two_names()); 0} 
        };
    }
    // println!("AVG: {:?} -> {}", persons.iter().map(|p| p.two_names()).collect::<String>(), add / persons.len() as i32);
    add / persons.len() as i32
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
                        if y == -1.0 {
                            println!("{}", person.two_names());
                        }
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
                        let label: String = format!("{}\n{}", person.two_names(), year);
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
        Err(_e) => {println!("YTY: {}", year); 0} 
    };

    if year_i32 != 0 {
        return translate(year_i32);
    }else{
        return -1.0;
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
                    Err(_e) => {println!("POSS1: {}", item);} 
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
                    Err(_e) => {println!("POSS2: {}", possibilitys[0]); String::new() } 
                }
            }else{
                let mut add: i32 = 0;
                for item in possibilitys.clone() {
                    match item.parse::<i32>() {
                        Ok(year) => {add += year},
                        Err(_e) => {println!("POSS3: {}", item);} 
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
                        Err(_e) => {println!("POSS4: {}", possibilitys[0]); String::new() } 
                    }
                }else{
                    let mut add: i32 = 0;
                    for item in possibilitys.clone() {
                        match item.parse::<i32>() {
                            Ok(year) => {add += year},
                            Err(_e) => {println!("POSS5: {}", item);} 
                        }
                    }
                    return (((add / possibilitys.len() as i32) as i32) + 30).to_string();
                }
            }else {
                // other children of parents
                let siblings: Vec<Person> = db::get_siblings_by_person_id(person.person_id);

                for sibling in siblings {

                    match sibling.birthday {
                        None => {},
                        Some(birth) => {
                            possibilitys.push(birth.format("%Y").to_string());
                        },
                    }

                }
                if possibilitys.len() >= 1 {
                    if possibilitys.len() == 1 {
                        return match possibilitys[0].clone().parse::<i32>() {
                            Ok(year) => {(year).to_string()},
                            Err(_e) => {println!("POSS6: {}", possibilitys[0]); String::new() } 
                        }
                    }else{
                        let mut add: i32 = 0;
                        for item in possibilitys.clone() {
                            match item.parse::<i32>() {
                                Ok(year) => {add += year},
                                Err(_e) => {println!("POSS7: {}", item);} 
                            }
                        }
                        return ((add / possibilitys.len() as i32) as i32).to_string();
                    }
                }
            }
        }
    }
    String::new()
}

fn all_familys_to_matrix(all_familys: Vec<Vec<Family>>, mut matrix: Matrix) -> Matrix {
    let last_gen: usize = all_familys.len();
    let mut skip: bool = false;

    for i in 1..=all_familys.len() {

        // println!(":: {} ::\n{}", i, matrix.to_string());

        let mut current_width: usize = 0;
        for j in 0..all_familys[last_gen-i].len() {
            let family: Family = all_familys[last_gen-i][j].clone();
            if family.family_id != -1 {

                let male_id = match family.male {
                    Some(per) => {per.person_id},
                    None => {0},
                };
                let female_id = match family.female {
                    Some(per) => {per.person_id},
                    None => {0},
                };
                let male_pos: usize;
                let female_pos: usize;
                if matrix.data[DEPTH-i].contains(&male_id) {
                    male_pos = match matrix.data[DEPTH-i].iter().position(|&e| e == male_id) {
                        Some(idx) => {idx},
                        None => {println!("ERROR: {} = pos 0 because not found", male_id); 0},
                    };
                }else{ // male missing
                    matrix.data[DEPTH-i][current_width] = male_id;
                    male_pos = current_width;
                }
                if matrix.data[DEPTH-i].contains(&female_id) {
                    female_pos = match matrix.data[DEPTH-i].iter().position(|&e| e == female_id) {
                        Some(idx) => {idx},
                        None => {println!("ERROR: {} = pos 0 because not found", female_id); 0},
                    };
                }else{ // female missing
                    matrix.data[DEPTH-i][current_width + 1 + family.children.len()] = female_id;
                    female_pos = current_width + 1 + family.children.len();
                    current_width += 1;
                }
                let space: usize = (female_pos - 1 - male_pos) as usize;
                if space == family.children.len() { // children passen gut rein
                    let mut start: usize = 1;
                    for child in family.children {
                        matrix.data[DEPTH-(i+1)][male_pos + start] = child.person_id;
                        start += 1;
                        current_width += 1;
                    }
                }else if space > family.children.len() { // oder to much space
                    current_width += 1;
                    if space % 2 == 0 && (space + 1) / 2 >= family.children.len() { // even
                        // not yet tested
                        if space / family.children.len() >= 2 {
                            let mut start: usize = (space / family.children.len()) / 2 as usize;
                            for child in family.children.clone() {
                                matrix.data[DEPTH-(i+1)][(male_pos + start - 1)] = child.person_id;
                                start += 2;
                            } 
                        }else {
                            let mut start: usize = 0;
                            for child in family.children.clone() {
                                matrix.data[DEPTH-(i+1)][(male_pos + start - 1)] = child.person_id;
                                start += 2;
                            }
                        }
                    }else if space % 2 == 1 && space / 2 >= family.children.len() { // odd
                        if (space - 1) / family.children.len() >= 2 {
                            let mut start: usize = (space / family.children.len()) / 2 as usize + 1;
                            for child in family.children.clone() {
                                matrix.data[DEPTH-(i+1)][male_pos + start + 1] = child.person_id;
                                start += 2;
                            } 
                        }else {
                            // not yet tested
                            let mut start: usize = 0;
                            for child in family.children.clone() {
                                matrix.data[DEPTH-(i+1)][(male_pos + start - 1)] = child.person_id;
                                start += 2;
                            }
                        }
                    }else {
                        // not yet tested
                        let mut start: usize = (space - family.children.len()) / 2 as usize;
                        for child in family.children.clone() {
                            matrix.data[DEPTH-(i+1)][(male_pos + start - 1)] = child.person_id;
                            start += 1;
                        }
                    }
                } else { // not enough space
                    println!("ERROR: not enough space for the amount of children");
                }
                current_width += 1;
            }else {
                if skip {
                    current_width += all_familys[last_gen-(i+1)][ (j / 2) as usize ].children.len() + 3;
                }else {
                    current_width += 0;
                }
            }
            skip = !skip;
        }
    }

    println!("{}", matrix.to_string());

    return matrix;
}

fn init_all_familys(mut all_familys: Vec<Vec<Family>>, person: Person, gen: usize, side: &str) -> Vec<Vec<Family>> {
    let all_connected_familys: Vec<Family> = db::get_family_by_child_id(person.person_id);

    if all_connected_familys.len() == 0 {
        if all_familys.len() <= gen {
            all_familys.push(Vec::new());
        }
        all_familys[gen].push(Family::new());

        return all_familys;
    }else{
        for mut family in all_connected_familys.clone() {
            if side == "left" {
                let fm_len = family.children.len() - 1;
                family.children.swap(0, fm_len);
            }
            if all_familys.len() > gen && !all_familys[gen].contains(&family) {
                all_familys[gen].push(family);
            }else {
                all_familys.push(Vec::new());
                all_familys[gen].push(family);
            }
        }
    
        for family in all_connected_familys {
            match family.male {
                None => {  },
                Some(male) => {
                    all_familys = init_all_familys(all_familys, male, gen + 1, "left");
                },
            }
            match family.female {
                None => {  },
                Some(female) => {
                    all_familys = init_all_familys(all_familys, female, gen + 1, "right");
                },
            }
            
        }
    }

    all_familys
}
