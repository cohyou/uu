use std::collections::{HashMap, HashSet};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Rule {
    lhs: String,
    rhs: String,
}

pub fn cpc() {
    // 初期の公理と規則を定義する
    let axioms = vec![
        "a*b = b*a".to_string(),
        "a*c = b*c".to_string(),
        "b*b = c".to_string(),
    ];
    let rules = vec![
        Rule { lhs: "a*c".to_string(), rhs: "c*a".to_string() },
        Rule { lhs: "b*c".to_string(), rhs: "c*b".to_string() },
    ];

    // Knuth-Bendixアルゴリズムで完備化手続きを行う
    let mut kb_pairs = HashSet::new();
    let mut kb_rules = HashMap::new();
    for axiom in axioms {
        kb_pairs.insert((axiom.clone(), "".to_string()));
    }
    for rule in rules {
        kb_pairs.insert((rule.lhs.clone(), rule.rhs.clone()));
    }
    while let Some((lhs, rhs)) = kb_pairs.iter().next().cloned() {
        kb_pairs.remove(&(lhs.clone(), rhs.clone()));
        let mut new_rules = HashSet::new();
        for pair in kb_pairs.iter() {
            if pair.0.ends_with(&lhs) {
                let new_rule = Rule {
                    lhs: format!("{}{}", pair.0[..pair.0.len()-lhs.len()].to_string(), rhs.clone()),
                    rhs: pair.1.clone(),
                };
                if !kb_rules.contains_key(&new_rule.lhs) {
                    new_rules.insert(new_rule.clone());
                }
            }
            if pair.1.starts_with(&rhs) {
                let new_rule = Rule {
                    lhs: pair.0.clone(),
                    rhs: format!("{}{}", lhs.clone(), pair.1[rhs.len()..].to_string()),
                };
                if !kb_rules.contains_key(&new_rule.lhs) {
                    new_rules.insert(new_rule.clone());
                }
            }
        }
        for new_rule in new_rules {
            kb_pairs.insert((new_rule.lhs.clone(), new_rule.rhs.clone()));
            kb_rules.insert(new_rule.lhs, new_rule.rhs);
        }
    }

    // 完備化手続き後の公理と規則を表示する
    println!("Knuth-Bendix完備化手続き後:");
    for (lhs, rhs) in kb_rules.iter() {
        println!("{} = {}", lhs, rhs);
    }
}

#[test]
fn test_cpc() {
    cpc()
}