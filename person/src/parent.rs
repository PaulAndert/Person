use super::person::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parent {
	pub male_id: i32,
    pub male: Option<Person>,
    pub female_id: i32,
    pub female: Option<Person>,
    pub child_id: i32,
    pub child: Option<Person>,
} 

impl Parent {
    fn new () -> Parent {
        return Parent{
            male_id: -1,
            male: None,
            female_id: -1,
            female: None,
            child_id: -1,
            child: None,
        };
    }
}

pub fn change(mut p: Option<Parent>) -> (Option<Parent>, bool) {
    let mut line: String = String::new();
    println!("for changes type the number you want to change, else press enter");
    let n = std::io::stdin().read_line(&mut line).unwrap();
    if n <= 1 {
        return (p, false);
    } else if n >= 3 {
        line.pop();
        println!("{} is not a valid number", line);
        return (p, true);
    }else{
        match line.chars().next() {
            None => return (p, true),
            Some('1') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.male = search(),
                }
            },
            Some('2') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.female = search(),
                }
            },
            Some('3') => {
                match p{
                    None => {},
                    Some(ref mut z) => z.child = search(),
                }
            },
            Some(_) => {
                line.pop();
                println!("{} is not a valid number", line);
            },
        }
        return (p, true);
    }
}

pub fn create() -> Option<Parent>{
    let mut new_parent: Option<Parent> = Some(Parent::new());

    let mut boo: bool = true;
    while boo{
        println!("\n\nNewly created Parent relationship:");
        print(new_parent.clone());
        (new_parent, boo) = change(new_parent.clone());
    }
    return new_parent;
}

pub fn print(parent: Option<Parent>) {
    let mut per : String = String::new();
    match parent {
        None => per.push_str("No parent relationship found"),
        Some(parent) => {
            
            per.push_str("[1] Father: ");
            match &parent.male {
                None => per.push_str("--"),
                Some(_) => per.push_str(&get_person_names(parent.male)),
            };

            per.push_str("\n[2] Mother: ");
            match &parent.female {
                None => per.push_str("--"),
                Some(_) => per.push_str(&get_person_names(parent.female)),
            };

            per.push_str("\n[3] Child: ");
            match &parent.child {
                None => per.push_str("--"),
                Some(_) => per.push_str(&get_person_names(parent.child)),
            };
        },
    }
    println!("{}", per);
}