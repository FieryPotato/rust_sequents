use std::cmp;
use std::fmt::{Display, Formatter};

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