-- Chapter 4, exercises 1–3: universal quantifiers and the barber paradox.
-- Read `∀ x : α, p x` as a function that accepts any `x : α` and returns
-- a proof of `p x`. To prove it, write `fun x => ...`; to use
-- `h : ∀ x, p x`, apply it to a chosen value: `h x : p x`.
-- Unlike a plain proposition p, `p : α → Prop` may differ at each x.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Quantifiers-and-Equality/#exercises

section Universal

variable (α : Type) (p q : α → Prop)

-- 1a. A proof that every x satisfies both properties can be separated
-- into two universal proofs. Start with `Iff.intro`; in the forward
-- function, build an And of two functions `fun x => ...`.
-- Inspect `(h x).1`/`.2` to get the needed proof at each x.
-- For the reverse direction, apply both universal proofs to the same x.
example : (∀ x, p x ∧ q x) ↔ (∀ x, p x) ∧ (∀ x, q x) :=
  sorry

-- 1b. Write a function taking the two universal proofs and then x.
-- The first gives `p x → q x`; the second gives `p x`. Apply one to the other.
example : (∀ x, p x → q x) → (∀ x, p x) → (∀ x, q x) :=
  sorry

-- 1c. The input chooses one property that holds for *every* x.
-- Match that Or first; only then choose x and inject its proof into
-- the corresponding side of `p x ∨ q x`.
example : (∀ x, p x) ∨ (∀ x, q x) → ∀ x, p x ∨ q x :=
  sorry

-- Why no reverse? For α = Bool, let p x mean x = true and q x mean
-- x = false. Every x satisfies one side, but neither side holds for all x.
end Universal

section MoveQuantifiers

variable (α : Type) (p : α → Prop) (r : Prop)

-- 2a. The initial `α →` gives you an actual value `a : α`.
-- From `∀ x : α, r`, specialize at a to obtain r. Conversely, since r
-- does not mention x, the same proof of r works for every x.
-- The value a matters: an arbitrary type α could otherwise be empty.
example : α → ((∀ x : α, r) ↔ r) :=
  sorry

-- 2b. Backward: if every x satisfies p, choose the left side of each Or;
-- if r holds, choose the right side for every x. Forward is harder:
-- match on `Classical.em r`. In the ¬r case,
-- each `p x ∨ r` must have come from p x.
example : (∀ x, p x ∨ r) ↔ (∀ x, p x) ∨ r :=
  sorry

-- 2c. In either direction, write functions for r and for x.
-- The only question is whether the function takes x before or after r;
-- the given implication can then be applied to both inputs.
example : (∀ x, r → p x) ↔ (r → ∀ x, p x) :=
  sorry

end MoveQuantifiers

section Barber

variable (men : Type) (barber : men)
variable (shaves : men → men → Prop)

-- 3. Set x to barber in h. The result says that the proposition
-- `shaves barber barber` is equivalent to its own negation.
-- You proved that such an equivalence is impossible in E02's final puzzle;
-- you can reconstruct that argument here without assuming excluded middle.
example (h : ∀ x : men, shaves barber x ↔ ¬shaves x x) : False :=
  sorry

end Barber
