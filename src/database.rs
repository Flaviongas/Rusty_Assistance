use colored::*;
use std::io;

#[derive(Debug, Clone)]
pub struct Student {
    pub ID: u8,
    pub Rut: u32,
    pub Nombres: String,
    pub Apellido1: String,
    pub Apellido2: String,
    pub DV: u8,
}

pub fn get_student(name: String) -> Student {
    let formatted_name = name.trim();
    let pattern = format!("{}%", formatted_name);
    let connection = sqlite::open("alumnos.db").unwrap();

    let query = "SELECT * FROM alumnos WHERE LOWER(Nombres) like LOWER(?)";
    let mut statement = connection.prepare(query).unwrap();
    statement
        .bind((1, &pattern[..]))
        .expect("Error binding value");

    let mut students: Vec<Student> = Vec::new();
    while let Ok(sqlite::State::Row) = statement.next() {
        let column_n = statement.column_count();
        let mut studentVector: Vec<String> = vec![];
        for i in 0..column_n {
            let value: Option<String> = statement.read(i).ok();
            studentVector.push(value.unwrap());
        }
        let currentStudent = Student {
            ID: studentVector[0].parse().unwrap(),
            Rut: studentVector[1].parse().unwrap(),
            Nombres: studentVector[2].clone(),
            Apellido1: studentVector[3].clone(),
            Apellido2: studentVector[4].clone(),
            DV: studentVector[5].parse().unwrap(),
        };
        students.push(currentStudent);
    }
    if students.len() > 1 {
        println!("{}", "Multiple students found:".yellow().bold());
        for i in 0..students.len() {
            println!("{} {}", "Option".cyan().bold(), i);
            println!(
                "{} {} {}",
                students[i].Nombres.green(),
                students[i].Apellido1.blue(),
                students[i].Apellido2.red()
            );
        }
        let mut selected_student = String::new();
        io::stdin()
            .read_line(&mut selected_student)
            .expect("Crash while reading selection");
        let parsed_selected_student: usize = selected_student
            .trim()
            .parse()
            .expect("Error while parsing selection");
        println!(
            "{} {}",
            "Student selected:".yellow().bold(),
            students[parsed_selected_student].Nombres.green()
        );
        students[parsed_selected_student].clone()
    } else if students.len() == 1 {
        let selected = students[0].clone();
        println!(
            "{} {}",
            "Selected:".yellow().bold(),
            selected.Nombres.green()
        );
        selected
    } else {
        println!("{}", "Estudiante no existe".red().bold());
        createStudent()
    }
}

pub fn createStudent() -> Student {
    println!("{}", "Insert rut:".cyan().bold());
    let mut rut = String::new();
    io::stdin()
        .read_line(&mut rut)
        .expect("Crash while reading rut");
    rut = rut.trim().parse().expect("Error while parsing rut");

    println!("{}", "Insert dv:".cyan().bold());
    let mut dv = String::new();
    io::stdin()
        .read_line(&mut dv)
        .expect("Crash while reading dv");
    dv = dv.trim().parse().expect("Error while parsing dv");

    println!("{}", "Insert names:".cyan().bold());
    let mut names = String::new();
    io::stdin()
        .read_line(&mut names)
        .expect("Crash while reading name");
    names = names.trim().to_string().to_uppercase();

    println!("{}", "Insert lastName1:".cyan().bold());
    let mut lastName1 = String::new();
    io::stdin()
        .read_line(&mut lastName1)
        .expect("Crash while reading lastName1");
    lastName1 = lastName1.trim().to_string().to_uppercase();

    println!("{}", "Insert lastName2:".cyan().bold());
    let mut lastName2 = String::new();
    io::stdin()
        .read_line(&mut lastName2)
        .expect("Crash while reading lastName2");
    lastName2 = lastName2.trim().to_string().to_uppercase();

    let connection = sqlite::open("alumnos.db").unwrap();
    let query = format!(
        "INSERT INTO alumnos VALUES (null,{},'{}','{}','{}',{});",
        rut, names, lastName1, lastName2, dv
    );
    println!("{}", query.magenta());
    connection.execute(query).unwrap();
    let newStudent = Student {
        ID: 99,
        Rut: rut.parse().expect("Rut not valid"),
        Nombres: names.trim().to_string(),
        Apellido1: lastName1.trim().to_string(),
        Apellido2: lastName2.trim().to_string(),
        DV: dv.parse().expect("DV not valid"),
    };
    println!("{} {:?}", "Student added:".green().bold(), newStudent);
    newStudent
}
