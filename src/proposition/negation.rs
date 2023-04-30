use crate::proposition::{Proposition, Connective, PropositionError};

#[derive(Debug, PartialEq, Eq)]
pub struct Negation {
    negatum: Box<Proposition>
}

impl Negation {
    pub fn negatum(&self) -> &Box<Proposition> { &self.negatum }
}

impl Connective for Negation {
    fn content(&self) -> Vec<&Proposition> { vec![&self.negatum] }
    fn complexity(&self) -> usize { 1 + self.negatum.complexity() }
    fn symbol(&self) -> Option<char> { Some('~') }
    fn word(&self) -> Option<String> { Some(String::from("not")) }
    fn arity(&self) -> usize { 1 }
    fn to_proposition(self) -> Proposition { Proposition::Negation(self) }

    /// Return a Negation from a Vec containing one Proposition.
    fn from_propositions(mut propositions: Vec<Proposition>) -> Result<Box<Self>, PropositionError> {
        match propositions.len() {
            1 => {
                let negatum = Box::new(propositions.pop().expect("propositions is not empty"));
                Ok(Box::new(Negation { negatum }))
            },
            n => Err(PropositionError::IncorrectNumberOfArguments(format!("Negation requires 1 subproposition, not {}", n)))
        }
    }
    fn from_connectives<T: Connective>(mut connectives: Vec<T>) -> Result<Box<Self>, PropositionError> {
        match connectives.len() {
            1 => {
                let proposition = connectives.pop().expect("connectives.len() == 1").to_proposition();
                let negatum = Box::new(proposition);
                Ok(Box::new(Negation { negatum }))
            },
            n => Err(PropositionError::IncorrectNumberOfArguments(format!("Negation requires 1 subproposition, not {}", n)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_complexity() {
        let atom = Proposition::Atom(String::from("hello"));
        let one: Box<Negation> = Negation::from_propositions(vec![atom]).unwrap();
        assert_eq!(1, one.complexity());
        let two = Negation::from_connectives(vec![*one]).unwrap();
        assert_eq!(2, two.complexity());
    }
}
