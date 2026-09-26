-- Chapter 5 exercises: revisit earlier puzzles with tactics, then try this one.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Tactics/#exercises

-- 1. Redo the E01–E06 exercises using tactic proofs. Try `intro`,
-- `constructor`, `cases`, `left`, `right`, `rw`, and `simp` where appropriate.
-- Reusing the original files keeps your earlier proofs beside the new ones.

-- 2. Solve this in one line with tactic combinators.
-- Since hp : p, each Or goal has a branch that `exact hp` can finish.
example (p q r : Prop) (hp : p) :
    (p ∨ q ∨ r) ∧ (q ∨ p ∨ r) ∧ (q ∨ r ∨ p) := by
  sorry

-- Extra worked arithmetic example from the chapter: rewrite in stages.
example (a b c : Nat) : a + b + c = a + c + b := by
  sorry
