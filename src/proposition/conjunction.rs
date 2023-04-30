use std::cmp;
use crate::proposition::{Connective, Proposition, PropositionError};


#[derive(Debug, PartialEq, Eq)]
pub struct Conjunction {
    left: Box<Proposition>,
    right: Box<Proposition>
}

impl Conjunction {
    pub fn left(&self) -> &Box<Proposition> { &self.left }

    pub fn right(&self) -> &Box<Proposition> { &self.right }
}

impl Connective for Conjunction {
    fn content(&self) -> Vec<&Proposition> {
        vec![&self.left, &self.right]
    }

    fn complexity(&self) -> usize {
        1 + cmp::max(self.left().complexity(), self.right().complexity())
    }

    fn symbol(&self) -> Option<char> { Some('&') }

    fn word(&self) -> Option<String> { Some(String::from("and")) }

    fn arity(&self) -> usize { 2 }

    fn to_proposition(self) -> Proposition { Proposition::Conjunction(self) }

    fn from_propositions(mut propositions: Vec<Proposition>) -> Result<Box<Self>, PropositionError>{
        match propositions.len() {
            2 =>  {
                let right = Box::new(propositions.pop().expect("propositions.len() == 2"));
                let left = Box::new(propositions.pop().expect("propositions.len() == 1"));
                Ok(Box::new(Conjunction { left, right }))
            },
            n => Err(PropositionError::IncorrectNumberOfArguments(format!("Conjunction requires 2 subpropositions, not {}", n)))
        }
    }

    fn from_connectives<T: Connective>(mut connectives: Vec<T>) -> Result<Box<Self>, PropositionError> {
        match connectives.len() {
            2 => {
                let right = Box::new(connectives.pop().expect("connectives.len() == 1").to_proposition());
                let left = Box::new(connectives.pop().expect("connectives.len() == 1").to_proposition());
                Ok(Box::new(Conjunction{ left, right }))
            },
            n => Err(PropositionError::IncorrectNumberOfArguments(format!("Conjunction requires 2 subpropositions, not {}", n)))
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
        let cat_mat: Box<Conjunction> = Conjunction::from_propositions(vec![cat, mat]).unwrap();
        assert_eq!(1, cat_mat.complexity());

        // wrapping it in a Proposition
        let cat_mat: Proposition = Proposition::Conjunction(*cat_mat);
        assert_eq!(1, cat_mat.complexity());

        // conjunction x conjunction
        let hat = Proposition::Atom(String::from("the hat is on the rat."));
        let rat = Proposition::Atom(String::from("the rat is wearing the hat."));
        let hat_rat: Box<Conjunction> = Conjunction::from_propositions(vec![hat, rat]).unwrap();
        let hat_rat: Proposition = Proposition::Conjunction(*hat_rat);
        let one_x_one: Box<Conjunction> = Conjunction::from_propositions(vec![hat_rat, cat_mat]).unwrap();
        assert_eq!(2, one_x_one.complexity());
        let one_x_one: Proposition = Proposition::Conjunction(*one_x_one);
        assert_eq!(2, one_x_one.complexity());

        // atom x conjunction
        let pat = Proposition::Atom(String::from("Pat has a bat."));
        let two_x_one: Box<Conjunction> = Conjunction::from_propositions(vec![one_x_one, pat]).unwrap();
        assert_eq!(3, two_x_one.complexity());
    }
}


