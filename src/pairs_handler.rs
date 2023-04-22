use std::collections::HashMap;

use crate::Member;

fn get_days_since_last_pair(pair_history: &HashMap<String, Vec<Vec<String>>>, member1: &str, member2: &str) -> u32 {
    let mut days = 0;

    let mut dates: Vec<&String> = pair_history.keys().collect();
    dates.sort_unstable();
    dates.reverse();

    for date in dates {
        let pairs = &pair_history[date];
        for pair in pairs {
            if pair.contains(&member1.to_string()) && pair.contains(&member2.to_string()) {
                return days;
            }
        }
        days += 1;
    }

    days
}

fn get_optimal_pairing(pair_history: &HashMap<String, Vec<Vec<String>>>, remaining: &[String], pairs: &mut Vec<Vec<String>>) -> Vec<Vec<String>> {
    if remaining.is_empty() {
        return pairs.clone();
    }

    let mut best_pairs = None;
    let mut best_score = None;

    for i in 0..remaining.len() {
        let member1 = remaining[i].clone();
        for j in (i + 1)..remaining.len() {
            let member2 = remaining[j].clone();

            let new_remaining: Vec<String> = remaining
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != i && *idx != j)
                .map(|(_, m)| m.clone())
                .collect();

            let mut new_pairs = pairs.clone();
            new_pairs.push(vec![member1.clone(), member2.clone()]);

            let current_pairs = get_optimal_pairing(pair_history, &new_remaining, &mut new_pairs);
            let score = current_pairs.iter().map(|pair| get_days_since_last_pair(pair_history, &pair[0], &pair[1])).sum::<u32>();

            if best_score.map(|s| score > s).unwrap_or(true) {
                best_score = Some(score);
                best_pairs = Some(current_pairs);
            }
        }
    }

    best_pairs.unwrap_or_else(|| vec![])
}

pub fn generate_pairs(members: &Vec<Member>, history: &HashMap<String, Vec<Vec<String>>>) -> Vec<Vec<String>> {
    let remaining: Vec<String> = members.iter().map(|m| m.name.clone()).collect();
    let mut pairs: Vec<Vec<String>> = Vec::new();

    get_optimal_pairing(history, &remaining, &mut pairs)
}

pub fn pairs_to_string(pairs: Vec<Vec<String>>) -> String {
    let mut out = String::new();

    for pair in pairs {
        if pair.len() > 1 {
            out.push(' ');
            out.push_str( pair.join("+").as_str());
        } else {
            out.push(' ');
            out.push_str(pair.get(0).unwrap());
        }
    }

    out.trim().to_string()
}