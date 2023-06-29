use std::iter::Peekable;
use std::str::Split;
use itertools::Itertools;
use crate::proposition::{Proposition, PropositionType};

const NEGATIONS: [&str; 2] = ["~", "not"];
const CONDITIONALS: [&str; 2] = [">", "implies"];
const CONJUNCTIONS: [&str; 2] = ["&", "and"];
const DISJUNCTIONS: [&str; 2] = ["v", "or"];
const EXISTENTIALS: [&str; 2] = ["∃", "exists"];
const UNIVERSALS: [&str; 2] = ["∀", "forall"];



pub(crate) fn proposition_from_string(source: String) -> Result<Proposition, PropositionCreationError> {
    match find_connective(source)? {
        Connective::Atom(atom) => Ok(Proposition::Atom(atom)),
        Connective::Unary(connective, content) => make_unary(connective, content),
        Connective::Binary(left, connective, right) => make_binary(left, connective, right),
        Connective::Quantifier(quantifier, variable, predicate) => make_quantifier(quantifier, variable, predicate)
    }
}

fn make_unary(_connective: String, content: String) -> Result<Proposition, PropositionCreationError> {
    // add a match statement if ever we add a non-negation unary connective
    Ok(Proposition::Negation(Box::new(proposition_from_string(content)?)))
}

fn make_binary(left: String, connective: String, right: String) -> Result<Proposition, PropositionCreationError> {
    match connective.as_str() {
        ">" | "implies" => Ok(
            Proposition::Conditional(
                Box::new(proposition_from_string(left)?),
                Box::new(proposition_from_string(right)?)
            )
        ),
        "&" | "and" => Ok(
            Proposition::Conjunction(
                Box::new(proposition_from_string(left)?),
                Box::new(proposition_from_string(right)?)
            )
        ),
        "v" | "or" => Ok(
            Proposition::Disjunction(
                Box::new(proposition_from_string(left)?),
                Box::new(proposition_from_string(right)?)
            )
        ),
        _ => Err(PropositionCreationError::InvalidConnective(connective))
    }
}

fn make_quantifier(quantifier: String, variable: String, predicate: String) -> Result<Proposition, PropositionCreationError> {
    match quantifier.as_str() {
        "∃" | "exists" => Ok(
            Proposition::Existential(
                variable,
                Box::new(proposition_from_string(predicate)?)
            )
        ),
        "∀" | "forall" => Ok(
            Proposition::Universal(
                variable,
                Box::new(proposition_from_string(predicate)?)
            )
        ),
        _ => Err(PropositionCreationError::InvalidConnective(quantifier))
    }
}

fn deparenthesize(string: &mut String) {
    while string_first_and_last_chars_are_connected_parens(&string) {
        string.remove(0);
        string.remove(string.len() - 1);
    }
}

fn string_first_and_last_chars_are_connected_parens(string: &String) -> bool {
    // if they're not parens, they can't be connected parens
    if !string.starts_with('(') || !string.ends_with(')') { return false }

    // check connectedness by counting open and closes
    let mut nestedness: usize = 0;
    for (index, char) in string.chars().enumerate() {
        match char {
            '(' => nestedness += 1,
            ')' => nestedness -= 1,
            _ => {}
        }
        // nestedness of 0 only occurs at the end of the string for connected parens
        // nestedness of 0 otherwise means they're not connected, eg. (A v B) & (C > D)
        if nestedness <= 0 && ((index + 1) < string.len()) { return false }
    }

    true
}

fn proposition_type_from_char(c: char) -> PropositionType {
    match c {
        '~' => PropositionType::Negation,
        '>' => PropositionType::Conditional,
        '&' => PropositionType::Conjunction,
        'v' => PropositionType::Disjunction,
        '∃' => PropositionType::Existential,
        '∀' => PropositionType::Universal,
        _ => PropositionType::Atom
    }
}

fn find_connective(mut s: String) -> Result<Connective, PropositionCreationError> {
    deparenthesize(&mut s);
    // peekable to check items without consuming
    let mut words = s.split(' ').peekable();

    // empty strings are not connectives
    if words.peek().is_none() { return Err(PropositionCreationError::EmptyString) }

    // check for negation
    if NEGATIONS.contains(words.peek().expect("words should not be empty")) {
        return find_negation(&mut words)
    }

    // check for quantifiers
    let quantifiers: [&str; 4] = <[&str; 4]>::try_from([EXISTENTIALS, UNIVERSALS].concat()).unwrap();
    if quantifiers.contains(words.peek().expect("words should not be empty")) {
        return find_quantifier(&mut words)
    }

    // check for binaries
    if let Some(binary) = find_binary(words.clone()) {
        return binary;
    }

    Ok(Connective::Atom(words.join(" ")))
}

fn find_binary(words: Peekable<Split<char>>) -> Option<Result<Connective, PropositionCreationError>> {
    let mut nestedness: usize = 0;  // nestedness of the head (index) in parentheses
    let word_clone = words.clone();  // clone words since we need a copy if successful
    for (index, word) in word_clone.enumerate() {
        if nested_word_is_binary_connective(word, nestedness) {
            return Some(create_binary(words, index))
        }
        for letter in word.chars() {
            match letter {
                '(' => nestedness += 1,
                ')' => nestedness -= 1,
                _ => {}
            }
        }
    }
    None
}

