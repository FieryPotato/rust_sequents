use std::iter::Peekable;
use std::str::SplitWhitespace;

use itertools::Itertools;

use crate::proposition::{Proposition, PropositionError, Connective};
use crate::proposition::{is_conjunction_str, is_negation_str, is_conditional_str, is_disjunction_str};
use crate::proposition::negation::Negation;
use crate::proposition::conjunction::Conjunction;
use crate::proposition::conditional::Conditional;
use crate::proposition::disjunction::Disjunction;



/// Remove the outermost connected pair of parentheses ("()") from the input string.
pub fn deparenthesize(string: &mut String) {
    // Empty strings trigger early return.
    if string.len() == 0 { return }
    // While string is bookended by parentheses:
    while string.starts_with('(') && string.ends_with(')') {
        let mut nestedness: i32 = 0;
        for (index, char) in string.chars().enumerate() {
            // '(' chars add 1 to nestedness,
            // ')' chars subtract 1.
            // Anything else doesn't affect it.
            match char {
                '(' => nestedness += 1,
                ')' => nestedness -= 1,
                _ => {}
            }
            // if the parents are connected, then nestendess should only equal
            // 0 if char is the last character in the string. If it's not, then
            // we've reached something like (A & B) -> (C v D)
            if nestedness <= 0 && ((index + 1) < string.len()) {
                return
            }
        }

        // If we haven't returned by now, the outermost characters are a
        // connected pair of parentheses, so we remove them and check again.
        *string = string[1..(string.len()-1)].to_owned()
    }
}

/// Return a Vec containing a connective and any propositional content as strings
/// in order around it.
/// ~ A => vec!["~", "A"],
/// A & B => vec!["A", "&", "B"]
/// etc.
pub fn proposition_from_string(mut string: String) -> Result<Proposition, PropositionError> {
    deparenthesize(&mut string);
    if string.len() == 0 { return Err(PropositionError::NoConnectiveFound) }
    let mut words: Peekable<SplitWhitespace> = string.split_whitespace().peekable();

    // Check for negations.
    if is_negation_str(words.peek().expect("words has len > 0")) {
        words.next();  // We know that the first word in words is a negation string,
                       // so we advance the iterator to get it out of the way.
        return create_negation(words.join(" "))
    }

    // Check for binary connectives
    match check_for_binary_proposition(&string) {
        Some(prop) => return prop,
        None => {}
    }

    // Return as an Atom
    Ok(Proposition::Atom(string))
}

fn create_negation(mut negatum: String) -> Result<Proposition, PropositionError> {
    deparenthesize(&mut negatum);
    match Proposition::from_string(negatum) {
        Ok(p) => {
            let negation: Negation = *Negation::from_propositions(vec![p])?;
            return Ok(Proposition::Negation(negation))
        },
        Err(e) => return Err(e)
    }
}

fn check_for_binary_proposition(string: &String) -> Option<Result<Proposition, PropositionError>> {
    let words = string.split_whitespace();
    let mut nestedness: i32 = 0;
    for (index, word) in words.enumerate() {
        if word.starts_with('(') { nestedness += 1 }
        if word.ends_with(')') { nestedness -= 1 }

        if nestedness == 0 {
            let word_vec: Vec<&str> = string.split_whitespace().collect();
            let left: String = word_vec[..index].join(" ");
            let right: String = word_vec[(index + 1)..].join(" ");
            match word {
                c if is_conjunction_str(c) => {
                    return Some(create_conjunction(left, right))
                },

                c if is_conditional_str(c) => {
                    return Some(create_conditional(left, right))
                },
                c if is_disjunction_str(c) => {
                    return Some(create_disjunction(left, right))
                },
                _ => {}
            }
        }
    }
    None
}

fn create_conjunction(mut left: String, mut right: String) -> Result<Proposition, PropositionError> {
    deparenthesize(&mut left);
    deparenthesize(&mut right);
    let left: Proposition = Proposition::from_string(left)?;
    let right: Proposition = Proposition::from_string(right)?;
    let conjunction: Conjunction = *Conjunction::from_propositions(vec![left, right])?;
    Ok(Proposition::Conjunction(conjunction))
}

fn create_conditional(mut left: String, mut right: String) -> Result<Proposition, PropositionError> {
    deparenthesize(&mut left);
    deparenthesize(&mut right);
    let left: Proposition = Proposition::from_string(left)?;
    let right: Proposition = Proposition::from_string(right)?;
    let conditional: Conditional = *Conditional::from_propositions(vec![left, right])?;
    Ok(Proposition::Conditional(conditional))
}

