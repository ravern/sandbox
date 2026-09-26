-- Chapter 3, exercises 18–25: classical propositional logic.
-- Try `classical` inside a proof when you need to split on an arbitrary Prop.
-- The final exercise is constructive: solve it without `classical`.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Propositions-and-Proofs/#exercises

variable (p q r : Prop)

-- 18–24: the book's classical exercises, in its original order.
example : (p → q ∨ r) → ((p → q) ∨ (p → r)) := by
  sorry

example : ¬(p ∧ q) → ¬p ∨ ¬q := by
  sorry

example : ¬(p → q) → p ∧ ¬q := by
  sorry

example : (p → q) → (¬p ∨ q) := by
  sorry

example : (¬q → ¬p) → (p → q) := by
  sorry

example : p ∨ ¬p := by
  sorry

example : (((p → q) → p) → p) := by
  sorry

-- 25: an apparent self-negating equivalence cannot hold.
-- Unpack both directions of ↔ and feed one into the other.
example : ¬(p ↔ ¬p) := by
  sorry
