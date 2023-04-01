// alfabet:
//   + - 0 x y z
// eqs:
//  0 + x = x
//  (-x) + x = 0
//  (x + y) + z = x + (y + z)

// 公理
// 1.公理
// E内に定義されていれば成り立つ
// 2.代入規則
// s=t ならば σs=σt σ: T->Tは任意の代入 
// 3.置換規則
// s1=t1, ..., s[n]=t[n] ならば f(s[1], ..., s[n])=f(t[1], ..., t[n])
// 4.反射律
// t=t
// 5.交換律
// s=t ならば t=s
// 6.推移律
// t[1]=t[2], t[2]=t[3] ならば t[1]=t[3]

// R := {analyze(s, t, >) | s=t ∈ E}
// CP := R の危険対の集合
// while CP != 0 do
//   begin
//     任意の対<p,q>をCPから選ぶ
//     CP := CP-{<p,q>}
//     p,qのRに関しての正規形bar(p), bar(q)を求める
//     if bar(p) != bar(q)
//     then begin
//            <α,β> := analyze(bar(p), bar(p), >)
//            R := R ∪ {<α,β>}
//            CP := CP ∪r∈R (α->βとrの危険対の集合)
//          end
//   end
// return R

// analyze
// 入力：s,t∈T
//       厳格半順序 >
// 出力：<α,β>(<s,t>あるいは<t,s>)
// 注意：CPCの手続きを終了させることがある
// if s>t
// then <α,β>=<s,t>
// else if t>s
//      then <α,β>:=<t,s>
//      else CPC手続きを終了させる（完備化は失敗である）
// return <α,β>
// analyze入力となる2つの項s,tに厳格半順序による向きをつけ、
// 順序対<s, t>(s > tのとき)
// 順序対<t, s>(t > sのとき)
// を返すものである。どちらでもないときはCPC手続きが失敗する

// 厳格半順序 strict partial order
// 推移的で、非反射的であるもの。[]
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