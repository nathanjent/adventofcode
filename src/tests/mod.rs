use day01;
use day02;
use day03;
use day04;
use day05;
use day06;
use day07;

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
#[ignore]
fn day05test2() {
    assert_eq!(day05::intern_elves_2("input/day05.txt"), 51);
}

#[test]
fn day06test1() {
    assert_eq!(day06::fire_hazard("input/day06.txt"), 377891);
}

#[test]
fn day06test2() {
    assert_eq!(day06::fire_hazard("input/day06.txt"), 14110788);
}

#[test]
fn day07test1() {
    assert_eq!(day07::assembly_required("input/day07.txt", "", 0), 956);
}

#[test]
fn day07test2() {
    println!("{:?}", day07::assembly_required("input/day07.txt", "b", 956));
}
