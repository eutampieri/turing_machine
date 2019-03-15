use std::fs::File;
use std::io::*;

#[derive(Debug)]
enum Movement{
    Left,
    Right
}

#[derive(Debug)]
struct Quintuple{
    initial_state: u16,
    read: Option<i32>,
    final_state: u16,
    write: Option<i32>,
    movement: Movement
}

fn main() {
    let mut quintuples: Vec<Quintuple> = Vec::new();
    let mut file = File::open("rules.txt").unwrap();
    let mut rules_raw = String::new();
    file.read_to_string(&mut rules_raw).unwrap();
    let mut rules = rules_raw.split("\n").collect::<Vec<&str>>();
    let info = rules[0].split(",").collect::<Vec<&str>>();
    rules.remove(0);
    for rule_raw in rules{
        let mut rule = rule_raw.replace("(","");
        rule = rule.replace(")","");
        let rule_pieces = rule.split(",").collect::<Vec<&str>>();
        let quintuple = Quintuple{
            initial_state: rule_pieces[0].parse::<u16>().unwrap(),
            read: if rule_pieces[1] == info[0]{None}else{Some(rule_pieces[1].parse::<i32>().unwrap())},
            final_state: rule_pieces[2].parse::<u16>().unwrap(),
            write: if rule_pieces[3] == info[0]{None}else{Some(rule_pieces[3].parse::<i32>().unwrap())},
            movement: if rule_pieces[4] == info[1]{Movement::Left}else{Movement::Right}
        };
        quintuples.push(quintuple);
    }

    let mut tape_raw = String::new();
    let mut file_t = File::open("tape.txt").unwrap();
    file_t.read_to_string(&mut tape_raw).unwrap();
    tape_raw = tape_raw.replace("\n","");//
    let mut tape = tape_raw.split("|").collect::<Vec<&str>>().iter().map(
        |&x| if x == info[0]{None}else{Some(x.parse::<i32>().unwrap())}
    ).collect::<Vec<Option<i32>>>();
    let mut i = 0;
    let mut state = 0;
    let exit_state = info[3].parse::<u16>().unwrap();
    while state != exit_state{
        for rule in &quintuples{
            if rule.initial_state == state && rule.read == tape[i]{
                tape[i] = rule.write;
                state = rule.final_state;
                match rule.movement{
                    Movement::Left => {i = i-1},
                    Movement::Right => {i = i+1}
                }
                continue;
            }
        }
    }
    println!("{:?}", tape)
}
