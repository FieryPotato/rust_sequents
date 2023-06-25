use crate::proposition::Proposition;
use crate::sequent::{Sequent, Side};
use crate::{branch, leaf};


pub fn decompose(mut sequent: Sequent, names: &Vec<String>) -> Option<Branch> {
    match sequent.first_complex_proposition() {
        None => None,
        Some(fcp) => {
            let proposition: Proposition = sequent.remove(&fcp);
            match proposition {
                Proposition::Atom(_) => panic!("Atom should have been caught by previous match statement"),
                Proposition::Negation(negatum) => Some(decompose_negation(sequent, fcp.side, *negatum, names)),
                Proposition::Conditional(left, right) => Some(decompose_conditional(sequent, fcp.side, *left, *right, names)),
                Proposition::Conjunction(left, right) => Some(decompose_conjunction(sequent, fcp.side, *left, *right, names)),
                Proposition::Disjunction(left, right) => Some(decompose_disjunction(sequent, fcp.side, *left, *right, names)),
                Proposition::Existential(var, content) => Some(decompose_existential(sequent, fcp.side, var, *content, names)),
                Proposition::Universal(var, content) => Some(decompose_universal(sequent, fcp.side, var, *content, names)),
            }
        }
    }
}

fn decompose_negation(mut sequent: Sequent, side: Side, negatum: Proposition, names: &Vec<String>) -> Branch {
    match side {
        Side::Antecedent => {
            sequent.push_right(negatum);
            branch![leaf![sequent]]
        },
        Side::Consequent => {
            sequent.push_left(negatum);
            branch![leaf![sequent]]
        }
    }
}

fn decompose_conditional(mut sequent: Sequent, side: Side, left: Proposition, right: Proposition, names: &Vec<String>) -> Branch {
    match side {
        Side::Antecedent => {
            let mut parent_0: Sequent = sequent.clone();
            parent_0.push_right(left);
            let mut parent_1: Sequent = sequent;
            parent_1.push_left(right);
            branch![leaf![parent_0, parent_1]]
        },
        Side::Consequent => {
            sequent.push_left(left);
            sequent.push_right(right);
            branch![leaf![sequent]]
        }
    }
}

fn decompose_conjunction(mut sequent: Sequent, side: Side, left: Proposition, right: Proposition, names: &Vec<String>) -> Branch {
    match side {
        Side::Antecedent => {
            sequent.push_left(left);
            sequent.push_left(right);
            branch![leaf![sequent]]
        },
        Side::Consequent => {
            let mut parent_0: Sequent = sequent.clone();
            parent_0.push_right(left);
            let mut parent_1: Sequent = sequent;
            parent_1.push_right(right);
            branch![leaf![parent_0, parent_1]]
        }
    }
}

fn decompose_disjunction(mut sequent: Sequent, side: Side, left: Proposition, right: Proposition, names: &Vec<String>) -> Branch {
    match side {
        Side::Antecedent => {
            let mut parent_0: Sequent = sequent.clone();
            parent_0.push_left(left);
            let mut parent_1: Sequent = sequent;
            parent_1.push_left(right);
            branch![leaf![parent_0, parent_1]]
        },
        Side::Consequent => {
            sequent.push_right(left);
            sequent.push_right(right);
            branch![leaf![sequent]]
        }
    }
}

fn decompose_existential(sequent: Sequent, side: Side, var: String, content: Proposition, names: &Vec<String>) -> Branch {
    match side {
        Side::Antecedent => {
           todo!("A branch for each name not in the sequent, each branch has one leaf.")
        },
        Side::Consequent => {
            let mut names = content.names();
            for name in sequent.names() {
                if !names.contains(&name) { names.push(name); }
            }
            let mut leaves: Vec<Leaf> = Vec::new();
            for name in names.into_iter() {
                let mut leaf: Sequent = sequent.clone();
                let mut prop: Proposition = content.clone();
                prop.instantiate(&var, &name);
                leaf.push_right(prop);
                leaves.push(leaf![leaf])
            }
            Branch { leaves }
        }
    }
}

fn decompose_universal(sequent: Sequent, side: Side, var: String, content: Proposition, names: &Vec<String>) -> Branch {
    match side {
        Side::Antecedent => {
            let mut names = content.names();
            for name in sequent.names() {
                if !names.contains(&name) { names.push(name); }
            }
            let mut leaves: Vec<Leaf> = Vec::new();
            for name in names.into_iter() {
                let mut leaf: Sequent = sequent.clone();
                let mut prop: Proposition = content.clone();
                prop.instantiate(&var.clone(), &name);
                leaf.push_right(prop);
                leaves.push(leaf![leaf])
            }
            Branch { leaves }
        },
        Side::Consequent => {
            todo!("A branch for each name not in the sequent, each branch has one leaf.")
        },
    }
}


/// Leaves represent one way a sequent could have been constructed. For invertible rules,
/// there is only the one set of parents. For non-invertible rules there may be multiple
/// sets of parents.
pub struct Leaf { parents: Vec<Sequent> }

/// Branches represent the full set of ways a sequent could have been constructed.
pub struct Branch { leaves: Vec<Leaf> }

#[macro_export]
macro_rules! branch {
    ( $( $x:expr ),* ) => {
        {
            let mut leaves = Vec::new();
            $(
                leaves.push($x);
            )*
            Branch { leaves }
        }
    };
}

#[macro_export]
macro_rules! leaf {
    ( $( $x:expr ),* ) => {
        {
            let mut parents = Vec::new();
            $(
                parents.push($x);
            )*
            Leaf { parents }
        }
    }
}