fn nested_word_is_binary_connective(word: &str, nestedness: usize) -> bool {
    let binaries: [&str; 6] = <[&str; 6]>::try_from([CONDITIONALS, CONJUNCTIONS, DISJUNCTIONS].concat()).unwrap();
    nestedness <= 0 && binaries.contains(&word)
}

fn create_binary(mut words: Peekable<Split<char>>, index: usize) -> Result<Connective, PropositionCreationError> {
    // left side is everything before the connective
    let mut left: Vec<String> = Vec::new();
    for _ in 0..index {
        left.push(String::from(words.next().expect("Words contains at least `index` items")));
    }
    let left: String = left.join(" ");

    let connective: String = String::from(words.next().expect("Words contains at least `index` items"));

    // right side is everything after the connective
    let mut right: Vec<String> = Vec::new();
    while let Some(right_word) = words.next() {
        right.push(String::from(right_word))
    }
    let right: String = right.join(" ");

    // neither side should be empty
    if left.len() <= 0 || right.len() <= 0 {
        let full_string: String = [left, connective, right].join(" ");
        return Err(PropositionCreationError::MalformedString(full_string));
    }

    return Ok(Connective::Binary(left, connective, right));
}

fn find_quantifier(words: &mut Peekable<Split<char>>) -> Result<Connective, PropositionCreationError> {
    let quantifier = words.next().expect("peeked").to_string();
    let var = match check_for_var(words.next()) {
        Some(letter) => letter,
        None => return Err(PropositionCreationError::MalformedString(String::from("improper variable shape")))
    };
    let mut predicate = words.join(" ");
    deparenthesize(&mut predicate);
    Ok(Connective::Quantifier(quantifier, var, predicate))
}

fn find_negation(words: &mut Peekable<Split<char>>) -> Result<Connective, PropositionCreationError> {
    let negation = words.next().expect("peeked").to_string();
    let mut negatum = words.join(" ");
    deparenthesize(&mut negatum);
    return Ok(Connective::Unary(negation, negatum));
}

fn check_for_var(word: Option<&str>) -> Option<String> {
    match word {
        None => None,
        Some(string) => {
            let mut string = String::from(string);

            // names and variables are single letters contained within "<" and ">"
            if !string.starts_with("<") && !string.ends_with(">") { return None };
            if !string.len() != 3 { return None }

            // we only care about the contents though
            string.remove(0);
            string.remove(string.len() - 1);
            Some(string)
        }
    }
}

enum Connective {
    Atom(String),
    Unary(String, String),
    Binary(String, String, String),
    Quantifier(String, String, String)
}

#[derive(Debug)]
pub enum PropositionCreationError {
    MalformedString(String),
    InvalidConnective(String),
    EmptyString,
}


#[cfg(test)]
mod test {
    use crate::proposition::create::proposition_from_string;
    use crate::proposition::Proposition;

    #[test]
    fn test_atom_from_str() {
        let s = "the cat is on the mat".to_string();
        let expected = Proposition::Atom(s.clone());
        let actual = proposition_from_string(s);
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn test_negation_from_str() {
        let expected = Proposition::Negation(
            Box::new(Proposition::Atom(
                String::from("the cat is on the mat"))
            )
        );
        let symb = "~ (the cat is on the mat)".to_string();
        let symb = proposition_from_string(symb);
        assert_eq!(expected, symb.unwrap());

        let char = "not (the cat is on the mat)".to_string();
        let char = proposition_from_string(char);
        assert_eq!(expected, char.unwrap());
    }

    #[test]
    fn test_conditional_from_str() {
        let expected = Proposition::Conditional(
            Box::new(Proposition::Atom(String::from("Kitty is a cat"))),
            Box::new(Proposition::Atom(String::from("Kitty is on the mat")))
        );
        let symb = "(Kitty is a cat) > (Kitty is on the mat)".to_string();
        let symb = proposition_from_string(symb);
        assert_eq!(expected, symb.unwrap());

        let char = "(Kitty is a cat) implies (Kitty is on the mat)".to_string();
        let char = proposition_from_string(char);
        assert_eq!(expected, char.unwrap());
    }

    #[test]
    fn test_conjunction_from_str() {
        let expected = Proposition::Conjunction(
            Box::new(Proposition::Atom(String::from("Kitty is a cat"))),
            Box::new(Proposition::Atom(String::from("Kitty is on the mat")))
        );
        let symb = "(Kitty is a cat) & (Kitty is on the mat)".to_string();
        let symb = proposition_from_string(symb);
        assert_eq!(expected, symb.unwrap());

        let char = "(Kitty is a cat) and (Kitty is on the mat)".to_string();
        let char = proposition_from_string(char);
        assert_eq!(expected, char.unwrap());
    }

    #[test]
    fn test_disjunction_from_str() {
        let expected = Proposition::Disjunction(
            Box::new(Proposition::Atom(String::from("Kitty is a cat"))),
            Box::new(Proposition::Atom(String::from("Kitty is on the mat")))
        );
        let symb = "(Kitty is a cat) v (Kitty is on the mat)".to_string();
        let symb = proposition_from_string(symb);
        assert_eq!(expected, symb.unwrap());

        let char = "(Kitty is a cat) or (Kitty is on the mat)".to_string();
        let char = proposition_from_string(char);
        assert_eq!(expected, char.unwrap());
    }
}