fn create_disjunction(mut left: String, mut right: String) -> Result<Proposition, PropositionError> {
    deparenthesize(&mut left);
    deparenthesize(&mut right);
    let left: Proposition = Proposition::from_string(left)?;
    let right: Proposition = Proposition::from_string(right)?;
    let disjunction: Disjunction = *Disjunction::from_propositions(vec![left, right])?;
    Ok(Proposition::Disjunction(disjunction))
}

#[cfg(test)]
mod test {
    use crate::proposition::create::deparenthesize;
    use super::*;

    #[test]
    fn test_deparenthesize() {
        let mut one_pair: String = String::from("(hello)");
        deparenthesize(&mut one_pair);
        assert_eq!(one_pair, String::from("hello"));

        let mut two_pair: String = String::from("((hello))");
        deparenthesize(&mut two_pair);
        assert_eq!(two_pair, String::from("hello"));

        let mut nested_pair: String = String::from("(hello (goodbye))");
        deparenthesize(&mut nested_pair);
        assert_eq!(nested_pair, String::from("hello (goodbye)"));

        let mut disjoint: String = String::from("((hello) (goodbye))");
        deparenthesize(&mut disjoint);
        assert_eq!(disjoint, String::from("(hello) (goodbye)"));
    }

    #[test]
    fn atom_from_string() {
        let string: String = String::from("the cat is on the mat");
        let atom: Proposition = Proposition::from_string(string).unwrap();
        match atom {
            Proposition::Atom(a) => assert_eq!(a, String::from("the cat is on the mat")),
            _ => panic!["atom was not successfully created"]
        }
    }

    #[test]
    fn negation_from_string() {
        // actual
        let string: String = String::from("~ the cat is on the mat");
        let actual: Proposition = Proposition::from_string(string).unwrap();

        // expected
        let string: String = String::from("the cat is on the mat");
        let atom: Proposition = Proposition::Atom(string);
        let expected: Negation = *Negation::from_propositions(vec![atom]).unwrap();

        match actual {
            Proposition::Negation(n) => assert_eq!(expected, n),
            _ => panic!["negation was not successfully created"]
        }
    }

    #[test]
    fn nested_negations_from_string() {
        // actual
        let string: String = String::from("~ (~ (the cat is on the mat))");
        let actual: Proposition = Proposition::from_string(string).unwrap();

        // expected
        let string: String = String::from("the cat is on the mat");
        let atom: Proposition = Proposition::Atom(string);
        let neg_1: Negation = *Negation::from_propositions(vec![atom]).unwrap();
        let expected: Negation = *Negation::from_connectives(vec![neg_1]).unwrap();

        match actual {
            Proposition::Negation(n) => assert_eq!(expected, n),
            _ => panic!["negation was not successfully created"]
        }
    }

    #[test]
    fn conjunction_from_string() {
        // actual
        let string: String = String::from("the cat is on the mat & the hat is on the rat");
        let actual: Proposition = Proposition::from_string(string).unwrap();

        // expected
        let left: String = String::from("the cat is on the mat");
        let left: Proposition = Proposition::Atom(left);
        let right: String = String::from("the hat is on the rat");
        let right: Proposition = Proposition::Atom(right);
        let expected: Conjunction = *Conjunction::from_propositions(vec![left, right]).unwrap();

        match actual {
            Proposition::Conjunction(c) => assert_eq!(expected, c),
            _ => panic!["conjunction was not successfully created"]
        }
    }

    #[test]
    fn nested_conjunction() {
        // actual
        let string: String = String::from("(the cat is on the mat & the mat is under the cat) and (the hat is on the rat & the rat wears the hat)");
        let actual: Proposition = Proposition::from_string(string).unwrap();

        // expected
        let inner_ll: String = String::from("the cat is on the mat");
        let inner_ll: Proposition = Proposition::Atom(inner_ll);
        let inner_lr: String = String::from("the mat is under the cat");
        let inner_lr: Proposition = Proposition::Atom(inner_lr);
        let inner_rl: String = String::from("the hat is on the rat");
        let inner_rl: Proposition = Proposition::Atom(inner_rl);
        let inner_rr: String = String::from("the rat wears the hat");
        let inner_rr: Proposition = Proposition::Atom(inner_rr);
        let outer_l: Conjunction = *Conjunction::from_propositions(vec![inner_ll, inner_lr]).unwrap();
        let outer_r: Conjunction = *Conjunction::from_propositions(vec![inner_rl, inner_rr]).unwrap();
        let expected: Conjunction = *Conjunction::from_connectives(vec![outer_l, outer_r]).unwrap();

        match actual {
            Proposition::Conjunction(c) => assert_eq!(expected, c),
            _ => panic!["Conjunction was not successfully created"]
        }
    }

