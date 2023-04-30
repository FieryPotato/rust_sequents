mod conditional;
mod conjunction;
mod create;
mod disjunction;
mod negation;

use crate::proposition::conditional::Conditional;
use crate::proposition::conjunction::Conjunction;
use crate::proposition::disjunction::Disjunction;
use crate::proposition::negation::Negation;

#[derive(Debug, PartialEq, Eq)]
pub enum Proposition {
    Atom(String),
    Negation(Negation),
    Conjunction(Conjunction),
    Conditional(Conditional),
    Disjunction(Disjunction),
    // Universal(Universal),
    // Existential(Existential)
}

impl Proposition {
    pub fn complexity(&self) -> usize {
        match self {
            Self::Atom(_) => 0,
            Self::Negation(n) => n.complexity(),
            Self::Conjunction(c) => c.complexity(),
            Self::Conditional(c) => c.complexity(),
            Self::Disjunction(d) => d.complexity(),
            // Self::Universal(u) => u.complexity(),
            // Self::Existential(e) => e.complexity()
        }
    }

    pub fn symbol(&self) -> Option<char> {
        match self {
            Self::Atom(_) => None,
            Self::Negation(n) => n.symbol(),
            Self::Conjunction(c) => c.symbol(),
            Self::Conditional(c) => c.symbol(),
            Self::Disjunction(d) => d.symbol(),
        }
    }

    pub fn word(&self) -> Option<String> {
        match self {
            Self::Atom(_) => None,
            Self::Negation(n) => n.word(),
            Self::Conjunction(c) => c.word(),
            Self::Conditional(c) => c.word(),
            Self::Disjunction(d) => d.word(),
        }
    }

    /// Return a proposition from input string. Each sub wff must be
    /// encapsulated by parentheses, but the outermost pair may be omitted.
    /// Order is always preserved. Improper bracketing may result in the
    /// string being converted into one big Proposition::Atom.
    /// Eg. "(the cat is on the mat & the rat wears a hat) -> the bat is a brat"
    /// becomes: Conditional(Conjunction(cat..., rat...), bat...)
    pub fn from_string(string: String) -> Result<Self, PropositionError> {
        create::find_connective(string)
    }
}

fn is_negation_str(string: &str) -> bool { string == "~" || string == "not" }
fn is_conjunction_str(string: &str) -> bool { string == "&" || string == "and" }
fn is_conditional_str(string: &str) -> bool { string == ">" || string == "implies" }
fn is_disjunction_str(string: &str) -> bool { string == "v" || string == "or" }

pub trait Connective {
    fn content(&self) -> Vec<&Proposition>;
    fn complexity(&self) -> usize;
    fn symbol(&self) -> Option<char>;
    fn word(&self) -> Option<String>;
    fn arity(&self) -> usize;
    fn to_proposition(self) -> Proposition;
    fn from_propositions(propositions: Vec<Proposition>) -> Result<Box<Self>, PropositionError>;
    fn from_connectives<T: Connective>(connectives: Vec<T>) -> Result<Box<Self>, PropositionError>;
}

#[derive(Debug)]
pub enum PropositionError {
    IncorrectNumberOfArguments(String),
    NoConnectiveFound,
    InvalidConnective,
}
