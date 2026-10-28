//! How the `desktop` scope meets a Space: the join rule for confidentiality and where data may
//! flow.
//!
//! The reserved `desktop` Space holds only facts the person stated themselves (preferences,
//! names, style; QUESTIONS Q9). It is a sub-scope of every Space: reading a `desktop` fact into
//! a `work` task must not widen the task's confidentiality to `{desktop, work}`, because that
//! would stop the task from ever flowing anywhere `work` data may flow. So in a join the
//! `desktop` member disappears whenever another Space is present.

use crate::label::Confidentiality;
use porter_core::SpaceId;
use std::collections::BTreeSet;

/// `spaces` without `desktop` when any other Space is present.
fn normalise(spaces: BTreeSet<SpaceId>) -> BTreeSet<SpaceId> {
    let desktop = SpaceId::desktop();
    if spaces.len() > 1 {
        spaces.into_iter().filter(|s| s != &desktop).collect()
    } else {
        spaces
    }
}

impl Confidentiality {
    /// The confidentiality of anything derived from both: `Secret` if either is, else the
    /// other when one is `Public`, else the union of the Spaces with the `desktop` member
    /// dropped when another Space is present. Commutative, associative and idempotent on
    /// normalised values; a non-normalised `Private` is normalised on the way through.
    pub fn join(&self, other: &Confidentiality) -> Confidentiality {
        match (self, other) {
            (Confidentiality::Secret, _) | (_, Confidentiality::Secret) => Confidentiality::Secret,
            (Confidentiality::Public, Confidentiality::Public) => Confidentiality::Public,
            (Confidentiality::Public, Confidentiality::Private(s))
            | (Confidentiality::Private(s), Confidentiality::Public) => {
                Confidentiality::Private(normalise(s.clone()))
            }
            (Confidentiality::Private(a), Confidentiality::Private(b)) => {
                Confidentiality::Private(normalise(a.union(b).cloned().collect()))
            }
        }
    }

    /// Whether data this confidential may go into `space`: `Public` anywhere; `Private` where
    /// every named Space is `space` or `desktop`; `Secret` nowhere.
    pub fn flow_to(&self, space: &SpaceId) -> bool {
        let desktop = SpaceId::desktop();
        match self {
            Confidentiality::Public => true,
            Confidentiality::Private(spaces) => spaces.iter().all(|s| s == space || s == &desktop),
            Confidentiality::Secret => false,
        }
    }
}
