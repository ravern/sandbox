-- Chapter 4, exercise 5: all eleven identities from the existential section.
-- They appear before the chapter's exercise list; the list asks you to revisit them.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Quantifiers-and-Equality/#the-existential-quantifier

variable (α : Type) (p q : α → Prop) (r : Prop)

-- 1–4: a proof of ∃ x contains a witness x and a proof about x.
example : (∃ x : α, r) → r := by
  sorry

-- The given a lets you construct a witness even if α has no other elements.
example (a : α) : r → (∃ x : α, r) := by
  sorry

example : (∃ x, p x ∧ r) ↔ (∃ x, p x) ∧ r := by
  sorry

example : (∃ x, p x ∨ q x) ↔ (∃ x, p x) ∨ (∃ x, q x) := by
  sorry

-- 5–8: relationships between ∀, ∃, and ¬.
-- Work out which directions need `classical`.
example : (∀ x, p x) ↔ ¬ (∃ x, ¬ p x) := by
  sorry

example : (∃ x, p x) ↔ ¬ (∀ x, ¬ p x) := by
  sorry

example : (¬ ∃ x, p x) ↔ (∀ x, ¬ p x) := by
  sorry

example : (¬ ∀ x, p x) ↔ (∃ x, ¬ p x) := by
  sorry

-- 9–11: moving an implication across an existential.
-- In 10 and 11, a : α prevents an empty-type obstruction.
example : (∀ x, p x → r) ↔ (∃ x, p x) → r := by
  sorry

example (a : α) : (∃ x, p x → r) ↔ (∀ x, p x) → r := by
  sorry

example (a : α) : (∃ x, r → p x) ↔ (r → ∃ x, p x) := by
  sorry
