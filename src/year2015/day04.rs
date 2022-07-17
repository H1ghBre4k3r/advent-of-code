use crate::util::solution;

const INPUT: &str = "ckczppom";

fn find_digest_starting_with(prefix: &str) -> i32 {
    let mut suffix = 1;
    loop {
        let dig = md5::compute(format!("{}{}", INPUT, suffix));
        let format_dig = format!("{:x}", dig);
        if format_dig.starts_with(prefix) {
            return suffix;
        }
        suffix += 1;
    }
}

fn solution_1() {
    solution!(find_digest_starting_with("00000"));
}

fn solution_2() {
    solution!(find_digest_starting_with("000000"));
}

pub fn run_day() {
    solution_1();
    solution_2();
}
