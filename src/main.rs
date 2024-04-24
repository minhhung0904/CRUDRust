use std::io;

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    age: u32,
}

pub struct Students {
    class: Vec<Student>,
}


impl Students {
    fn new() -> Self {
        Self {class: Vec::new()}
    }

    fn add(&mut self, student: Student) {
        self.class.push(student)
    }

    fn view_all(&self) -> Vec<&Student> {
        self.class.iter().collect()
    }

    fn edit(&mut self, index: usize, student: Student) {
        let item = self.class.get_mut(index);
        match item {
            Some(input) => *input = student,
            _ => println!("Please enter true index of student"),
        }
    }

    fn delete(&mut self, index: usize) {
        let item: Option<&Student> = self.class.get(index);
        match item {
            Some(input) => {
                self.class.remove(index);
            },
            None => println!("Please enter true index of student"),
        }
    }
}

mod manager {
    use crate::{get_input, get_int, Student, Students};

    pub fn add_student(students: &mut Students) {
        println!("Enter name of student:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter age of student:");
        let age = match get_int() {
            Some(input) => input,
            None => return,
        };
        let student = Student {name, age};
        students.add(student);
    }

    pub fn view(students: &Students) {
        for student in students.view_all() {
            println!("{:?}", student);
        }
    }

    pub fn edit(students: &mut Students) {
        println!("Enter index of student:");
        let index = match get_int() {
            Some(input) => input,
            None => return,
        };
        println!("Enter new name of student:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter new age of student:");
        let age = match get_int() {
            Some(input) => input,
            None => return,
        };

        let student = Student {name, age};
        students.edit(index.try_into().unwrap(), student)
    }

    pub fn delete(students: &mut Students) {
        println!("Enter index of student:");
        let index = match get_int() {
            Some(input) => input,
            None => return,
        };

        students.delete(index.try_into().unwrap());
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please try again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_int() -> Option<u32> {
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };

    let parse_input: Result<u32, _> = input.parse();
    match parse_input {
        Ok(input) => Some(input),
        Err(_) => {
            println!("Please enter a number");
            None
        },
    }
}

enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

impl Manager {
    fn show() {
        println!("--------------------------");
        println!("Manager Paner");
        println!("1. Add student");
        println!("2. View Student");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("Please enter your choise!");
        println!("-------------------------")
    }

    fn choise(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn main() {
    let mut students = Students::new();
    loop {
        Manager::show();
        let input = get_input().expect("Please enter your data");
        match Manager::choise(&input.as_str()) {
            Some(Manager::AddStudent) => manager::add_student(&mut students),
            Some(Manager::ViewStudent) => manager::view(&students),
            Some(Manager::EditStudent) => manager::edit(&mut students),
            Some(Manager::DeleteStudent) => manager::delete(&mut students),
            None => return,
        }
    }
}
