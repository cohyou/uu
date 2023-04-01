#[derive(Clone, PartialEq, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum State {
    WesternAustralia,
    NorthernTerritory,
    SouthAustralia,
    Queensland,
    NewSouthWales,
    Victoria,
    Tasmania,
}



pub struct CSP {
    variables: Vec<Variable>,
    constraints: Vec<Constraint>,
}
#[derive(PartialEq, Debug)]
pub struct Solution(Assignment);
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Variable(State);

#[derive(Clone, PartialEq, Debug)]
struct Value(Color);
type Constraint = (State, State);
use std::collections::HashMap;
type Assignment = HashMap<Variable, Value>;

pub fn backtracking_search(csp: CSP) -> Option<Solution> {
    recursive_backtracking(HashMap::new(), &csp)
}

fn recursive_backtracking(mut assignment: Assignment, csp: &CSP) -> Option<Solution> {
    if is_complete(&assignment) { return Some(Solution(assignment.clone())); }
    let var = select_unassigned_variable(&csp.variables, &assignment, &csp);
    for value in order_domain_values(&var, &assignment, csp) {
        if is_consistent(&var, &value, &assignment, &csp.constraints) {
            assignment.insert(var.clone(), value);
            let result = recursive_backtracking(assignment.clone(), csp);
            if result.is_some() { return result; }
            assignment.remove(&var);
        }
    }
    None
}

fn select_unassigned_variable(variables: &Vec<Variable>, assignment: &Assignment, _csp: &CSP) -> Variable {
    // もっとも単純な変数の選び方。まだ割り当てが存在しない最初の変数を持ってくる。
    for v in variables {
        if !assignment.contains_key(v) {
            return v.clone();
        }
    }
    unreachable!()
}

fn order_domain_values(_var: &Variable, _assignment: &Assignment, _csp: &CSP) -> Vec<Value> {
    use Color::*;
    // もっとも単純な値の選び方。特に何も考えずに値の候補となるvectorを渡す。
    vec![Value(Red), Value(Green), Value(Blue)]
}

fn is_consistent(variable: &Variable, value: &Value, assignment: &Assignment, constraints: &Vec<Constraint>) -> bool {
    // variable = valueの組み合わせが、すでに存在する割り当てと衝突しないかどうか
    for c in constraints {
        if variable.0 == c.0 {
            if assignment.contains_key(&Variable(c.1.clone())) {
                if &assignment[&Variable(c.1.clone())] == value {
                    // 衝突
                    return false;
                }
            }
        }
        if variable.0 == c.1 {
            if assignment.contains_key(&Variable(c.0.clone())) {
                if &assignment[&Variable(c.0.clone())] == value {
                    // 衝突
                    return false;
                }
            }
        }
    }
    true
}

fn is_complete(assignment: &Assignment) -> bool { assignment.len() == 7 }

#[test]
fn test() {
    use State::*;

    let variables = vec![
        Variable(WesternAustralia),
        Variable(NorthernTerritory),
        Variable(SouthAustralia),
        Variable(Queensland),
        Variable(NewSouthWales),
        Variable(Victoria),
        Variable(Tasmania),
    ];
    let constraints = vec![
        (WesternAustralia, NorthernTerritory),
        (WesternAustralia, SouthAustralia),
        (NorthernTerritory, SouthAustralia),
        (NorthernTerritory, Queensland),
        (SouthAustralia, Queensland),
        (SouthAustralia, NewSouthWales),
        (SouthAustralia, Victoria),
        (Queensland, NewSouthWales),
        (NewSouthWales, Victoria),
    ];
    let csp = CSP {
        variables: variables,
        constraints: constraints,
    };
    let res = backtracking_search(csp);
    println!("{:#?}", res);
    assert_eq!(res, None);
}