pub mod database;
use chrono::prelude::*;
use std::io;

use serde::{Deserialize, Serialize};

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
    let _period = "OTOÑO";
    let _plantilla = "template.xlsx";
    let local: DateTime<Local> = Local::now(); //`2014-11-28T21:45:59.324310806+09:00`
    let _day_month_year = local.format("%d-%m-%Y").to_string(); // 09-09-1999
    let current_day = chrono::offset::Local::now()
        .date_naive()
        .weekday()
        .to_string();
    let spanish_day = Day::spanish();
    let mapping = spanish_day.get_day(&current_day).unwrap();

    println!("{:?}", mapping);
    //let find_current = day_list.iter().find(|item| **item == current_day);
    //println!("{:?}", find_current);

    //let mut selection = String::new();
    //io::stdin()
    //    .read_line(&mut selection)
    //    .expect("Crash while reading name");
    //database::get_student(selection)
}
