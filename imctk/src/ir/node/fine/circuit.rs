//! Fine grained nodes for representing combinational and sequential circuits.
use imctk_ids::{Id, Id32};

use crate::{
    ir::{
        node::expr::{Expr, ExprNode},
        var::{Lit, Pol, Var},
    },
    unordered_pair::UnorderedPair,
};

/// Expression representing a logical 'and'.
///
/// This is a combinational operation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct And {
    /// The operands for the 'and' given as input literals.
    pub inputs: UnorderedPair<Lit>,
}

/// Node representing a logical 'and'.
pub type AndNode = ExprNode<And>;

impl Expr for And {
    type Output = Lit;

    const NAME: &'static str = "And";

    fn input_var_iter(&self) -> impl Iterator<Item = Var> + '_ {
        self.inputs.map(|lit| lit.var()).into_iter()
    }

    fn representative_input_var(&self) -> Var {
        self.inputs[1].var()
    }

    fn max_var(&self) -> Var {
        self.inputs.max_element().var()
    }

    fn apply_var_map(&mut self, var_map: impl FnMut(Var) -> Lit) -> Pol {
        self.inputs.apply_var_map(var_map);
        Pol::Pos
    }
}

/// Expression representing a logical 'exclusive or' / 'xor'.
///
/// This is a combinational operation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Xor {
    /// The operands for the 'xor' given as input literals.
    pub inputs: UnorderedPair<Var>,
}

/// Node representing a logical 'exclusive or' / 'xor'.
pub type XorNode = ExprNode<Xor>;

impl Expr for Xor {
    type Output = Lit;
    const NAME: &'static str = "Xor";

    fn input_var_iter(&self) -> impl Iterator<Item = Var> + '_ {
        self.inputs.into_iter()
    }

    fn representative_input_var(&self) -> Var {
        self.inputs[1]
    }

    fn apply_var_map(
        &mut self,
        var_map: impl FnMut(Var) -> Lit,
    ) -> <Self::Output as crate::ir::var::VarOrLit>::Pol {
        self.inputs.apply_var_map_compose_pol(var_map)
    }
}

/// Expression representing a steady input or an unconstrained steady value.
///
/// An important use case is unconstrained register initialization, where this provides the initial
/// value.
#[derive(Id, Debug)]
#[repr(transparent)]
pub struct SteadyInput(Id32);

/// Node representing a steady input or a steady unconstrained constant value.
pub type SteadyInputNode = ExprNode<SteadyInput>;

impl Expr for SteadyInput {
    type Output = Lit;
    const NAME: &'static str = "Init";

    fn input_var_iter(&self) -> impl Iterator<Item = Var> + '_ {
        [].into_iter()
    }

    fn apply_var_map(&mut self, _var_map: impl FnMut(Var) -> Lit) -> Pol {
        Pol::Pos
    }
}

/// Expression representing a time-varying input or an unconstrained time-varying value.
#[derive(Id, Debug)]
#[repr(transparent)]
pub struct Input(Id32);

/// Node representing a time-varying input or an unconstrained time-varying value.
pub type InputNode = ExprNode<Input>;

impl Expr for Input {
    type Output = Lit;
    const NAME: &'static str = "Input";

    fn input_var_iter(&self) -> impl Iterator<Item = Var> + '_ {
        [].into_iter()
    }

    fn apply_var_map(&mut self, _var_map: impl FnMut(Var) -> Lit) -> Pol {
        Pol::Pos
    }
}

/// Expression representing a register that updates with each transition of the represented state
/// system.
///
/// In the initial state it transparently passes through the `init` input, and after every
/// transition it will output the value the `next` input had before the transition.
///
/// When viewing time-varying signals as infinite streams this can also be seen as the "cons"
/// operation on infinite streams.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(C)]
pub struct Reg {
    /// Produced value in the initial state.
    pub init: Lit,
    /// Value to produce after the next transition.
    pub next: Var,
}

/// Node representing a register that updates with each transition of the represented state
/// system.
pub type RegNode = ExprNode<Reg>;

impl Expr for Reg {
    type Output = Lit;
    const NAME: &'static str = "Reg";

    fn input_var_iter(&self) -> impl Iterator<Item = Var> + '_ {
        [self.init.var(), self.next].into_iter()
    }

    fn unguarded_input_var_iter(&self) -> impl Iterator<Item = Var> + '_ {
        [self.init.var()].into_iter()
    }

    fn representative_input_var(&self) -> Var {
        self.next
    }

    fn apply_var_map(&mut self, mut var_map: impl FnMut(Var) -> Lit) -> Pol {
        let mapped_next = var_map(self.next);
        let mapped_init = self.init.map_var_to_lit(var_map);

        *self = Reg {
            init: mapped_init ^ mapped_next.pol(),
            next: mapped_next.var(),
        };

        mapped_next.pol()
    }
}
