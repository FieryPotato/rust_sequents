use crate::proposition::Proposition;
use crate::sequent::{Sequent, Side};

pub fn decompose(mut sequent: Sequent) -> Option<Vec<Sequent>> {
    match sequent.first_complex_proposition() {
        None => None,
        Some(fcp) => {
            let proposition: Proposition = sequent.remove(&fcp);
            match proposition {
                Proposition::Atom(_) => panic!("Atom should have been caught by previous match statement"),
                Proposition::Negation(negatum) => Some(decompose_negation(sequent, *negatum)),
                Proposition::Conditional(left, right) => Some(decompose_conditional(sequent, *left, *right)),
                Proposition::Conjunction(left, right) => Some(decompose_conjunction(sequent, *left, *right)),
                Proposition::Disjunction(left, right) => Some(decompose_disjunction(sequent, *left, *right)),
                Proposition::Existential(var, content) => Some(decompose_existential(sequent, var, *content)),
                Proposition::Universal(var, content) => Some(decompose_universal(sequent, var, *content)),
            }
        }
    }
}

fn decompose_negation(sequent: Sequent, negatum: Proposition) -> Vec<Sequent> {
    todo!()
}

fn decompose_conditional(sequent: Sequent, left: Proposition, right: Proposition) -> Vec<Sequent> {
    todo!()
}

fn decompose_conjunction(sequent: Sequent, left: Proposition, right: Proposition) -> Vec<Sequent> {
    todo!()
}

fn decompose_disjunction(sequent: Sequent, left: Proposition, right: Proposition) -> Vec<Sequent> {
    todo!()
}

fn decompose_existential(sequent: Sequent, var: String, content: Proposition) -> Vec<Sequent> {
    todo!()
}

fn decompose_universal(sequent: Sequent, var: String, content: Proposition) -> Vec<Sequent> {
    todo!()
}
