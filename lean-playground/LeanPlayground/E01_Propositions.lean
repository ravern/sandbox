-- Chapter 3, exercises 1–17: constructive propositional logic.
-- Work top to bottom; later proofs combine earlier moves.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Propositions-and-Proofs/#exercises

variable (p q r : Prop)

-- 1–2: commute And and Or. For ↔, prove each direction.
-- And has one constructor; Or requires a left or right choice.
example : p ∧ q ↔ q ∧ p := by
  sorry

example : p ∨ q ↔ q ∨ p := by
  sorry

-- 3–4: associativity. Unpack nested evidence, then rebuild it.
example : (p ∧ q) ∧ r ↔ p ∧ (q ∧ r) := by
  sorry

example : (p ∨ q) ∨ r ↔ p ∨ (q ∨ r) := by
  sorry

-- 5–6: distributivity. `cases` helps with each Or.
example : p ∧ (q ∨ r) ↔ (p ∧ q) ∨ (p ∧ r) := by
  sorry

example : p ∨ (q ∧ r) ↔ (p ∨ q) ∧ (p ∨ r) := by
  sorry

-- 7–8: implications. Treat `→` as a function.
example : (p → (q → r)) ↔ (p ∧ q → r) := by
  sorry

example : ((p ∨ q) → r) ↔ (p → r) ∧ (q → r) := by
  sorry

-- 9–14: negation is an implication to False.
-- To prove ¬p, assume p and derive a contradiction.
example : ¬(p ∨ q) ↔ ¬p ∧ ¬q := by
  sorry

example : ¬p ∨ ¬q → ¬(p ∧ q) := by
  sorry

example : ¬(p ∧ ¬p) := by
  sorry

example : p ∧ ¬q → ¬(p → q) := by
  sorry

example : ¬p → (p → q) := by
  sorry

example : (¬p ∨ q) → (p → q) := by
  sorry

-- 15–16: False can eliminate a branch; it cannot supply evidence itself.
example : p ∨ False ↔ p := by
  sorry

example : p ∧ False ↔ False := by
  sorry

-- 17: contraposition needs only constructive reasoning.
example : (p → q) → (¬q → ¬p) := by
  sorry
