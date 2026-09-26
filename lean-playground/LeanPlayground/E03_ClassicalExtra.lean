-- An additional exercise embedded in chapter 3, before its exercise list.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Propositions-and-Proofs/#classical-logic

-- `dne` is an assumed rule: if assuming ¬p leads to False, it returns p.
-- Your goal is to derive excluded middle, `p ∨ ¬p`, for an arbitrary p.
-- Start with `fun p => dne (p ∨ ¬p) ...`. The remaining argument must
-- have type `¬¬(p ∨ ¬p)`: take a function saying the Or is impossible,
-- derive ¬p from that assumption, and use that ¬p as the Or's right side.
-- Do not invoke `Classical.em`; the point is to derive it from `dne`.
example (dne : ∀ p : Prop, ¬¬p → p) : ∀ p : Prop, p ∨ ¬p :=
  sorry
