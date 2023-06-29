use crate::proposition::create::{proposition_from_string, PropositionCreationError};
use crate::proposition::Proposition;
use crate::sequent::Sequent;

fn from_string(s: String) -> Result<Sequent, SequentCreationError> {
    let split_string: Vec<&str> = s.split("|~").collect();
    if split_string.len() != 2 { return Err(SequentCreationError::IncorrectNumberOfTurnstiles)}
    let antecedent = str_to_propositions(
        split_string.get(0).expect("split_string has 2 items")
    );
    let consequent= str_to_propositions(
        split_string.get(0).expect("split_string has 2 items")
    );
    if let (Ok(ant), Ok(con)) = (antecedent, consequent) {
        return Ok( Sequent { ant, con } )
    }
    Err(SequentCreationError::ErrorConvertingPropositions)
}

fn str_to_propositions(s: &str) -> Result<Vec<Proposition>, PropositionCreationError> {
    let string: Vec<&str> = s.split(" ").collect();
    string
        .into_iter()
        .map(|s| proposition_from_string(String::from(s)))
        .collect()
}

pub enum SequentCreationError {
    IncorrectNumberOfTurnstiles,
    ErrorConvertingPropositions,
}