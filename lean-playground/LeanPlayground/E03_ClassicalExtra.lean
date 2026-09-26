-- An additional exercise embedded in chapter 3, before its exercise list.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Propositions-and-Proofs/#classical-logic

-- Derive excluded middle from double-negation elimination.
-- Treat `dne` as a supplied rule; do not use `classical` in the proof.
example (dne : ∀ p : Prop, ¬¬p → p) : ∀ p : Prop, p ∨ ¬p := by
  sorry
