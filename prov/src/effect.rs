//! What an action can do to the world: the four effects of the companion's rule 9, and the
//! fifth, running a command.

use crate::slug::slug_enum;
use serde::{Deserialize, Serialize};

slug_enum! {
    /// The effect class of an action, ordered by severity: `Read < UndoableWrite < Outbound <
    /// Destructive < Execute`. Policy compares with `>=`; the router asks per class. `Execute` is
    /// the top: a command can do anything its sandbox allows, so it ranks at least as high as every
    /// other class (a rule that asks from `Outbound` up, or from `Destructive` up, asks about it).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
    pub enum Effect {
        /// Looks, changes nothing.
        Read = "read",
        /// Changes something the app can take back (archive, move, label, a draft).
        UndoableWrite = "undoable_write",
        /// Sends something to another person or service.
        Outbound = "outbound",
        /// Removes something for good.
        Destructive = "destructive",
        /// Runs a command (a sandboxed shell): what it does is whatever the sandbox allows, which
        /// can be any of the above. Added after the first four; a record written before it never
        /// holds it.
        Execute = "execute",
    }
}
