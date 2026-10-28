//! The names of our own span attributes, in the `quire.*` namespace.
//!
//! Pure constants and slug functions: no `tracing` dependency here. Spans are created only in the
//! daemons (inferd, companiond, readerd, cuad, memoryd, intentd), each in one `telemetry.rs`
//! whose `macro_rules!` span constructors spell these names as literals; a unit test there asserts
//! the macro's declared field set equals [`attr::ALL`] (and `genai-names` for the `gen_ai.*`
//! half). The OpenTelemetry GenAI names live in stoker's `genai-names`, the lowest repo.
//!
//! Privacy, hard: no prompt, completion, tool argument or result, screen text, title, recipient
//! or `SpaceId` ever goes on a span or metric. Counts, sizes, hashed ids, slugs and enums only.
//! `quire.space.hash` is a blake3 prefix of the id, keyed per install so it cannot be joined
//! across machines, never the id itself.

/// Span attribute keys.
pub mod attr {
    /// Declares each key as a constant and lists them all in [`ALL`], from the one list.
    macro_rules! keys {
        ($( $(#[$doc:meta])* $name:ident = $value:literal, )+) => {
            $( $(#[$doc])* pub const $name: &str = $value; )+

            /// Every key above, in declaration order: what a span macro's declared fields are
            /// tested against.
            pub const ALL: &[&str] = &[$($name),+];
        };
    }

    keys! {
        /// Who acted: a value of `ActorKind::slug()`.
        ACTOR_KIND = "quire.actor.kind",
        /// The effect class of an action: a value of `Effect::slug()`.
        EFFECT = "quire.effect",
        /// A keyed hash prefix of the `SpaceId`, never the id.
        SPACE_HASH = "quire.space.hash",
        /// The companion session.
        SESSION = "quire.session.id",
        /// The run.
        RUN = "quire.run.id",
        /// The task.
        TASK = "quire.task.id",
        /// Where a turn ran: `local`, `lan` or `cloud` (from `ServedBy`).
        SERVED_LOCALITY = "quire.served.locality",
        /// The code of a refusal.
        DENY_CODE = "quire.deny.code",
        /// The policy ruling.
        RULING = "quire.policy.ruling",
        /// How a confirmation ended.
        CONFIRM_OUTCOME = "quire.confirm.outcome",
        /// The pipeline stage.
        STAGE = "quire.pipeline.stage",
        /// The computer-use step number.
        CUA_STEP = "quire.cua.step",
        /// The computer-use mode.
        CUA_MODE = "quire.cua.mode",
    }
}
