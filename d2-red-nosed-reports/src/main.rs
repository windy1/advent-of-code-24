use std::cmp;

enum ReportTrend {
    Increasing,
    Decreasing,
    None,
}

fn main() {
    let file_path = "./d2-red-nosed-reports/input.txt";
    let contents = std::fs::read_to_string(file_path).unwrap();
    let reports = parse_reports(&contents);
    let safe_reports = count_safe_reports(&reports);

    println!("Safe reports: {}", safe_reports);
}

fn count_safe_reports(reports: &[Vec<i32>]) -> i32 {
    reports
        .iter()
        .filter(|&report| is_safe_report(report))
        .count() as i32
}

fn is_safe_report(report: &[i32]) -> bool {
    if _is_safe_report(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report_copy = report.to_vec();
        report_copy.remove(i);

        if _is_safe_report(&report_copy) {
            return true;
        }
    }

    false
}

fn _is_safe_report(report: &[i32]) -> bool {
    let mut trend = ReportTrend::None;

    for window in report.windows(2) {
        let a = window[0];
        let b = window[1];
        let diff = (a - b).abs();

        if !(1..=3).contains(&diff) {
            return false;
        }

        match trend {
            ReportTrend::None => match a.cmp(&b) {
                cmp::Ordering::Less => trend = ReportTrend::Increasing,
                cmp::Ordering::Greater => trend = ReportTrend::Decreasing,
                cmp::Ordering::Equal => {
                    return false;
                }
            },
            ReportTrend::Increasing => {
                if a >= b {
                    return false;
                }
            }
            ReportTrend::Decreasing => {
                if a <= b {
                    return false;
                }
            }
        }
    }

    true
}

fn parse_reports(str: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();

    for line in str.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.trim())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        reports.push(report);
    }

    reports
}
