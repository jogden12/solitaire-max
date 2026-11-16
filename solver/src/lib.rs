//! Klondike solitaire solver.
//!
//! This library holds all the real logic: dealing, rules, and the two searches.
//! The command-line program in `main.rs` is a thin wrapper around it, so the same
//! code can be called from the iOS app later.
//!
//! The rules it follows are written down in `docs/phase-0-solver-spec.md`.

/// Cards in a standard deck.
pub const DECK_SIZE: usize = 52;

/// Tableau piles, numbered 1 to 7 from left to right.
pub const TABLEAU_PILES: usize = 7;

/// Cards left over for the stock once the tableau has been dealt.
pub const STOCK_SIZE: usize = 24;

/// Cards dealt to the tableau: pile 1 gets one card, pile 2 gets two, up to pile 7.
pub fn tableau_card_count() -> usize {
    (1..=TABLEAU_PILES).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tableau_and_stock_use_the_whole_deck() {
        assert_eq!(tableau_card_count(), 28);
        assert_eq!(tableau_card_count() + STOCK_SIZE, DECK_SIZE);
    }
}
