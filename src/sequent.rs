use std::slice::Iter;
use std::cmp;

use crate::proposition::{Proposition, PropositionError};

#[derive(Debug, PartialEq, Eq)]
pub struct Sequent {
    antecedent: Vec<Proposition>,
    consequent: Vec<Proposition>
}
impl Sequent {
    pub fn is_atomic(&self) -> bool {
        for side in Side::iterator() {
            for prop in self.side(side) {
                if prop.complexity() > 0 { return false }
            }
        }
        true
    }

    pub fn complexity(&self) -> usize {
        let ant: usize = self.antecedent.iter().fold(0, |acc, x| acc + x.complexity());
        let con: usize = self.consequent.iter().fold(0, |acc, x| acc + x.complexity());
        ant + con
    }

    pub fn remove_prop_at(&mut self, side: Side, index: usize) {
        match side {
            Side::Ant => { self.antecedent.remove(index) }
            Side::Con => { self.consequent.remove(index) }
        };
    }

    pub fn mix(&mut self, other: Sequent) {
        self.antecedent.extend(other.antecedent);
        self.consequent.extend(other.consequent);
    }

    pub fn side(&self, side: &Side) -> &Vec<Proposition> {
        match side {
            Side::Ant => &self.antecedent,
            Side::Con => &self.consequent,
        }
    }

    pub fn first_complex_prop<'a>(&'a self) -> Option<FirstComplexProp> {
        for side in Side::iterator() {
            for (index, proposition) in self.side(side).iter().enumerate() {
                return Some(
                    FirstComplexProp {
                        proposition,
                        side,
                        index
                    }
                )
            }
        }
        None
    }

    pub fn from_string(s: String) -> Result<Self, SequentError> {
        let s: Vec<&str> = s.split(';').collect();
        let (ant, con): (&str, &str)  = match s.len().cmp(&2) {  // There are two sides to a sequent.
            cmp::Ordering::Less => return Err(SequentError::TooFewSemicolons),
            cmp::Ordering::Greater => return Err(SequentError::TooManySemicolons),
            cmp::Ordering::Equal => { (s[0], s[1]) }
        };
        let ant: Vec<Result<Proposition, PropositionError>> = ant
            .split(',')
            .map(|str| String::from(str))
            .map(|prop| Proposition::from_string(prop))
            .collect();
        let con: Vec<Result<Proposition, PropositionError>> = con
            .split(',')
            .map(|str| String::from(str))
            .map(|prop| Proposition::from_string(prop))
            .collect();


        todo!()
    }
}

#[derive(Copy, Clone)]
pub enum Side {
    Ant,
    Con
}
impl Side {
    pub fn iterator() -> Iter<'static, Side> {
        static SIDES: [Side; 2] = [Side::Ant, Side::Con];
        SIDES.iter()
    }
}

pub struct FirstComplexProp<'a, 'b> {
    proposition: &'a Proposition,
    side: &'b Side,
    index: usize
}

pub enum SequentError {
    TooFewSemicolons,
    TooManySemicolons,
    PropositionError(PropositionError)
}
