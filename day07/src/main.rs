#[macro_use]
extern crate lazy_static;

use std::io::BufRead;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use regex::Regex;
use itertools::chain;

fn main() {
    lazy_static! {
        static ref CHILD_RE: Regex = Regex::new(r"^([0-9]+) ([a-z]+ [a-z]+) bag[s.]*$").unwrap();
    }

    let lines = std::io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    
    let mut containers = HashMap::<String, Vec<String>>::new();
    
    let mut parse = |line: String| {
        let mut chunks = line.split(" bags contain ");
        
        let parent = chunks.next().unwrap();
        if !containers.contains_key(parent) {
            containers.insert(parent.to_string(), Vec::<_>::new());
        }

        let children = chunks.next().unwrap()
            .split(", ")
            .filter_map(|s| {
                if let Some(caps) = CHILD_RE.captures(s) {
                    Some(caps.get(2).unwrap().as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        
        for child in children {
            if let Some(parents) = containers.get_mut(child) {
                parents.push(parent.to_string());
            } else {
                containers.insert(child.to_string(), vec![parent.to_string()]);
            }
        }
    };

    for line in lines {
        parse(line);
    }

    let mut can_contain: HashSet<_> = HashSet::from_iter(containers.remove("shiny gold").unwrap());
    let mut num_parents = can_contain.len();

    loop {
        let additional = can_contain.iter()
            .filter_map(|color| containers.remove(color))
            .collect::<Vec<_>>()
            .concat();
        
        can_contain = chain(can_contain, additional).collect();

        if can_contain.len() == num_parents { // Didn't add anything on this pass
            break;
        }
        else {
            num_parents = can_contain.len();
        }
    }

    println!("Part 1: {}", num_parents);
}