pub mod database;
use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};
use chrono::prelude::*;
use rust_xlsxwriter::{Workbook, XlsxError};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    Mon: String,
    Tue: String,
    Wed: String,
    Thu: String,
    Fri: String,
    Sat: String,
    Sun: String,
}
#[derive(Debug, Deserialize)]
struct MyRow {
    column1: String,
    column2: String,
    column3: String,
    column4: String,
    column5: String,
    column6: String,
    column7: String,
}

impl Day {
    fn spanish() -> Day {
        Day {
            Mon: String::from("LUNES"),
            Tue: String::from("MARTES"),
            Wed: String::from("MIERCOLES"),
            Thu: String::from("JUEVES"),
            Fri: String::from("VIERNES"),
            Sat: String::from("SABADO"),
            Sun: String::from("DOMINGO"),
        }
    }
    fn get_day(&self, key: &str) -> Option<&String> {
        match key {
            "Mon" => Some(&self.Mon),
            "Tue" => Some(&self.Tue),
            "Wed" => Some(&self.Wed),
            "Thu" => Some(&self.Thu),
            "Fri" => Some(&self.Fri),
            "Sat" => Some(&self.Sat),
            "Sun" => Some(&self.Sun),
            _ => None,
        }
    }
}

fn main() {
    //database::get_all_students()
    let _period = String::from("OTOÑO");
    let plantilla = String::from("template.xlsx");
    let _day_month_year = Local::now().format("%d-%m-%Y").to_string(); // 09-09-1999
    let current_day = chrono::offset::Local::now()
        .date_naive()
        .weekday()
        .to_string();
    let spanish_day = Day::spanish();
    let mapping = spanish_day.get_day(&current_day).unwrap();
    println!("{}", mapping);

    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), plantilla);
    println!("{}", path);
    let mut excel: Xlsx<_> = open_workbook(path).unwrap();

    if let Ok(r) = excel.worksheet_range("ASISTENCIA") {
        let rows: Vec<_> = r.rows().collect();
        if !rows.is_empty() {
            let headers: Vec<_> = rows[0].to_vec();
            println!("row={:?}", headers);

            let mut workbook = Workbook::new();

            let worksheet = workbook.add_worksheet();

            let mut col = 0;
            for row in headers {
                worksheet.write(0, col, row.to_string()).unwrap();
                col += 1;
            }
            workbook.save("tutorial1.xlsx").unwrap();
        }
    }

    //let mut selection = String::new();
    //io::stdin()
    //    .read_line(&mut selection)
    //    .expect("Crash while reading name");
    //database::get_student(selection)
}
