use crate::Problem;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Instance<'a> {
    passports: Vec<HashMap<&'a str, &'a str>>,
}

fn passport_has_required_fields(passport: &HashMap<&str, &str>) -> bool {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in required_fields {
        if !passport.contains_key(field) {
            return false;
        }
    }

    true
}

impl<'a> Problem<'a> for Instance<'a> {
    fn new(input: &'a str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let passports = lines
            .split(|line| line.is_empty())
            .map(|entry_lines| {
                entry_lines
                    .iter()
                    .flat_map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
                    .map(|entry| entry.split(':').tuples::<(_, _)>().next().unwrap())
                    .collect()
            })
            .collect::<Vec<_>>();

        Instance { passports }
    }

    fn solve_part_one(&self) -> usize {
        self.passports
            .iter()
            .filter(|p| passport_has_required_fields(p))
            .count()
    }

    fn solve_part_two(&self) -> usize {
        let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let passports_with_required_fields = self
            .passports
            .iter()
            .filter(|p| passport_has_required_fields(p));

        let mut valid_count = 0;
        for passport in passports_with_required_fields {
            let is_valid = 'validation: {
                // Birth year check
                let byr = passport.get("byr").unwrap().parse().unwrap();
                if !(1920..=2002).contains(&byr) {
                    break 'validation false;
                }

                // Issue year
                let iyr = passport.get("iyr").unwrap().parse().unwrap();
                if !(2010..=2020).contains(&iyr) {
                    break 'validation false;
                }

                // Expiration year
                let eyr = passport.get("eyr").unwrap().parse().unwrap();
                if !(2020..=2030).contains(&eyr) {
                    break 'validation false;
                }

                // Height
                let hgt = passport.get("hgt").unwrap();
                let hgt_unit = &hgt[hgt.len() - 2..];
                if !(hgt_unit == "cm" || hgt_unit == "in") {
                    break 'validation false;
                }
                let hgt_num: i32 = hgt[..hgt.len() - 2].parse().unwrap();
                let valid_range = if hgt_unit == "cm" { 150..=193 } else { 59..=76 };
                if !valid_range.contains(&hgt_num) {
                    break 'validation false;
                }

                // Hair Color
                let hcl = passport.get("hcl").unwrap().as_bytes();
                if !hcl.len() == 7
                    || hcl[0] != b'#'
                    || !hcl.iter().skip(1).all(u8::is_ascii_hexdigit)
                {
                    break 'validation false;
                }

                // Eye color
                let ecl = passport.get("ecl").unwrap();
                if !valid_eye_colors.iter().any(|c| c == ecl) {
                    break 'validation false;
                }

                // Passport ID
                let pid = passport.get("pid").unwrap();
                if pid.len() != 9 || !pid.as_bytes().iter().all(u8::is_ascii_digit) {
                    break 'validation false;
                }

                true
            };

            if is_valid {
                valid_count += 1;
            }
        }

        valid_count
    }
}
