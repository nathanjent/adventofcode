use day01;
use day02;
use day03;
use day04;
use day05;
use day06;
use day07;
use day08;
use day09;
use day10;
use day11;
use day12;
use day13;
use day14;
use day15;
use day16;
use day17;
use day18;
use day19;

use day01_2016;
use day02_2016;
use day03_2016;
use day04_2016;
use day05_2016;
use day06_2016;
use day07_2016;
use day08_2016;
use day09_2016;
use day10_2016;
use day11_2016;

// 2015
#[test]
fn day01test1() {
    assert_eq!(day01::not_quite_lisp_1("input/day01.txt"), 232);
}

#[test]
fn day01test2() {
    assert_eq!(day01::not_quite_lisp_2("input/day01.txt"), 1783);
}

#[test]
fn day02test1() {
    assert_eq!(day02::no_math_1("input/day02.txt"), 1586300);
}

#[test]
fn day02test2() {
    assert_eq!(day02::no_math_2("input/day02.txt"), 3737498);
}

#[test]
fn day03test1() {
    assert_eq!(day03::sperical_houses_1("input/day03.txt"), 2592);
}

#[test]
fn day03test2() {
    assert_eq!(day03::sperical_houses_2("input/day03.txt"), 2360);
}

#[test]
fn day04test1() {
    assert_eq!(day04::ideal_stocking_stuffer("input/day04.txt", 5), 282749);
}

#[test]
fn day04test2() {
    assert_eq!(day04::ideal_stocking_stuffer("input/day04.txt", 6), 9962624);
}

#[test]
fn day05test1() {
    assert_eq!(day05::intern_elves_1("input/day05.txt"), 236);
}

#[test]
fn day05test2() {
    assert_eq!(day05::intern_elves_2("input/day05.txt"), 51);
}

#[test]
fn day06test1() {
    assert_eq!(day06::fire_hazard_1("input/day06.txt"), 377891);
}

#[test]
fn day06test2() {
    assert_eq!(day06::fire_hazard_2("input/day06.txt"), 14110788);
}

#[test]
fn day07test1() {
    assert_eq!(day07::assembly_required("input/day07.txt", "", 0), 956);
}

#[test]
fn day07test2() {
    assert_eq!(day07::assembly_required("input/day07.txt", "b", 956), 40149);
}

#[test]
fn day08test1() {
    assert_eq!(day08::matchsticks_1("input/day08.txt"), 1371);
}

#[test]
fn day08test2() {
    assert_eq!(day08::matchsticks_2("input/day08.txt"), 2117);
}

#[test]
fn day09test1base() {
    assert_eq!(day09::single_night_1("input/day09_base.txt"), 605);
}

#[test]
fn day09test2base() {
    assert_eq!(day09::single_night_2("input/day09_base.txt"), 982);
}

#[test]
fn day09test1() {
    assert_eq!(day09::single_night_1("input/day09.txt"), 251);
}

#[test]
fn day09test2() {
    assert_eq!(day09::single_night_2("input/day09.txt"), 0);
}

#[test]
fn day10test1() {
    assert_eq!(day10::look_say_1("input/day10.txt"), 329356);
}

#[test]
fn day10test2() {
    assert_eq!(day10::look_say_2("input/day10.txt"), 4666278);
}

#[test]
fn day11test1base() {
    assert_eq!(day11::corporate_policy_1("input/day11_base.txt"), "abcdffaa");
}

#[test]
fn day11test1() {
    assert_eq!(day11::corporate_policy_1("input/day11.txt"), "hxbxxyzz");
}

#[test]
fn day11test2() {
    assert_eq!(day11::corporate_policy_2("input/day11.txt"), "hxcaabcc");
}

#[test]
fn day12test2base() {
    assert_eq!(day12::abacus_framework_2("input/day12_base.txt"), 16);
}

#[test]
fn day12test1() {
    assert_eq!(day12::abacus_framework_1("input/day12.txt"), 156366);
}

#[test]
fn day12test2() {
    assert_eq!(day12::abacus_framework_2("input/day12.txt"), 96852);
}

#[test]
fn day13test1base() {
    assert_eq!(day13::knights_table_1("input/day13_base.txt"), 330);
}

#[test]
fn day13test1() {
    assert_eq!(day13::knights_table_1("input/day13.txt"), 618);
}

#[test]
fn day13test2() {
    assert_eq!(day13::knights_table_2("input/day13.txt"), 601);
}

#[test]
fn day14test1base() {
    assert_eq!(day14::reindeer_olympics_1("input/day14_base.txt", 1000), 1120);
}

#[test]
fn day14test1() {
    assert_eq!(day14::reindeer_olympics_1("input/day14.txt", 2503), 2696);
}

#[test]
fn day14test2() {
    assert_eq!(day14::reindeer_olympics_2("input/day14.txt", 2503), 1084);
}

#[test]
fn day15test1() {
    assert_eq!(day15::hungry_science_1("input/day15.txt"), 222870);
}

#[test]
fn day15test2() {
    assert_eq!(day15::hungry_science_2("input/day15.txt"), 117936);
}

#[test]
fn day16test1() {
    assert_eq!(day16::aunt_sue_1("input/day16.txt"), 40);
}

#[test]
fn day16test2() {
    assert_eq!(day16::aunt_sue_2("input/day16.txt"), 241);
}

