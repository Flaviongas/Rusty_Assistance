#![allow(non_snake_case)]
pub mod database;
use calamine::{open_workbook, Reader, Xlsx};
use chrono::prelude::*;
use colored::*;
use rust_xlsxwriter::{Color, Format, Workbook};
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
    println!(
        "{}",
        "Initializing Attendance Record Program...".cyan().bold()
    );
    let plantilla = String::from("template.xlsx");
    let day_month = Local::now().format("%d-%m").to_string();
    let day_month_year = Local::now().format("%d-%m-%Y").to_string();
    let current_day = chrono::offset::Local::now()
        .date_naive()
        .weekday()
        .to_string();
    let spanish_day = Day::spanish();
    let mapping = spanish_day.get_day(&current_day).unwrap();

    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), plantilla);
    let mut excel: Xlsx<_> = open_workbook(path).unwrap();

    if let Ok(r) = excel.worksheet_range("ASISTENCIA") {
        println!("{}", "Template loaded successfully.".green().bold());
        let rows: Vec<_> = r.rows().collect();
        if !rows.is_empty() {
            let headers: Vec<_> = rows[0].to_vec();

            let mut workbook = Workbook::new();
            let worksheet = workbook.add_worksheet();
            let color_format = Format::new()
                .set_background_color(Color::RGB(0xc00000))
                .set_font_color(Color::White)
                .set_bold()
                .set_border(rust_xlsxwriter::FormatBorder::Thin);

            println!("{}", "Writing headers to new workbook:".cyan().bold());
            let mut col = 0;
            for row in headers {
                worksheet
                    .write_string_with_format(0, col, row.to_string(), &color_format)
                    .unwrap();
                col += 1;
            }

            let _ = worksheet.set_column_width_pixels(0, 110);
            let _ = worksheet.set_column_width_pixels(1, 140);
            let _ = worksheet.set_column_width_pixels(2, 110);
            let _ = worksheet.set_column_width_pixels(3, 200);
            let _ = worksheet.set_column_width_pixels(4, 200);
            let _ = worksheet.set_column_width_pixels(5, 100);
            let _ = worksheet.set_column_width_pixels(6, 400);

            println!("{}", "How many students?".yellow().bold());
            let mut amount = String::new();
            io::stdin()
                .read_line(&mut amount)
                .expect("Crash while reading N° of Students");

            for i in 0..amount.trim().parse::<u8>().unwrap() {
                println!("{} {}", "Enter name of student".blue().bold(), i + 1);
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Crash while reading name of Student");

                let student = database::get_student(name);
                let last_names = format!("{} {}", student.Apellido1, student.Apellido2);

                worksheet
                    .write((i + 1).into(), 0, day_month_year.clone())
                    .unwrap();
                worksheet
                    .write((i + 1).into(), 1, student.Rut.to_string())
                    .unwrap();
                worksheet
                    .write((i + 1).into(), 2, student.DV.to_string())
                    .unwrap();
                worksheet
                    .write((i + 1).into(), 3, student.Nombres.to_string())
                    .unwrap();
                worksheet.write((i + 1).into(), 4, last_names).unwrap();
                worksheet.write((i + 1).into(), 5, "1").unwrap();
                worksheet
                    .write((i + 1).into(), 6, "FÍSICA MECANICA / 3")
                    .unwrap();
            }

            let output_file = format!(
                "REGISTROS DE ASISTENCIA - SAAC ({} {} INFORMATICA).xlsx",
                mapping, day_month
            );
            workbook.save(&output_file).unwrap();
            println!(
                "{} {}",
                "Excel file compiled successfully:".green().bold(),
                output_file.yellow().underline()
            );
        } else {
            println!("{}", "No data found in the worksheet!".red().bold());
        }
    }
}
