//! AC1: `day-stamp seed` against an empty
//! `$XDG_CONFIG_HOME/daily-receipt/stamps/` writes 4 starter stamp
//! files. Re-running `seed` does not overwrite; exits 0.
//!
//! Read-only after scaffold: the edit-agent must NOT modify acceptance
//! tests. If a test is wrong, write agent/intent_card_amendment_request.json.

#[test]
fn ac1_seed_idempotent_placeholder() {
    // Scaffold stub: replaced when iter-1 implements `day-stamp seed`.
}
