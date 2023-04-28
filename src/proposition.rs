use std::cmp;


/// The Proposition Enum


pub enum Proposition {
    Atom(Atom),
    Negation(Negation),
    Conjunction(Conjunction),
    Conditional(Conditional),
    Disjunction(Disjunction),
    Universal(Universal),
    Existential(Existential)
}
impl Proposition {
    pub(crate) fn complexity(&self) -> usize {
        match self {
            Self::Atom(_) => 0,
            Self::Negation(n) => n.complexity(),
            Self::Conjunction(c) => cmp::max(c.left().complexity(), c.right().complexity()),
            Self::Conditional(c) => cmp::max(c.left().complexity(), c.right().complexity()),
            Self::Disjunction(d) => cmp::max(d.left().complexity(), d.right().complexity()),
            Self::Universal(u) => u.complexity(),
            Self::Existential(e) => e.complexity()
        }
    }
}


/// Propositions and their Implementations


pub struct Atom {
    content: String
}
impl Atom {
    pub fn complexity(&self) -> usize { 0 }
    pub fn content(&self) -> &String { &self.content }
}

pub struct Negation {
    negatum: Box<Proposition>
}
impl Negation {
    pub fn complexity(&self) -> usize { self.negatum.complexity() }
    pub fn negatum(&self) -> &Box<Proposition> { &self.negatum }
}

pub struct Conjunction {
    left: Box<Proposition>,
    right: Box<Proposition>
}
impl Conjunction {
}
impl BinaryConnective for Conjunction {
    fn left(&self) -> &Box<Proposition> { &self.left }
    fn right(&self) -> &Box<Proposition> { &self.right }
}

pub struct Conditional {
    left: Box<Proposition>,
    right: Box<Proposition>
}
impl Conditional {
}
impl BinaryConnective for Conditional {
    fn left(&self) -> &Box<Proposition> { &self.left }
    fn right(&self) -> &Box<Proposition> { &self.right }
}

pub struct Disjunction {
    left: Box<Proposition>,
    right: Box<Proposition>
}
impl Disjunction {
}
impl BinaryConnective for Disjunction {
    fn left(&self) -> &Box<Proposition> { &self.left }
    fn right(&self) -> &Box<Proposition> { &self.right }
}

pub struct Universal {
    variable: String,
    predicate: Box<Proposition>
}
impl Universal { }
impl Quantifier for Universal {
    fn variable(&self) -> &String { &self.variable }
    fn predicate(&self) -> &Box<Proposition> { &self.predicate }
}

pub struct Existential {
    variable: String,
    predicate: Box<Proposition>
}
impl Existential {
}
impl Quantifier for Existential {
    fn variable(&self) -> &String { &self.variable }
    fn predicate(&self) -> &Box<Proposition> { &self.predicate }
}


/// Traits

pub trait BinaryConnective {
    fn left(&self) -> &Box<Proposition>;
    fn right(&self) -> &Box<Proposition>;
    fn complexity(&self) -> usize {
        1 + cmp::max(self.left().complexity(), self.right().complexity())
    }
}

pub trait Quantifier {
    fn variable(&self) -> &String;
    fn predicate(&self) -> &Box<Proposition>;
    fn complexity(&self) -> usize {
        1 + self.predicate().complexity()
    }
}

