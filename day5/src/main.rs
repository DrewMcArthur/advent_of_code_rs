use std::collections::HashMap;

use log::{debug, LevelFilter::Info};
use simple_logger::SimpleLogger;

/// a hashmap where the value is a list of page numbers that must come before the key
#[derive(Debug)]
struct Rules(HashMap<String, Vec<String>>);

impl Rules {
    // these aren't necessary, but they remove the `.0` from code below
    fn get(&self, key: &str) -> Option<&Vec<String>> {
        self.0.get(key)
    }

    fn add(&mut self, val: &str, prerequisite: &str) {
        match self.0.get_mut(val) {
            Some(prereqs) => prereqs.push(prerequisite.to_string()),
            None => {
                self.0
                    .insert(val.to_string(), vec![prerequisite.to_string()]);
            }
        }
    }
}

/// a list of page numbers
#[derive(Debug)]
struct Update(Vec<String>);

impl Update {
    fn new() -> Update {
        Update(Vec::new())
    }

    fn from(line: &str) -> Update {
        let pages = line.split(",").map(|s| s.to_string()).collect();
        Update(pages)
    }

    fn contains(&self, page: &String) -> bool {
        self.0.contains(page)
    }

    fn append(&mut self, pages: &mut Vec<String>) {
        self.0.append(pages);
    }

    fn iter(&self) -> std::slice::Iter<'_, String> {
        self.0.iter()
    }

    fn middle(&self) -> i32 {
        self.0[self.0.len() / 2].clone().parse().unwrap()
    }

    fn at(&self, i: usize) -> &String {
        &self.0[i]
    }

    fn get_pages_before(&self, i: usize) -> &[String] {
        &self.0[0..i]
    }
}

fn main() {
    SimpleLogger::new().with_level(Info).init().unwrap();
    let (rules, updates) = parse_input(load());

    let middles_sum: i32 = updates
        .iter()
        .filter(|u| follows_rules(&rules, u))
        .map(|u| u.middle())
        .sum();

    println!("pt1: {}", middles_sum);

    let corrected_middles_sum: i32 = updates
        .iter()
        .filter(|u| !follows_rules(&rules, u))
        .map(|u| fix_update(&rules, u))
        .map(|u| u.middle())
        .sum();
    println!("pt2: {}", corrected_middles_sum);
}

fn load() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(lines: Vec<String>) -> (Rules, Vec<Update>) {
    let mut in_rules_section = true;
    let mut rules = Rules(HashMap::new());
    let mut updates = Vec::new();
    for line in lines {
        if line == "" {
            in_rules_section = false;
            continue;
        }

        if in_rules_section {
            let (a, b) = line.split_once("|").unwrap();
            rules.add(b, a);
        } else {
            updates.push(Update::from(&line));
        }
    }

    (rules, updates)
}

fn follows_rules(rules: &Rules, update: &Update) -> bool {
    for (i, page) in update.iter().enumerate() {
        if let Some(prereqs) = rules.get(page) {
            let follows = update.get_pages_before(i);
            let should_follow: Vec<&String> =
                prereqs.iter().filter(|s| update.contains(s)).collect();
            if should_follow.iter().any(|s| !follows.contains(&s)) {
                debug!(
                    "{:?} doesn't follow at {} because {} should follow {:?}",
                    update,
                    i,
                    update.at(i),
                    should_follow,
                );
                return false;
            }
        }
    }
    return true;
}

/// reorders the update so that it follows the rules
fn fix_update(rules: &Rules, update: &Update) -> Update {
    let mut new_update = Update::new();

    let not_yet_added = |u: &&String, dest: &Update| !dest.contains(u);
    let remaining_requirements = |n, dest: &Update| {
        rules
            .get(n)
            .unwrap()
            .iter()
            .filter(|f| update.contains(f) && !dest.contains(f))
            .collect::<Vec<&String>>()
    };

    loop {
        let requirements: Vec<(&String, Vec<&String>)> = update
            .iter()
            .filter(|u| not_yet_added(u, &new_update)) // numbers we haven't added yet
            .map(|n| (n, remaining_requirements(n, &new_update)))
            .collect();

        if requirements.len() == 0 {
            // no more requirements, done!
            return new_update;
        }

        // get any numbers with zero remaining requirements
        let mut next: Vec<String> = requirements
            .iter()
            .filter(|(_, v)| v.len() == 0)
            .map(|(k, _)| k.to_string())
            .collect();

        if next.len() == 0 {
            // if we still have requirements but nothing can be added, we panic
            panic!(
                "no solution for {:?} with rules: {:?}",
                update, requirements
            );
        }

        // add any numbers without requirements
        new_update.append(&mut next);
    }
}
