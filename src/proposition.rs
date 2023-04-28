enum Proposition {
    Atom(String),
    Negation(Box<Proposition>),
    Conjunction(Box<Proposition>, Box<Proposition>),
    Conditional(Box<Proposition>, Box<Proposition>),
    Disjunction(Box<Proposition>, Box<Proposition>),
    Universal(String, Box<Proposition>),
    Existential(String, Box<Proposition>)
}
