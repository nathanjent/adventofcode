use day01;
use day02;
use day03;
use day04;
use day05;
use day06;
use day07;
use day08;
use day09;

use day01_2016;

//2015
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
fn day09test1() {
    assert_eq!(day09::single_night("input/day09.txt"), 0);
}

#[test]
fn day09test2() {
    assert_eq!(day09::single_night("input/day09.txt"), 0);
}

// 2016
#[test]
fn day01_2016test2() {
    assert_eq!(day01_2016::taxicab("input/day01_2016.txt"), 0);
}
