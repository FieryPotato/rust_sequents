use itertools::Itertools;
use crate::proposition::{is_conjunction_str, is_negation_str, Proposition, PropositionError, Connective};
use crate::proposition::negation::Negation;
use crate::proposition::conjunction::Conjunction;


/// Remove the outermost connected pair of parentheses ("()") from the input string.
pub fn deparenthesize(string: &mut String) {
    // Empty strings trigger early return.
    if string.len() == 0 { return }
    // While string is bookended by parentheses:
    while string.starts_with('(') && string.ends_with(')') {
        // Ensure outer parentheses match.
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
pub fn find_connective(mut string: String) -> Vec<String> {
    deparenthesize(&mut string);
    if string.len() == 0 { return vec![String::new()] }
    // Check for negations.
    match try_parse_as_unary_connective(&string) {
        Some(r) => return r,
        None => {}
    }
    // Check for binary connections
    match try_parse_as_binary_connective(&string) {
        Some(r) => return r,
        None => {}
    }
    // Return as an Atom
    vec![string]
}

fn try_parse_as_unary_connective(string: &String) -> Option<Vec<String>> {
    let mut words = string.split_whitespace();
    match words.next().expect("words has len > 0") {
        c if is_negation_str(c) => {
            let mut negatum: String = words.join(" ");
            deparenthesize(&mut negatum);
            return Some(vec![c.to_owned(), negatum])
        },
        _ => None
    }
}

fn try_parse_as_binary_connective(string: &String) -> Option<Vec<String>> {
    let words = string.split_whitespace();
    let mut nestedness: i32 = 0;
    for (index, word) in words.enumerate() {
        for char in word.chars() {
            match char {
                '(' => nestedness += 1,
                ')' => nestedness -= 1,
                _ => {}
            };
        }
        if nestedness == 0 {
            let word_vec: Vec<&str> = string.split_whitespace().collect();
            match word {
                c if is_conjunction_str(c) => {
                    let mut left: String = word_vec[..index].join(" ");
                    deparenthesize(&mut left);
                    let mut right: String = word_vec[(index + 1)..].join(" ");
                    deparenthesize(&mut right);
                    return Some(vec![left, c.to_owned(), right])
                },
                _ => {}
            }
        }
    }
    None
}

pub(crate) fn make_atom(mut word_groups: Vec<String>) -> Proposition {
    let content = word_groups.pop().expect("split_string.len() should be 1");
    Proposition::Atom(content)
}

pub(crate) fn make_unary(mut word_groups: Vec<String>) -> Result<Proposition, PropositionError> {
    let content: String = word_groups.pop().expect("split_string.len() == 2");
    let connective: String = word_groups.pop().expect("split_string.len() == 1");
    if is_negation_str(&connective) {
        match Proposition::from_string(content) {
            Ok(p) => {
                let negation: Negation = *Negation::from_propositions(vec![p])?;
                Ok(Proposition::Negation(negation))
            },
            Err(e) => Err(e)
        }
    } else {
        Err(PropositionError::InvalidConnective)
    }
}

pub(crate) fn make_binary(mut word_groups: Vec<String>) -> Result<Proposition, PropositionError> {
    let right: String = word_groups.pop().expect("split_string.len() == 3");
    let connective: String = word_groups.pop().expect("split_string.len() == 2");
    let left: String = word_groups.pop().expect("split_string.len() == 1");
    match connective {
        _ if is_conjunction_str(&connective) => {
            let left: Proposition = Proposition::from_string(left)?;
            let right: Proposition = Proposition::from_string(right)?;
            let conjunction: Conjunction = *Conjunction::from_propositions(vec![left, right])?;
            Ok(Proposition::Conjunction(conjunction))
        }
        _ => Err(PropositionError::NoConnectiveFound)
    }
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
}
