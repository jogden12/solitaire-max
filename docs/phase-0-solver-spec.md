# Phase 0: Klondike solver spec

Goal: a command-line tool that, given a deal seed, reports whether the deal is winnable
and the minimum number of moves needed to win it. Nothing else.

Place this file in the repo at `docs/phase-0-solver-spec.md`.

## Exit test

The phase is finished when this command runs clean:

```
solitaire-solver bench --count 1000 --timeout 30
```

and the output shows:

1. 1,000 randomly seeded draw-1 deals attempted.
2. A winnability verdict for at least 950 of them.
3. A minimum move count for every deal judged winnable.
4. Mean wall-clock time per deal under 30 seconds on an ordinary laptop.
5. A winnable rate between 87% and 94%, measured as winnable deals divided by deals that
   got a verdict. The published rate for draw-1 Klondike with full information is
   90.480% ± 0.116% (Blake and Gent, "The Winnability of Klondike Solitaire and Many
   Other Patience Games", Table 1). The often-quoted 82% is the draw-3 figure. A rate
   outside this band means the rules or the solver are wrong, not that the deals are
   unusual.

Report the rate and stop if it falls outside the band.

How the exit test is measured:

- `bench` always runs both searches, winnability then minimum moves. It has no
  `--optimal` flag because it never skips the second search.
- `--timeout` is one budget per deal covering both searches.
- Mean time is the average of each deal's own `elapsed_ms`, so running deals in parallel
  does not change it.
- Seeds are sequential, starting at `--seed-start` (default `0000000000000001`), so
  every run attempts the same deals.
- The summary also reports how many winnable deals had their minimum proven. No pass
  mark is set for that number yet. It gets set once real numbers exist.

## Deal generation, frozen

Every deal is produced from a 64-bit seed. The same seed must produce the same deal on
any machine, in any language, forever. Do not use a language's built-in shuffle.

1. **Seed**: unsigned 64-bit integer, written as 16 lowercase hex characters.
2. **PRNG**: xoshiro256\*\*, with its four 64-bit state words initialized by running
   SplitMix64 four times from the seed. The first SplitMix64 output is state word 0, the
   fourth is state word 3. Both are the public reference algorithms by Blackman and
   Vigna, with all arithmetic wrapping at 64 bits:
   - SplitMix64 step: add `0x9E3779B97F4A7C15` to the state, call the result z, then
     `z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9`,
     `z = (z ^ (z >> 27)) * 0x94D049BB133111EB`, and output `z ^ (z >> 31)`.
   - xoshiro256\*\* step: output `rotl(s1 * 5, 7) * 9`, then `t = s1 << 17`,
     `s2 ^= s0`, `s3 ^= s1`, `s1 ^= s2`, `s0 ^= s3`, `s2 ^= t`, `s3 = rotl(s3, 45)`.
3. **Deck before shuffling**: index 0 to 51, suit-major. Suit order: clubs, diamonds,
   hearts, spades. Within a suit: ace, 2 through 10, jack, queen, king.
4. **Shuffle**: Fisher-Yates descending. For i from 51 down to 1, pick j uniformly in
   [0, i] and swap. Use rejection sampling for the uniform draw, not modulo, so the
   result has no bias. The exact method, because different methods give different
   deals: let n = i + 1 and t = 2^64 mod n. Draw a 64-bit value x from the PRNG. If
   x >= 2^64 - t, throw it away and draw again. Otherwise j = x mod n.
5. **Deal**: for i in 0..7, for j in i..7, deal the next card from the top of the deck to
   tableau pile j. The top of the deck is index 0, so cards are dealt in index order
   0, 1, 2 and so on. The last card placed on each pile is face up, the rest face down.
6. **Stock**: the remaining 24 cards, in order, face down. The first card drawn is deck
   index 28, the last is index 51. The waste starts empty.

Write a test that pins seed `0000000000000001` to its exact resulting layout, so any
future change to this code fails loudly. Before pinning, check the PRNG against
published reference outputs, so the pinned layout is not built on a bug.

## Rules, draw-1

- Foundations build up by suit from ace to king, four of them.
- Tableau builds down in alternating colors.
- Only a king, or a run headed by a king, may move to an empty column.
- Any face-up run that is already in valid descending alternating-color order may move as
  a unit.
- A card may move from a foundation back to the tableau.
- Drawing turns one card from stock to waste. Only the top waste card is playable.
- When the stock is empty, the waste may be turned back over to form a new stock.
  Redeals are unlimited. Turning it over keeps the order, so the cards come out of the
  new stock in the same order they came out of the old one.
- When a move exposes a face-down tableau card, it flips face up automatically.
- The game is won when all 52 cards are on the foundations. There is no auto-complete
  shortcut: every card moved to a foundation is a counted move.

## Move counting, frozen

Every efficiency number the product will ever show divides by this count. Freeze it now.

| Action | Notation | Counts as |
| --- | --- | --- |
| Draw one card from stock to waste | `D` | 1 move |
| Turn the waste back into the stock | `R` | 1 move |
| Waste to tableau pile n | `WT<n>` | 1 move |
| Waste to foundation of suit s | `WF<s>` | 1 move |
| Move k cards from tableau a to tableau b | `T<a>T<b>:<k>` | 1 move |
| Tableau pile n to foundation of suit s | `T<n>F<s>` | 1 move |
| Foundation of suit s to tableau pile n | `F<s>T<n>` | 1 move |
| Automatic flip of an exposed card | not recorded | 0 moves |

Piles are numbered 1 to 7, left to right. Suits are `c`, `d`, `h`, `s`. A solution is a
space-separated string of these tokens.

## Commands

```
solitaire-solver deal --seed <hex>
```
Prints the dealt layout as JSON. Used to verify determinism and to eyeball deals.

A card is written as rank then suit. Ranks are `A 2 3 4 5 6 7 8 9 T J Q K`, suits are
`c d h s`, so `Ac`, `Td`, `Kh`. The layout is one JSON object:

```json
{
  "seed": "0000000000000001",
  "draw": 1,
  "tableau": [
    {"down": [], "up": ["Kh"]},
    {"down": ["2c"], "up": ["7d"]}
  ],
  "stock": ["Ac", "9s"]
}
```

`tableau` has seven piles, left to right. `down` and `up` list a pile's face-down and
face-up cards from the bottom of the pile to the top. `stock` lists all 24 cards in the
order they will be drawn. The cards shown above are placeholders, not the real deal.

```
solitaire-solver solve --seed <hex> [--timeout <seconds>] [--optimal]
```
Without `--optimal`, answers winnability only. With it, also finds the shortest winning
solution. Prints one JSON object.

```
solitaire-solver bench --count <n> [--timeout <seconds>] [--seed-start <hex>]
```
Runs the exit test. Prints one JSON object per deal to stdout, then a summary to stderr.

## Output format

One JSON object per deal, one per line, so the Phase 1 pipeline can stream it:

```json
{
  "seed": "0000000000000001",
  "draw": 1,
  "winnable": true,
  "min_moves": 117,
  "optimal_proven": true,
  "solution": "D WT3 T7Fc ...",
  "nodes": 4821993,
  "elapsed_ms": 8120
}
```

- `winnable` is `true`, `false`, or `null` when the search hit the timeout.
- `optimal_proven` is `false` when a win was found but the search could not prove it was
  the shortest. `min_moves` then means best known, and the Phase 1 pipeline will label it
  that way to players.
- `solution` is omitted when `winnable` is not `true`. `min_moves` and `optimal_proven`
  are `null` in that case.

## Approach

Two different searches. Do not try to make one search do both jobs.

**Winnability** is a reachability question. Depth-first search with a transposition table
of visited states, cheap state hashing, and move ordering that prefers forced and
productive moves. Most deals resolve in well under a second this way.

**Minimum moves** is a shortest-path question. Use iterative-deepening A\*. An admissible
heuristic: 52 minus the number of cards already on foundations. Every move places at most
one card on a foundation, so this never overestimates. A stronger admissible bound is
worth exploring once the simple one works, but correctness comes first.

Run the winnability search first. Only attempt optimal search on deals already known to
be winnable.

Iterative-deepening A\* only produces an answer at the moment it proves the minimum. If
it runs out of time it has nothing better to offer than the first win the winnability
search stumbled on, which can be far too long. So add a shortening pass that keeps
improving the best known win, and report that when the minimum cannot be proven.

Every pruning rule in either search must be provably safe. `winnable: false` means
proven unwinnable, and `optimal_proven: true` means proven shortest. A shortcut that is
only usually right is not allowed, because every number the product shows rests on
these two claims.

## Things that will bite

- **Move explosion from foundation-to-tableau moves.** They are legal and sometimes
  necessary, but they let the search cycle. Prune them only where the cut can be shown
  never to remove a win or a shortest win.
- **Equivalent states reached by different move orders.** Canonicalize the state before
  hashing: sort empty columns, normalize the stock and waste as a single cycle. The
  stock and waste part applies to the winnability search only. The minimum-moves search
  must keep the position in the stock, because reaching a card costs draws.
- **Redeal loops.** Cycling the stock with no intervening change of state is never
  useful. Detect it and cut the branch.
- **Memory.** A transposition table over hard deals will grow without bound. Cap it and
  evict, rather than letting the process die.

## What is not in Phase 0

Difficulty ratings, the deal catalog, any database, any threading beyond simple
parallelism in `bench`, and anything to do with the app. Resist all of it.
