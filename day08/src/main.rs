//! https://adventofcode.com/2023/day/8
//!
use std::{
    collections::{HashMap, HashSet},
    iter::Cycle,
    ops::ControlFlow,
    str::Chars,
};

fn main() {
    let s = std::fs::read_to_string("day08.in").unwrap();
    println!("{}", execute_all(&s, "AAA"));
    println!("{}", execute_all_p2(&s));
}

// Data representation
// -------------------
//
// To represent AAA = (BBB, CCC), we use hash table, where:
//              ^^^   ^^^^^^^^^^
//              key      value
//
type Transition = (String, String);

type State = String;

type DesertMap = HashMap<State, Transition>;

fn parse_desert_map(raw_text: &str) -> DesertMap {
    let mut desert_map: DesertMap = HashMap::new();
    for line in raw_text.lines() {
        let (k, v) = parse_map_pair(line);
        desert_map.insert(k, v);
    }
    desert_map
}

#[test]
fn test_parse_map_pair() {
    let test_text = "AAA = (BBB, CCC)";
    let (s, (l, r)) = parse_map_pair(test_text);
    assert_eq!(s, "AAA");
    assert_eq!(l, "BBB");
    assert_eq!(r, "CCC");
}

fn parse_map_pair(line_text: &str) -> (State, Transition) {
    let state = line_text[0..3].to_string();
    let left_transition = line_text[7..10].to_string();
    let right_transition = line_text[12..15].to_string();
    (state, (left_transition, right_transition))
}

// To represent instructions as streams (infinity iterator) of String
type InstStream<'a> = Cycle<Chars<'a>>;

#[test]
fn test_parse_instruction() {
    let instructions = "LR";
    let stream = parse_instruction(instructions);
    assert_eq!(stream.take(4).collect::<String>(), "LRLR");
}

fn parse_instruction(inst: &str) -> InstStream<'_> {
    inst.chars().cycle()
}

fn inst_count(map: &DesertMap, insts: InstStream, init_v: &str) -> u64 {
    match insts
        .zip(1..)
        .try_fold((0_u64, init_v), |acc, (inst, idx)| {
            let next = match inst {
                'L' => map.get(acc.1).unwrap().0.as_str(),
                'R' => map.get(acc.1).unwrap().1.as_str(),
                _ => panic!("bad pattern"),
            };
            match next.ends_with("Z") {
                true => ControlFlow::Break(acc.0 + 1),
                _ => ControlFlow::Continue((acc.0 + 1, next)),
            }
        }) {
        ControlFlow::Break(s) => s,
        _ => panic!("never happens"),
    }
}

#[test]
fn test_inst_count() {
    let inst_pat = "RL";
    let map = "AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    let desert_map = parse_desert_map(map);
    let insts = parse_instruction(inst_pat);
    assert_eq!(2, inst_count(&desert_map, insts, "AAA"));
}

#[test]
fn test_parse_all() {
    let test_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    println!("{:?}", parse_all(test_input));
}

fn parse_all(s: &str) -> Option<(DesertMap, InstStream)> {
    let mut ls = s.lines();
    let stream = parse_instruction(ls.next()?);
    ls.next();
    let remaining = ls.map(|x| format!("{}\n", x)).collect::<String>();
    Some((parse_desert_map(&remaining), stream))
}

#[test]
fn test_execute_all() {
    let test_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    println!("{}", execute_all(test_input, "AAA"));
}

fn execute_all(s: &str, init_v: &str) -> u64 {
    let (dm, inst) = parse_all(s).unwrap();
    inst_count(&dm, inst, init_v)
}

// part 2 specifics
fn find_all_as(m: &DesertMap) -> HashSet<&str> {
    let mut set = HashSet::new();
    for k in m.keys() {
        match k.ends_with("A") {
            true => {
                set.insert(k.as_str());
            }
            _ => continue,
        }
    }
    set
}

fn calculate_p2(m: &DesertMap, insts: InstStream) -> u64 {
    let start_with_as = find_all_as(m);
    let mut lcm = 1_u64;
    for a in start_with_as {
        lcm = num::integer::lcm(lcm, inst_count(m, insts.clone(), a));
    }
    lcm
}

#[test]
fn test_exec_p2() {
    let test_text = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    println!("{}", execute_all_p2(test_text))
}

fn execute_all_p2(s: &str) -> u64 {
    let (dm, inst) = parse_all(s).unwrap();
    calculate_p2(&dm, inst)
}
