use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reports = get_input();
    let valid_reports = reports.iter().filter(|r| is_safe(r)).count();
    println!("Valid reports: {}", valid_reports);
    let valid_dampened_reports =
        reports.iter().filter(|r| is_safe_dampened(r)).count();
    println!("Valid dampened reports: {}", valid_dampened_reports);
}

fn is_safe_dampened(report: &[i32]) -> bool {
    report.iter().enumerate().any(|(i, _)| {
        let mut new = report.to_owned();
        new.remove(i);
        is_safe(&new)
    })
}

fn is_safe(report: &[i32]) -> bool {
    let mut increasing = Option::None;
    for i in 0..report.len() - 1 {
        let a = report[i];
        let b = report[i + 1];
        let diff = b - a;
        if diff == 0 {
            return false;
        }
        match increasing {
            None => increasing = Some(diff > 0),
            Some(true) => {
                if diff < 0 {
                    return false;
                }
            }
            Some(false) => {
                if diff > 0 {
                    return false;
                }
            }
        }
        if diff.abs() > 3 {
            return false;
        }
    }
    true
}

fn get_input() -> Vec<Vec<i32>> {
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let reports = reader
        .lines()
        .map(|l| {
            let line = l.expect("error reading line");
            let level = line.split(" ");
            let levels = level
                .into_iter()
                .map(|l| l.trim().parse::<i32>().expect("error parsing level"))
                .collect::<Vec<i32>>();
            levels
        })
        .collect::<Vec<Vec<i32>>>();
    reports
}
