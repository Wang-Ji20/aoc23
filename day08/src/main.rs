//! https://adventofcode.com/2023/day/8
//!
use std::{collections::HashMap, iter::Cycle, ops::ControlFlow, str::Chars};

fn main() {
    let s = std::fs::read_to_string("day08.in").unwrap();
    println!("{}", execute_all(&s));
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

type InstStream<'a> = Cycle<Chars<'a>>;

#[test]
fn test_parse_instruction() {
    let instructions = "LR";
    let stream = parse_instruction(instructions);
    assert_eq!(stream.take(4).collect::<String>(), "LRLR");
}

// To represent instructions as streams (infinity iterator) of String
fn parse_instruction(inst: &str) -> InstStream<'_> {
    inst.chars().cycle()
}

fn inst_count(map: &DesertMap, insts: InstStream) -> u64 {
    match insts.zip(1..).try_fold((0_u64, "AAA"), |acc, (inst, idx)| {
        let next = match inst {
            'L' => map.get(acc.1).unwrap().0.as_str(),
            'R' => map.get(acc.1).unwrap().1.as_str(),
            _ => panic!("bad pattern"),
        };
        match next {
            "ZZZ" => ControlFlow::Break(acc.0 + 1),
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
    assert_eq!(2, inst_count(&desert_map, insts));
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
    println!("{}", execute_all(test_input));
}

fn execute_all(s: &str) -> u64 {
    let (dm, inst) = parse_all(s).unwrap();
    inst_count(&dm, inst)
}
