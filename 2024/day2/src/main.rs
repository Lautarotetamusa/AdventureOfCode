use std::io::{self, Read};

//struct Report {
//    levels: Vec<u32>
//}

fn is_safe(report: Vec<u32>) -> bool {
    if report.len() < 2 {
        return true;
    }
    let incresing = report[1] > report[0];
    let mut prev: u32 = match incresing {
        true => report[0] - 1,
        false => report[0] + 1,
    };

    for level in report {
        if incresing != (level > prev) {
            return false
        }

        let diff = level.abs_diff(prev);
        if  diff < 1 || diff > 3 {
            return false
        }
        prev = level;
    }
    true
}

fn subsets(report: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut result = Vec::new();
    for i in 0..report.len() {
        let mut subset = report.clone();
        subset.remove(i); 
        result.push(subset);
    }
    result
}

fn part_one(reports: Vec<Vec<u32>>) -> u32 {
    let mut ans = 0;
    for report in reports {
        if is_safe(report) {
            ans += 1
        }    
    }
    return ans;
}

fn part_two(reports: Vec<Vec<u32>>) -> u32 {
    let mut ans = 0;
    for report in reports {
        for r in subsets(&report) {
            if is_safe(r) {
                ans += 1;
                break;
            }    
        }
    }
    return ans;
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let reports = buf
        .lines()
        .map(|line| {
            line
                .split(' ')
                .map(|s| {s.parse::<u32>().unwrap_or(0)})
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    println!("number of safe reports: {}", part_one(reports.clone()));
    println!("number of safe reports removing one: {}", part_two(reports));
}
