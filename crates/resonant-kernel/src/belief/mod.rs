//! Scoped belief: the canonical state machine, cells, and views.

pub mod cell;
pub mod state;
pub mod view;

pub use cell::{BeliefCell, Transition, TransitionError};
pub use state::{
    transition, BeliefEvent, BeliefState, EventKind, InvalidTransition, NarrowReason,
    QuarantineReason, RemovalBasis, RevocationEvent, RevocationSupport, RevocationTarget,
    SuspicionBasis, TRANSITION_TABLE,
};
pub use view::MembershipView;