    #[test]
    fn conditional_from_string() {
        // actual
        let string: String = String::from("the cat is on the mat > the hat is on the rat");
        let actual: Proposition = Proposition::from_string(string).unwrap();

        // expected
        let left: String = String::from("the cat is on the mat");
        let left: Proposition = Proposition::Atom(left);
        let right: String = String::from("the hat is on the rat");
        let right: Proposition = Proposition::Atom(right);
        let expected: Conditional = *Conditional::from_propositions(vec![left, right]).unwrap();

        match actual {
            Proposition::Conditional(c) => assert_eq!(expected, c),
            _ => panic!["Conditional was not successfully created"]
        }
    }

    #[test]
    fn nested_conditional() {
        // actual
        let string: String = String::from("(the cat is on the mat > the mat is under the cat) implies (the hat is on the rat > the rat wears the hat)");
        let actual: Proposition = Proposition::from_string(string).unwrap();

        // expected
        let inner_ll: String = String::from("the cat is on the mat");
        let inner_ll: Proposition = Proposition::Atom(inner_ll);
        let inner_lr: String = String::from("the mat is under the cat");
        let inner_lr: Proposition = Proposition::Atom(inner_lr);
        let inner_rl: String = String::from("the hat is on the rat");
        let inner_rl: Proposition = Proposition::Atom(inner_rl);
        let inner_rr: String = String::from("the rat wears the hat");
        let inner_rr: Proposition = Proposition::Atom(inner_rr);
        let outer_l: Conditional = *Conditional::from_propositions(vec![inner_ll, inner_lr]).unwrap();
        let outer_r: Conditional = *Conditional::from_propositions(vec![inner_rl, inner_rr]).unwrap();
        let expected: Conditional = *Conditional::from_connectives(vec![outer_l, outer_r]).unwrap();

        match actual {
            Proposition::Conditional(c) => assert_eq!(expected, c),
            _ => panic!["Conditional was not successfully created"]
        }
    }

    #[test]
    fn disjunction_from_string() {
        // actual
        let string: String = String::from("the cat is on the mat v the hat is on the rat");
        let actual: Proposition = Proposition::from_string(string).unwrap();

        // expected
        let left: String = String::from("the cat is on the mat");
        let left: Proposition = Proposition::Atom(left);
        let right: String = String::from("the hat is on the rat");
        let right: Proposition = Proposition::Atom(right);
        let expected: Disjunction = *Disjunction::from_propositions(vec![left, right]).unwrap();

        match actual {
            Proposition::Disjunction(c) => assert_eq!(expected, c),
            _ => panic!["Disjunction was not successfully created"]
        }
    }

    #[test]
    fn nested_disjunction() {
        // actual
        let string: String = String::from("(the cat is on the mat v the mat is under the cat) or (the hat is on the rat v the rat wears the hat)");
        let actual: Proposition = Proposition::from_string(string).unwrap();

        // expected
        let inner_ll: String = String::from("the cat is on the mat");
        let inner_ll: Proposition = Proposition::Atom(inner_ll);
        let inner_lr: String = String::from("the mat is under the cat");
        let inner_lr: Proposition = Proposition::Atom(inner_lr);
        let inner_rl: String = String::from("the hat is on the rat");
        let inner_rl: Proposition = Proposition::Atom(inner_rl);
        let inner_rr: String = String::from("the rat wears the hat");
        let inner_rr: Proposition = Proposition::Atom(inner_rr);
        let outer_l: Disjunction = *Disjunction::from_propositions(vec![inner_ll, inner_lr]).unwrap();
        let outer_r: Disjunction = *Disjunction::from_propositions(vec![inner_rl, inner_rr]).unwrap();
        let expected: Disjunction = *Disjunction::from_connectives(vec![outer_l, outer_r]).unwrap();

        match actual {
            Proposition::Disjunction(c) => assert_eq!(expected, c),
            _ => panic!["Disjunction was not successfully created"]
        }
    }

}