#[test]
fn day17test1base() {
    assert_eq!(day17::much_such_1("input/day17_base.txt", 25), 4);
}

#[test]
fn day17test2base() {
    assert_eq!(day17::much_such_2("input/day17_base.txt", 25), 3);
}

#[test]
fn day17test1() {
    assert_eq!(day17::much_such_1("input/day17.txt", 150), 1304);
}

#[test]
fn day17test2() {
    assert_eq!(day17::much_such_2("input/day17.txt", 150), 18);
}

#[test]
fn day18test1base() {
    assert_eq!(day18::yard_gif_1("input/day18_base.txt", 4), 4);
}

#[test]
fn day18test1() {
    assert_eq!(day18::yard_gif_1("input/day18.txt", 100), 768);
}

#[test]
fn day18test2() {
    assert_eq!(day18::yard_gif_2("input/day18.txt", 100), 781);
}

#[test]
fn day19test1() {
    assert_eq!(day19::rudolph_meds_1("input/day19.txt"), 0);
}

#[test]
fn day19test2() {
    assert_eq!(day19::rudolph_meds_2("input/day19.txt"), 0);
}

// 2016
#[test]
fn day01_2016test1() {
    assert_eq!(day01_2016::taxicab_1("input/day01_2016.txt"), 161);
}

#[test]
fn day01_2016test2() {
    assert_eq!(day01_2016::taxicab_2("input/day01_2016.txt"), 110);
}

#[test]
fn day02_2016test1() {
    assert_eq!(day02_2016::bathroom_security_1("input/day02_2016.txt"),
               "95549");
}

#[test]
fn day02_2016test2() {
    assert_eq!(day02_2016::bathroom_security_2("input/day02_2016.txt"), "");
}

#[test]
fn day03_2016test1() {
    assert_eq!(day03_2016::square_triangles_1("input/day03_2016.txt"), 862);
}

#[test]
fn day03_2016test2() {
    assert_eq!(day03_2016::square_triangles_2("input/day03_2016.txt"), 1577);
}

#[test]
fn day04_2016test1() {
    assert_eq!(day04_2016::obsecurity_1("input/day04_2016.txt"), 185371);
}

#[test]
fn day04_2016test2() {
    assert_eq!(day04_2016::obsecurity_2("input/day04_2016.txt", "northpole object"),
               984);
}

#[test]
fn day05_2016test1() {
    assert_eq!(day05_2016::chess_1("input/day05_2016.txt"),
               "2414bc77".to_string());
}

#[test]
fn day05_2016test2() {
    assert_eq!(day05_2016::chess_2("input/day05_2016.txt"),
               "437e60fc".to_string());
}

#[test]
fn day06_2016test1() {
    assert_eq!(day06_2016::noisy_signals_1("input/day06_2016.txt"),
               "tkspfjcc".to_string());
}

#[test]
fn day06_2016test2() {
    assert_eq!(day06_2016::noisy_signals_2("input/day06_2016.txt"),
               "xrlmbypn".to_string());
}

#[test]
fn day07_2016test1base() {
    assert_eq!(day07_2016::ip_7_1("input/day07_2016_base.txt"), 2);
}

#[test]
fn day07_2016test2base() {
    assert_eq!(day07_2016::ip_7_2("input/day07_2016_base2.txt"), 3);
}

#[test]
fn day07_2016test1() {
    assert_eq!(day07_2016::ip_7_1("input/day07_2016.txt"), 118);
}

#[test]
fn day07_2016test2() {
    assert_eq!(day07_2016::ip_7_2("input/day07_2016.txt"), 260);
}

#[test]
fn day08_2016test1base() {
    assert_eq!(day08_2016::two_factor_1("input/day08_2016_base.txt", 7, 3),
               6);
}

#[test]
fn day08_2016test1() {
    assert_eq!(day08_2016::two_factor_1("input/day08_2016.txt", 50, 6), 0);
}

#[test]
fn day08_2016test2() {
    assert_eq!(day08_2016::two_factor_2("input/day08_2016.txt", 50, 6), 0);
}

#[test]
fn day09_2016test1base() {
    assert_eq!(day09_2016::explosives_1("input/day09_2016_base.txt"), 57);
}

#[test]
fn day09_2016test2base() {
    assert_eq!(day09_2016::explosives_2("input/day09_2016_base2.txt"),
               242394);
}

#[test]
fn day09_2016test1() {
    assert_eq!(day09_2016::explosives_1("input/day09_2016.txt"), 74532);
}

#[test]
fn day09_2016test2() {
    assert_eq!(day09_2016::explosives_2("input/day09_2016.txt"),
               11558231665);
}

#[test]
fn day10_2016test1base() {
    assert_eq!(day10_2016::balance_bots_1("input/day10_2016_base.txt"), 0);
}

#[test]
fn day10_2016test1() {
    assert_eq!(day10_2016::balance_bots_1("input/day10_2016.txt"), 0);
}

#[test]
fn day10_2016test2() {
    assert_eq!(day10_2016::balance_bots_2("input/day10_2016.txt"), 0);
}

#[test]
fn day11_2016test1() {
    assert_eq!(day11_2016::rtg_1("input/day11_2016.txt"), 0);
}

#[test]
fn day11_2016test2() {
    assert_eq!(day11_2016::rtg_2("input/day11_2016.txt"), 0);
}
