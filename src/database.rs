use sqlite::State;

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
        // TODO: Be able to select
        println!("students: {:?}", students);
        students[0].clone()
    } else if students.len() == 1 {
        let selected = students[0].clone();
        println!("Selected: {:?}", selected.Nombres);
        selected
    } else {
        // TODO: Be able to create
        println!("Estudiante no existe");
        students[0].clone()
    }
}
