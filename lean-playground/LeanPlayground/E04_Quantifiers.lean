-- Chapter 4, exercises 1–3: universal quantifiers and the barber paradox.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Quantifiers-and-Equality/#exercises

section Universal
variable (α : Type) (p q : α → Prop)

-- 1a: `intro x` picks an arbitrary element; split the And at that x.
example : (∀ x, p x ∧ q x) ↔ (∀ x, p x) ∧ (∀ x, q x) := by
  sorry

-- 1b: apply the first universal fact to the same x as the second.
example : (∀ x, p x → q x) → (∀ x, p x) → (∀ x, q x) := by
  sorry

-- 1c: split the outer Or before choosing an arbitrary x.
example : (∀ x, p x) ∨ (∀ x, q x) → ∀ x, p x ∨ q x := by
  sorry

-- Think: why can't the reverse of 1c be proved for arbitrary α, p, q?
end Universal

section MoveQuantifiers
variable (α : Type) (p : α → Prop) (r : Prop)

-- 2a: the input `α` matters: an arbitrary type might be empty.
example : α → ((∀ x : α, r) ↔ r) := by
  sorry

-- 2b: one direction needs classical reasoning. Can you spot which?
example : (∀ x, p x ∨ r) ↔ (∀ x, p x) ∨ r := by
  sorry

-- 2c: r does not depend on x.
example : (∀ x, r → p x) ↔ (r → ∀ x, p x) := by
  sorry
end MoveQuantifiers

section Barber
variable (men : Type) (barber : men)
variable (shaves : men → men → Prop)

-- 3: instantiate the universal claim with the barber himself.
-- If he shaves himself, use the right side of ↔; otherwise use the left.
example (h : ∀ x : men, shaves barber x ↔ ¬ shaves x x) : False := by
  sorry
end Barber
