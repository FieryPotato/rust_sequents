mod create;

use lazy_static::lazy_static;
use std::cmp;
use std::fmt::{Display, Formatter};
use regex::Regex;

#[derive(Debug, PartialEq)]
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
            Proposition::Existential(_, predicate) => 1 + predicate.complexity(),
            Proposition::Universal(_, predicate) => 1 + predicate.complexity(),
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
            Proposition::Existential(_, predicate) => predicate.names(),
            Proposition::Universal(_, predicate) => predicate.names()
        }
    }

    pub fn variables(&self) -> Vec<String> {
        match self {
            Proposition::Atom(atom) => get_variables(atom),
            Proposition::Negation(negatum) => negatum.variables(),
            Proposition::Conditional(left, right) => {
                let mut variables: Vec<String> = Vec::new();
                for variable in left.variables() { variables.push(variable); }
                for variable in right.variables() { variables.push(variable); }
                variables
            },
            Proposition::Conjunction(left, right) => {
                let mut variables: Vec<String> = Vec::new();
                for variable in left.variables() { variables.push(variable); }
                for variable in right.variables() { variables.push(variable); }
                variables
            },
            Proposition::Disjunction(left, right) => {
                let mut variables: Vec<String> = Vec::new();
                for variable in left.variables() { variables.push(variable); }
                for variable in right.variables() { variables.push(variable); }
                variables
            },
            Proposition::Existential(_, predicate) => predicate.variables(),
            Proposition::Universal(_, predicate) => predicate.variables()
        }
    }

    pub fn instantiate(&mut self, var: &String, name: &String) {
        match self {
            Self::Atom(ref mut atom) => set_name(atom, var, name),
            Self::Negation(ref mut negatum) => negatum.instantiate(var, name),
            Self::Conditional(ref mut left, ref mut right) => {
                left.instantiate(var, name);
                right.instantiate(var, name);
            },
            Self::Conjunction(ref mut left, ref mut right) => {
                left.instantiate(var, name);
                right.instantiate(var, name);
            },
            Self::Disjunction(ref mut left, ref mut right) => {
                left.instantiate(var, name);
                right.instantiate(var, name);
            },
            Self::Existential(_ , ref mut predicate) => predicate.instantiate(var, name),
            Self::Universal(_ , ref mut predicate) => predicate.instantiate(var, name),
        }
    }

    /// Return a vec containing references to the propositional content of self. Atoms return references
    /// to themselves (and not the strings they contain.)
    pub fn content(&self) -> Vec<&Proposition> {
        match self {
            // Atoms' content is just themselves. We can't reasonably return a mixed vec full of
            // both strings and propositions, so we don't.
            Self::Atom(_) => vec![&self],
            // &* on each contained proposition because we want references to
            // the objects, not to their boxes
            Self::Negation(negatum) => vec![&*negatum],
            Self::Conditional(left, right) => vec![&*left, &*right],
            Self::Conjunction(left, right) => vec![&*left, &*right],
            Self::Disjunction(left, right) => vec![&*left, &*right],
            Self::Existential(_, predicate) => vec![&*predicate],
            Self::Universal(_, predicate) => vec![&*predicate],
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
            Proposition::Existential(var, predicate) => Proposition::Existential(String::from(var), predicate.clone()),
            Proposition::Universal(var, predicate) => Proposition::Universal(String::from(var), predicate.clone()),
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
            Proposition::Existential(var, predicate) => write!(f, "∃{}({})", var, predicate),
            Proposition::Universal(var, predicate) => write!(f, "∀{}({})", var, predicate),
        }
    }
}

fn get_variables(string: &String) -> Vec<String> {
    let mut variables: Vec<String> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\<([a-z]{1})\>").unwrap();
    }
    for capture in RE.captures_iter(string) {
        variables.push(capture[1].to_string());    }
    variables
}

fn get_names(string: &String) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\<([a-z]{2,})\>").unwrap();
    }
    for capture in RE.captures_iter(string) {
        names.push(capture[1].to_string());    }
    names
}

