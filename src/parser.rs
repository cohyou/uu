use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub enum PExpr {
    PChar(String),
    PAny,
    PNot(Box<PExpr>),
    PSeq(Box<PExpr>, Box<PExpr>),
    POre(Box<PExpr>, Box<PExpr>),
    PRef(Rc<RefCell<HashMap::<String, PExpr>>>, String),
}

impl PExpr {
    pub fn match_pattern(&self, text: &str) -> Option<String> {
        match self {
            PExpr::PChar(t) => {
                if text.starts_with(t) {
                    text.strip_prefix(t).map(|s| s.to_string())
                } else {
                    None
                }
            },
            PExpr::PAny => {
                if text.len() > 0 { Some(text[1..].to_string()) } else { None }
            }
            PExpr::PNot(e) => {
                if e.match_pattern(text).is_some() {
                    None
                } else {
                    Some(text.to_string())
                }
            }
            PExpr::PSeq(e1, e2) => {
                if let Some(r) = e1.match_pattern(text) {
                    e2.match_pattern(&r)
                } else {
                    None
                }
            }
            PExpr::POre(e1, e2) => {
                if let res @ Some(_) = e1.match_pattern(text) {
                    res
                } else {
                    e2.match_pattern(text)
                }
            }
            PExpr::PRef(dict, name) => {
                // println!("{:?}", dict);
                if let Some(e) = dict.borrow().get(name) {
                    e.match_pattern(text)
                } else {
                    panic!("no rule name!");
                }
            }
            // _ => None,
        }
    }
}

impl std::fmt::Debug for PExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PExpr::PChar(t) => {
                f.debug_tuple("Char")
                    .field(t)
                    .finish()
            }
            PExpr::PAny => {
                write!(f, "{:?}", "Any")
            }
            PExpr::PNot(e) => {
                f.debug_tuple("Not")
                .field(&format!("{:?}", e))
                .finish()
            }
            PExpr::PSeq(e1, e2) => {
                f.debug_tuple("Seq")
                .field(&format!("{:?}", e1))
                .field(&format!("{:?}", e2))
                .finish()
            }
            PExpr::POre(e1, e2) => {
                f.debug_tuple("Ore")
                .field(&format!("{:?}", e1))
                .field(&format!("{:?}", e2))
                .finish()
            }
            PExpr::PRef(_, name) => {
                f.debug_tuple("Char")
                    .field(name)
                    .finish()
            }
            // _ => unimplemented!()
        }
        
    }
}

#[test]
fn test_a() {
    let mut m = HashMap::<String, PExpr>::new();
    m.insert("A".to_string(), PExpr::PChar("a".to_string()));
    m.insert("B".to_string(), PExpr::PAny);
    m.insert("C".to_string(), PExpr::PNot(Box::new(PExpr::PChar("a".to_string()))));
    m.insert("D".to_string(), PExpr::PSeq(Box::new(PExpr::PChar("a".to_string())), Box::new(PExpr::PChar("b".to_string()))));
    m.insert("E".to_string(), PExpr::POre(Box::new(PExpr::PChar("a".to_string())), Box::new(PExpr::PChar("b".to_string()))));
    m.insert("F".to_string(), PExpr::POre(Box::new(PExpr::PChar("b".to_string())), Box::new(PExpr::PChar("a".to_string()))));
    println!("{:?}", m);

    println!("A 'b' {:?}", m.get("A").unwrap().match_pattern("b"));
    println!("A 'abcd' {:?}", m.get("A").unwrap().match_pattern("abcd"));
    println!("B '' {:?}", m.get("B").unwrap().match_pattern(""));
    println!("B 'ab' {:?}", m.get("B").unwrap().match_pattern("ab"));
    println!("C 'b' {:?}", m.get("C").unwrap().match_pattern("b"));
    println!("C 'a' {:?}", m.get("C").unwrap().match_pattern("a"));
    println!("D 'ab' {:?}", m.get("D").unwrap().match_pattern("ab"));
    println!("D 'ac' {:?}", m.get("D").unwrap().match_pattern("ac"));
    println!("E 'ab' {:?}", m.get("E").unwrap().match_pattern("ab"));
    println!("F 'ab' {:?}", m.get("F").unwrap().match_pattern("ab"));
}

#[test]
fn test_2() {
    let m_ref = Rc::new(RefCell::new(HashMap::<String, PExpr>::new()));
    m_ref.borrow_mut().insert("B".to_string(), PExpr::PChar("b".to_string()));
    m_ref.borrow_mut().insert("A".to_string(), PExpr::PRef(m_ref.clone(), "B".to_string()));
    
    println!("A 'aa' {:?}", m_ref.borrow().get("A").unwrap().match_pattern("aa"));
    println!("A 'bb' {:?}", m_ref.borrow().get("A").unwrap().match_pattern("bb"));
}

#[test]
fn test_3() {
    let m = Rc::new(RefCell::new(HashMap::<String, PExpr>::new()));
    m.borrow_mut().insert("A".to_string(), PExpr::POre(Box::new(PExpr::PSeq(Box::new(PExpr::PChar("a".to_string())), Box::new(PExpr::PRef(m.clone(), "A".to_string())))), Box::new(PExpr::PChar("".to_string()))));
    println!("A 'aaaabb' {:?}", m.borrow()["A"].match_pattern("aaaabb"));
    println!("A 'bbaaaa' {:?}", m.borrow()["A"].match_pattern("bbaaaa"));
}