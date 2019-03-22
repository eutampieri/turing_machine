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
    read: Option<char>,
    final_state: u16,
    write: Option<char>,
    movement: Movement
}

fn main() {
    // Initialize a vector of quintuples to store the rules
    let mut quintuples: Vec<Quintuple> = Vec::new();

    // Load the quintuples and the alphabet (kinda) from the file
    let mut file = File::open("rules.txt").unwrap();
    let mut rules_raw = String::new();
    file.read_to_string(&mut rules_raw).unwrap();
    let mut rules = rules_raw.split("\n").collect::<Vec<&str>>();
    // The first line of the file contains parsing informations
    let info = rules[0].split(",").collect::<Vec<&str>>();
    // So we remove it from the rules
    rules.remove(0);

    // Parse the quintuples
    for rule_raw in rules{
        // Discard the parethesis
        let mut rule = rule_raw.replace("(","");
        rule = rule.replace(")","");
        // We're usung the comma as a separator here
        let rule_pieces = rule.split(",").collect::<Vec<&str>>();
        // Create a new Quintuple
        let quintuple = Quintuple{
            initial_state: rule_pieces[0].parse::<u16>().unwrap(),
            // If we're reading a null symbol, then don't try to parse it
            read: if rule_pieces[1] == info[0]{None}else{Some(rule_pieces[1].parse::<char>().unwrap())},
            final_state: rule_pieces[2].parse::<u16>().unwrap(),
            write: if rule_pieces[3] == info[0]{None}else{Some(rule_pieces[3].parse::<char>().unwrap())},
            movement: if rule_pieces[4] == info[1]{Movement::Left}else{Movement::Right}
        };
        // Add to the list of rules
        quintuples.push(quintuple);
    }

    // Parse the tape
    let mut tape_raw = String::new();
    let mut file_t = File::open("tape.txt").unwrap();
    file_t.read_to_string(&mut tape_raw).unwrap();
    tape_raw = tape_raw.replace("\n","");//
    let mut tape = tape_raw.split("|").collect::<Vec<&str>>().iter().map(
        |&x| if x == info[0]{None}else{Some(x.parse::<char>().unwrap())}
    ).collect::<Vec<Option<char>>>();
    let mut i = 0;
    let mut state = 0;
    let exit_state = info[3].parse::<u16>().unwrap();

    // Run until we reach the exit condition
    while state != exit_state{
        // Let's see if there's a matching rule
        for rule in &quintuples{
            if rule.initial_state == state && rule.read == tape[i]{
                // Apply the rule
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

    // Print the result
    for (pos, ref symbol) in tape.iter().enumerate() {
        match symbol{
            Some(x)=>print!("{}", x),
            None => print!("{}", info[0])
        }
        if pos < tape.len()-1{
            print!("|");
        }
    }
    print!("\n");
}
