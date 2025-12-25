use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = (&args[1]).as_str();

    println!("=============== DAY {day} ===============");

    let start = std::time::Instant::now();
    match day {
        "1.1" => day1::day1(),
        "1.2" => day1::day1_part2(),
        "2.1" => day2::day2(1),
        "2.2" => day2::day2(2),
        "3.1" => day3::day3(1),
        "3.2" => day3::day3(2),
        "4.1" => day4::day4(1),
        "4.2" => day4::day4(2),
        "5.1" => day5::day5(1),
        "5.2" => day5::day5(2),
        "6.1" => day6::day6(1),
        "6.2" => day6::day6(2),
        "7.1" => day7::day7(1),
        "7.2" => day7::day7(2),
        "8.1" => day8::day8(1),
        "8.2" => day8::day8(2),
        "9.1" => day9::day9(1),
        "9.2" => day9::day9(2),
        "10.1" => day10::day10(1),
        "10.2" => day10::day10(2),
        "11.1" => day11::day11(1),
        "11.2" => day11::day11(2),
        "12.1" => day12::day12(1),
        _ => panic!("I'm not there yet!!!!")
    }
    println!("{:.2?}", start.elapsed());
}
