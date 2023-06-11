use lazy_static::lazy_static;
use std::cmp;
use std::fmt::{Display, Formatter};
use regex::Regex;

pub enum Proposition {
    Atom(String),
    Negation(Box<Proposition>),
    Conditional(Box<Proposition>, Box<Proposition>),
    Conjunction(Box<Proposition>, Box<Proposition>),
    Disjunction(Box<Proposition>, Box<Proposition>),
    Existential(String, Box<Proposition>),
    Universal(String, Box<Proposition>)
}

impl Proposition {
    pub fn complexity(&self) -> usize {
        match self {
            Proposition::Atom(_) => 0,
            Proposition::Negation(negatum) => 1 + negatum.complexity(),
            Proposition::Conditional(left, right) => 1 + cmp::max(left.complexity(), right.complexity()),
            Proposition::Conjunction(left, right) => 1 + cmp::max(left.complexity(), right.complexity()),
            Proposition::Disjunction(left, right) => 1 + cmp::max(left.complexity(), right.complexity()),
            Proposition::Existential(_, content) => 1 + content.complexity(),
            Proposition::Universal(_, content) => 1 + content.complexity(),
        }
    }
    pub fn names(&self) -> Vec<String> {
        match self {
            Proposition::Atom(atom) => get_names(atom),
            Proposition::Negation(negatum) => negatum.names(),
            Proposition::Conditional(left, right) => {
                let mut names: Vec<String> = Vec::new();
                for name in left.names() { names.push(name); }
                for name in right.names() { names.push(name); }
                names
            },
            Proposition::Conjunction(left, right) => {
                let mut names: Vec<String> = Vec::new();
                for name in left.names() { names.push(name); }
                for name in right.names() { names.push(name); }
                names
            },
            Proposition::Disjunction(left, right) => {
                let mut names: Vec<String> = Vec::new();
                for name in left.names() { names.push(name); }
                for name in right.names() { names.push(name); }
                names
            },
            Proposition::Existential(_, content) => content.names(),
            Proposition::Universal(_, content) => content.names()
        }
    }

    pub(crate) fn instantiate(&mut self, var: String, name: String) {
        todo!()
    }
}

impl Clone for Proposition {
    fn clone(&self) -> Self {
        match self {
            Proposition::Atom(atom) => Proposition::Atom(String::from(atom)),
            Proposition::Negation(negatum) => Proposition::Negation(negatum.clone()),
            Proposition::Conditional(left, right) => Proposition::Conditional(left.clone(), right.clone()),
            Proposition::Conjunction(left, right) => Proposition::Conjunction(left.clone(), right.clone()),
            Proposition::Disjunction(left, right) => Proposition::Disjunction(left.clone(), right.clone()),
            Proposition::Existential(var, content) => Proposition::Existential(String::from(var), content.clone()),
            Proposition::Universal(var, content) => Proposition::Universal(String::from(var), content.clone()),
        }
    }
}

impl Display for Proposition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Proposition::Atom(atom) => write!(f, "{}", atom),
            Proposition::Negation(negatum) => write!(f, "~({})", negatum),
            Proposition::Conditional(left, right) => write!(f, "({} > {})", left, right),
            Proposition::Conjunction(left, right) => write!(f, "({} & {})", left, right),
            Proposition::Disjunction(left, right) => write!(f, "({} v {})", left, right),
            Proposition::Existential(var, content) => write!(f, "∃{}({})", var, content),
            Proposition::Universal(var, content) => write!(f, "∀{}({})", var, content),
        }
    }
}


fn get_names(string: &String) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    lazy_static! {
        // match sets of two or more contiguous lowercase letters between angle brackets
        static ref RE: Regex = Regex::new(r"\<([a-z]{2,})\>").unwrap();
    }
    for capture in RE.captures_iter(string) {
        names.push(capture[1].to_string());    }
    names
}