fn set_name(string: &mut String, var: &String, name: &String) {
    let mut name_re = Regex::new(format!("<({var})>").as_str()).unwrap();
    *string = name_re.replace_all(string.as_str(), format!("<{name}>")).to_string();
}


#[cfg(test)]
mod test {
    use crate::proposition::Proposition;

    #[test]
    fn test_atomic_instantiate() {
        let string = String::from("<a> is on the mat");
        let mut atom = Proposition::Atom(string);
        let var = "a".to_string();
        let name = "kitty".to_string();

        atom.instantiate(&var, &name);

        assert_eq!(atom.content(), vec![&Proposition::Atom(String::from("<kitty> is on the mat"))]);
    }

    #[test]
    fn test_negation_instantiate() {
        let string: String = String::from("<a> is on the mat");
        let atom = Proposition::Atom(string);
        let mut negation = Proposition::Negation(Box::new(atom));
        let var = "a".to_string();
        let name = "kitty".to_string();

        negation.instantiate(&var, &name);

        assert_eq!(negation.content(),
                   vec![&Proposition::Atom(String::from("<kitty> is on the mat"))]
        )
    }

    #[test]
    fn test_conditional_instantiate() {
        let left = String::from("<a> is a cat");
        let right = String::from("<a> is on the mat");

        let left = Proposition::Atom(left);
        let right = Proposition::Atom(right);

        let mut conditional = Proposition::Conditional(
            Box::new(left),
            Box::new(right)
        );

        let var = "a".to_string();
        let name = "kitty".to_string();

        conditional.instantiate(&var, &name);

        assert_eq!(
            conditional.content(),
            vec![
                &Proposition::Atom(String::from("<kitty> is a cat")),
                &Proposition::Atom(String::from("<kitty> is on the mat"))
            ]
        )
    }

    #[test]
    fn test_conjunction_instantiate() {
        let left = String::from("<a> is a cat");
        let right = String::from("<a> is on the mat");

        let left = Proposition::Atom(left);
        let right = Proposition::Atom(right);

        let mut conjunction = Proposition::Conjunction(
            Box::new(left),
            Box::new(right)
        );

        let var = "a".to_string();
        let name = "kitty".to_string();

        conjunction.instantiate(&var, &name);

        assert_eq!(
            conjunction.content(),
            vec![
                &Proposition::Atom(String::from("<kitty> is a cat")),
                &Proposition::Atom(String::from("<kitty> is on the mat"))
            ]
        )
    }

    #[test]
    fn test_disjunction_instantiate() {
        let left = String::from("<a> is a cat");
        let right = String::from("<a> is on the mat");

        let left = Proposition::Atom(left);
        let right = Proposition::Atom(right);

        let mut disjunction = Proposition::Disjunction(
            Box::new(left),
            Box::new(right)
        );

        let var = "a".to_string();
        let name = "kitty".to_string();

        disjunction.instantiate(&var, &name);

        assert_eq!(
            disjunction.content(),
            vec![
                &Proposition::Atom(String::from("<kitty> is a cat")),
                &Proposition::Atom(String::from("<kitty> is on the mat"))
            ]
        )
    }

    #[test]
    fn test_instantiate_existential() {
        let predicate = String::from("<a> is on <bb>");
        let atom = Proposition::Atom(predicate);
        let mut existential = Proposition::Existential(
            String::from("a"), Box::new(atom)
        );

        let var = "bb".to_string();
        let name = "the mat".to_string();

        existential.instantiate(&var, &name);

        assert_eq!(
            existential.content(),
            vec![
                &Proposition::Atom(String::from("<a> is on <the mat>"))
            ]
        )
    }


    #[test]
    fn test_instantiate_universal() {
        let predicate = String::from("<a> is on <b>");
        let atom = Proposition::Atom(predicate);
        let mut universal = Proposition::Universal(
            String::from("a"), Box::new(atom)
        );

        let var = "b".to_string();
        let name = "the mat".to_string();

        universal.instantiate(&var, &name);

        assert_eq!(
            universal.content(),
            vec![
                &Proposition::Atom(String::from("<a> is on <the mat>"))
            ]
        )
    }
}
