use sqlite::State;

pub fn get_all_students(){
    println!("getAllStudents");
    let connection= sqlite::open("alumnos.db").unwrap();
    let query = "SELECT * FROM alumnos";
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(State::Row) = statement.next() {
        let column_n = statement.column_count();
        for i in 0..column_n {
            let name = statement.column_name(i).unwrap();
            let value: Option<String> = statement.read(i).ok();
println!("  {} = {:?}", name, value.unwrap());
        }
    }
}
pub fn get_student(name:String){

    let formatted_name= name.trim();
    let pattern= format!("{}%",formatted_name);
    let connection= sqlite::open("alumnos.db").unwrap();

    let query = "SELECT * FROM alumnos WHERE LOWER(Nombres) like LOWER(?)";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, &pattern[..])).expect("Error binding value");


    while let Ok(sqlite::State::Row) = statement.next() {
        let column_n = statement.column_count();
        for i in 0..column_n {
            let name = statement.column_name(i).unwrap();
            let value: Option<String> = statement.read(i).ok();
println!("  {} = {:?}", name, value.unwrap());
        }
    }
}

    //conn = sql.connect("alumnos.db")
    //cursor = conn.cursor()
    //add = f"SELECT * FROM alumnos"
    //cursor.execute(add)
    //data = cursor.fetchall()
    //conn.commit()
    //conn.close()
    //print(data)
