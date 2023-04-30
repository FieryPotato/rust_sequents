use std::cmp;
use crate::proposition::{Connective, Proposition, PropositionError};


#[derive(Debug, PartialEq, Eq)]
pub struct Disjunction {
    left: Box<Proposition>,
    right: Box<Proposition>
}

impl Disjunction {
    pub fn left(&self) -> &Box<Proposition> { &self.left }

    pub fn right(&self) -> &Box<Proposition> { &self.right }
}

impl Connective for Disjunction {
    fn content(&self) -> Vec<&Proposition> { vec![&self.left, &self.right] }

    fn complexity(&self) -> usize {
        1 + cmp::max(self.left().complexity(), self.right().complexity())
    }

    fn symbol(&self) -> Option<char> { Some('v') }

    fn word(&self) -> Option<String> { Some(String::from("or")) }

    fn arity(&self) -> usize { 2 }

    fn to_proposition(self) -> Proposition { Proposition::Disjunction(self) }

    fn from_propositions(mut propositions: Vec<Proposition>) -> Result<Box<Self>, PropositionError> {
        match propositions.len() {
            2 =>  {
                let left = Box::new(propositions.pop().expect("propositions.len() == 2"));
                let right = Box::new(propositions.pop().expect("propositions.len() == 1"));
                Ok(Box::new(Disjunction { left, right }))
            },
            n => Err(PropositionError::IncorrectNumberOfArguments(format!("Disjunction requires two subpropositions, not {}", n)))
        }
    }

    fn from_connectives<T: Connective>(mut connectives: Vec<T>) -> Result<Box<Self>, PropositionError> {
        match connectives.len() {
            2 => {
                let left = Box::new(connectives.pop().expect("connectives.len() == 2").to_proposition());
                let right = Box::new(connectives.pop().expect("connectives.len() == 1").to_proposition());
                Ok(Box::new(Disjunction { left, right }))
            },
            n => Err(PropositionError::IncorrectNumberOfArguments(format!("Disjunction requires two subpropositions, not {}", n)))
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_complexity() {
        // atom x atom
        let cat = Proposition::Atom(String::from("the cat is on the mat."));
        let mat = Proposition::Atom(String::from("the mat is under the cat."));
        let cat_mat: Box<Disjunction> = Disjunction::from_propositions(vec![cat, mat]).unwrap();
        assert_eq!(1, cat_mat.complexity());

        // wrapping it in a Proposition
        let cat_mat: Proposition = Proposition::Disjunction(*cat_mat);
        assert_eq!(1, cat_mat.complexity());

        // conjunction x conjunction
        let hat = Proposition::Atom(String::from("the hat is on the rat."));
        let rat = Proposition::Atom(String::from("the rat is wearing the hat."));
        let hat_rat: Box<Disjunction> = Disjunction::from_propositions(vec![hat, rat]).unwrap();
        let hat_rat: Proposition = Proposition::Disjunction(*hat_rat);
        let one_x_one: Box<Disjunction> = Disjunction::from_propositions(vec![hat_rat, cat_mat]).unwrap();
        assert_eq!(2, one_x_one.complexity());
        let one_x_one: Proposition = Proposition::Disjunction(*one_x_one);
        assert_eq!(2, one_x_one.complexity());

        // atom x conjunction
        let pat = Proposition::Atom(String::from("Pat has a bat."));
        let two_x_one: Box<Disjunction> = Disjunction::from_propositions(vec![one_x_one, pat]).unwrap();
        assert_eq!(3, two_x_one.complexity());
    }
}


