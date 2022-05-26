use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    let valid_passports = input.split("\n\n").filter(|p| complete_passport(p)).count();
    println!("Part1: Complete passports: {}", valid_passports);

    let valid_passports2 = input.split("\n\n").filter(|p| complete_passport(p) && valid_fields(p)).count();
    println!("Part2: Valid passports: {}", valid_passports2);
}

fn complete_passport(input: &str) -> bool {
    let fields = [
        "byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:", // "cid:",
    ];
    fields.iter().all(|field| input.contains(field))
}

fn valid_fields(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"([a-z]{3}):([^\s]+)").unwrap();
    }
    REGEX
        .captures_iter(input)
        .all(|c| validate_field(&c[1], &c[2]))
}

fn validate_field(field_name: &str, value: &str) -> bool {
    lazy_static! {
        static ref REGEX_HCL: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
        static ref REGEX_HGT: Regex = Regex::new("^([0-9]{2,3})(cm|in)$").unwrap();
    }

    match field_name {
        "byr" => value
            .parse::<u32>()
            .map(|year| (1920..=2002).contains(&year))
            .unwrap_or(false),
        "iyr" => value
            .parse::<u32>()
            .map(|year| (2010..=2020).contains(&year))
            .unwrap_or(false),
        "eyr" => value
            .parse::<u32>()
            .map(|year| (2020..=2030).contains(&year))
            .unwrap_or(false),
        "hgt" => match REGEX_HGT.captures(value) {
            Some(c) if &c[2] == "cm" => c[1]
                .parse::<u32>()
                .map(|l| (150..=193).contains(&l))
                .unwrap_or(false),
            Some(c) if &c[2] == "in" => c[1]
                .parse::<u32>()
                .map(|l| (59..=76).contains(&l))
                .unwrap_or(false),
            _ => false,
        },
        "hcl" => REGEX_HCL.is_match(value),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_alphanumeric()),
        "cid" => true,
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_field() {
        assert_eq!(validate_field("byr", "2002"), true);
        assert_eq!(validate_field("byr", "2003"), false);

        assert_eq!(validate_field("hgt", "60in"), true);
        assert_eq!(validate_field("hgt", "190cm"), true);
        assert_eq!(validate_field("hgt", "190in"), false);
        assert_eq!(validate_field("hgt", "190"), false);

        assert_eq!(validate_field("byr", "2002"), true);
        assert_eq!(validate_field("byr", "2003"), false);
        assert_eq!(validate_field("hgt", "60in"), true);
        assert_eq!(validate_field("hgt", "190cm"), true);
        assert_eq!(validate_field("hgt", "190in"), false);
        assert_eq!(validate_field("hgt", "190"), false);
        assert_eq!(validate_field("hcl", "#123abc"), true);
        assert_eq!(validate_field("hcl", "#123abz"), false);
        assert_eq!(validate_field("hcl", "123abc"), false);
        assert_eq!(validate_field("ecl", "brn"), true);
        assert_eq!(validate_field("ecl", "wat"), false);
        assert_eq!(validate_field("pid", "000000001"), true);
        assert_eq!(validate_field("pid", "0123456789"), false);
    }

    #[test]
    fn valid_passport_test() {
        let input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(input.split("\n\n").filter(|p| complete_passport(p) && valid_fields(p)).count(), 4);
    }

    
    #[test]
    fn invalid_passport_test() {
        let input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(input.split("\n\n").filter(|p| complete_passport(p) && valid_fields(p)).count(), 0);
    }
}