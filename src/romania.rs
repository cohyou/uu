// use std::rc::Rc;

#[derive(PartialEq, Clone, Copy, Debug)]
enum RomaniaCity {
    Arad,
    Sibiu,
    Timisoara,
    Zerind,
    Bucharest,
    Oradea,
    Lugoj,
    Mehadia,
    Dobreta,
    Fagaras,
    RimnicuVilcea,
    Craiova,
    Pitesti,
    Giurgiu,
    Urziceni,
    Vaslui,
    Iasi,
    Neamt,
    Hirsova,
    Eforie,
}
impl Default for RomaniaCity {
    fn default() -> Self { Arad }
}

use RomaniaCity::*;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Go(RomaniaCity);

#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub struct In(RomaniaCity);

pub struct Percept(In);

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Action {
    pub go: Go,
    pub cost: Cost,
}

pub struct Goal(In);

pub struct Problem {
    initial_state: In,
    _action: Action,
    goal_test: Box<dyn FnMut(&In) -> bool>,
}
type Cost = u16;
impl Problem {
    fn _path_cost() -> Cost { unimplemented!( )}
}

pub fn go_agent() {
    let initial_percept = Percept(In(Arad));
    simple_problem_solving_agent(initial_percept);
}

fn simple_problem_solving_agent(percept: Percept) -> Vec<Action> {
    // an action sequence, initially empty
    let mut seq = vec![Action::default()];
    // some description of the current world state
    let mut state = percept.0;

    state = update_state(&state, &percept);

    if seq.is_empty() {
        // a goal, initially null
        let goal = formulate_goal(&state);
        // a problem formulation
        let mut problem = formulate_problem(state, &goal);
        seq = search(&mut problem);
    }
    let _action = seq.pop();
    seq
}

fn update_state(_state: &In, percept: &Percept) -> In { percept.0 }

fn formulate_goal(_state: &In) -> Goal { Goal(In(Bucharest)) }

fn formulate_problem(state: In, goal: &Goal) -> Problem {
    let goal_state = goal.0;
    Problem {
        initial_state: state,
        _action: Action::default(),
        goal_test: Box::new(move |in_now| in_now == &goal_state)
    }
}

fn search(problem: &mut Problem) -> Vec<Action> {
    tree_search(problem, &Strategy);
    unimplemented!()
}


struct Strategy;
#[derive(PartialEq, Debug)]
struct Solution(Node);
#[derive(Default, Clone, PartialEq, Debug)]
struct Node {
    pub state: In,
    parent: Option<Box<Node>>,
    action: Action,
    path_cost: Cost,
    pub depth: usize,
}
impl Node {
    fn new() -> Self {
        Node::default()
    }
    fn with_state(state: In) -> Self {
        Node {
            state: state,
            parent: Option::default(),
            action: Action::default(),
            path_cost: Cost::default(),
            depth: usize::default(),
        }
    }
}
fn tree_search(problem: &mut Problem, _strategy: &Strategy) -> Option<Solution> {
    tree_search_internal(problem, vec![])
    // let node = Node::default();
    // // initiallize the search tree using the initial state of problem

    // loop {
    //     // if there are no candidates for expansion
    //     if true { return None; }
    //     // choose a leaf node for expansion according to stategy
    //     // if node.0.iter().any::<FnMut(&In) -> bool>(problem.goal_test.into()) {
    //     if (problem.goal_test)(&node.state) {
    //         return Some(Solution(node));
    //     } else {
    //         // expand the node and add the resulting nodes to the search tree
    //     }
    // }
}

type Fringe = Vec<Node>;
// impl Fringe {
//     pub fn insert(&self, node: Node) {}
// }
fn tree_search_internal(problem: &mut Problem, mut fringe: Fringe) -> Option<Solution> {
    fringe.insert(0, make_node(problem.initial_state));
    loop {
        if fringe.is_empty() { return None; }
        let node = fringe.pop();
        if (problem.goal_test)(&node.as_ref().unwrap().state) {
            return Some(Solution(node.unwrap()));
        }
        fringe.extend(expand(node.unwrap(), problem));
    }
}

fn expand(node: Node, _problem: &mut Problem) -> Vec<Node> {
    let mut successors = vec![];
    for (action, result) in successor(node.state) {
        let mut s = Node::new();
        s.state = In(result);
        s.parent = Some(Box::new(node.clone()));
        s.action = action.clone();
        s.path_cost = &node.clone().path_cost + step_cost(&node, &action, &s);
        s.depth = &node.clone().depth + 1;
        successors.push(s);
        // println!("{:?}", successors);
    }
    successors
}

fn make_node(state: In) -> Node { Node::with_state(state) }

fn step_cost(_node: &Node, action: &Action, _s: &Node) -> Cost {
    action.cost
}

fn successor(in_now: In) -> Vec<(Action, RomaniaCity)> {
    let city_routes = city_routes();
    let mut res = vec![];
    for (from, to, cost) in city_routes.iter() {
        if &in_now.0 == from {
            res.push((Action{go: Go(to.clone()), cost: cost.clone()}, to.clone()));
        }
        if &in_now.0 == to {
            res.push((Action{go: Go(from.clone()), cost: cost.clone()}, from.clone()));
        }
    }
    res
}

fn city_routes() -> Vec<(RomaniaCity, RomaniaCity, Cost)> {
    vec![
        (Arad, Zerind, 75),
        (Arad, Timisoara, 118),
        (Arad, Sibiu, 140),
        (Zerind, Oradea, 71),
        (Timisoara, Lugoj, 111),
        (Sibiu, Oradea, 151),
        (Sibiu, Fagaras, 99),
        (Sibiu, RimnicuVilcea, 80),
        (Lugoj, Mehadia, 70),
        (Fagaras, Bucharest, 211),
        (RimnicuVilcea, Craiova, 146),
        (RimnicuVilcea, Pitesti, 97),
        (Mehadia, Dobreta, 75),
        (Bucharest, Giurgiu, 90),
        (Bucharest, Urziceni, 85),
        (Craiova, Pitesti, 138),
        (Pitesti, Bucharest, 101),
        (Dobreta, Craiova, 120),
        (Urziceni, Vaslui, 142),
        (Urziceni, Hirsova, 98),
        (Vaslui, Iasi, 92),
        (Iasi, Neamt, 87),
        (Hirsova, Eforie, 86),
    ]
}

#[test]
fn test_routes() {
    assert_eq!(city_routes().len(), 23);
}

#[test]
fn test_tree_search_internal() {
    let goal = Goal(In(Bucharest));
    let mut problem = formulate_problem(In(Arad), &goal);
    let fringe = vec![];
    let sol = tree_search_internal(&mut problem, fringe);
    println!("{:#?}", &sol);
    if let Some(sol) = sol {
        assert_eq!(sol.0.state, In(Bucharest));
        assert_eq!(sol.0.path_cost, 418);
        assert_eq!(sol.0.depth, 4);   
    }
}

#[test]
fn test_successor() {
    // let action = Action { go: Go(Arad), cost: 0 };
    // assert_eq!(successor(In(Arad)), vec![(action, Arad)]);
    assert_eq!(successor(In(Timisoara)), vec![(Action::default(), Arad)]);
}

#[test]
fn test_expand() {
    let goal = Goal(In(Bucharest));
    let mut problem = formulate_problem(In(Arad), &goal);
    let node = Node::with_state(In(Arad));
    assert_eq!(expand(node, &mut problem), vec![]);
}

#[test]
fn test_formulate_problem() {
    let goal = Goal(In(Bucharest));
    formulate_problem(In(Arad), &goal);
}