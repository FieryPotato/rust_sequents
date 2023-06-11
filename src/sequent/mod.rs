mod decompose;

use std::fmt::{Display, Formatter};
use itertools::Itertools;
use crate::proposition::Proposition;

pub struct Sequent {
    ant: Vec<Proposition>,
    con: Vec<Proposition>,
}

impl Sequent {
    /// Return the number of connectives in self.
    pub fn complexity(&self) -> usize {
        let ant_complexity = itertools::max(self.ant.iter().map(|prop| prop.complexity())).unwrap_or(0);
        let con_complexity = itertools::max(self.con.iter().map(|prop| prop.complexity())).unwrap_or(0);
        return ant_complexity + con_complexity
    }

    /// Returns the proposition at index of side.
    ///
    /// # Panics
    /// Panics if index is greater than self.side's length.
    pub fn remove(&mut self, coordinates: &Coordinates) -> Proposition {
        match coordinates.side {
            Side::Antecedent => self.ant.remove(coordinates.index),
            Side::Consequent => self.con.remove(coordinates.index)
        }
    }

    /// Returns the coordinates of this sequent's first (from left to right)
    /// complex proposition. Returns None if self is atomic.
    pub fn first_complex_proposition(&self) -> Option<Coordinates> {
        for (index, prop) in self.ant.iter().enumerate() {
            if prop.complexity() > 0 {
                return Some( Coordinates { side: Side::Antecedent, index } )
            }
        }
        for (index, prop) in self.con.iter().enumerate() {
            if prop.complexity() > 0 {
                return Some( Coordinates { side: Side::Consequent, index } )
            }
        }
        None
    }

    /// Push proposition to the consequent of self.
    pub(crate) fn push_right(&mut self, proposition: Proposition) {
        self.ant.push(proposition);
    }

    /// Push proposition to the antecedent of self.
    pub(crate) fn push_left(&mut self, proposition: Proposition) {
        self.con.push(proposition);
    }

    /// Return the names in all the propositions in self.
    pub fn names(&self) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        for prop in self.ant.iter() {
            for name in prop.names() {
                names.push(name);
            }
        }
        for prop in self.con.iter() {
            for prop in self.con.iter() {
                for name in prop.names() {
                    names.push(name);
                }
            }
        }
        names
    }
}

impl Display for Sequent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ant: String = self.ant.iter().map(|x| x.to_string()).join(", ");
        let con: String = self.con.iter().map(|x| x.to_string()).join(", ");
        write!(f, "{ant} |~ {con}")
    }
}

impl Clone for Sequent {
    fn clone(&self) -> Self {
        let ant = self.ant.clone();
        let con = self.con.clone();
        Sequent { ant, con }
    }
}

pub enum Side {
    Antecedent,
    Consequent
}

pub struct Coordinates {
    pub side: Side,
    pub index: usize
}
