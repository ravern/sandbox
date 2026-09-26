-- Start here. Replace each `sorry` with a proof, then move to E01.
-- E00–E06 use proof terms, as in the book before its tactics chapter.
-- Hover over a term or place the cursor after `:=` to inspect its type.
-- These warmups are adapted from the early examples in:
-- https://lean-lang.org/theorem_proving_in_lean4/Propositions-and-Proofs/

-- The expected type tells Lean that ⟨..., ...⟩ means And.intro.
example (P Q : Prop) (h : P ∧ Q) : Q ∧ P :=
  And.intro h.2 h.1

-- 1. A proof of Q → P is a function. It may ignore its Q input because
-- h already proves P. The underscore names an unused input.
example (P Q : Prop) (h : P) : Q → P := fun _ => h

-- 2. h.1 is itself a proof of P; no tactic command is needed.
example (P Q : Prop) (h : P ∧ Q) : P :=
  h.1

-- Embedded chapter 2 exercise: applying a function reduces by substitution.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Dependent-Type-Theory/
-- `rfl` here is a proof term. Lean reduces the function application
-- before comparing the two sides of the equality.
example (f : Nat → Nat) (n : Nat) : (fun x => f x) n = f n :=
  Eq.refl (f n)

-- Nat.add_zero n is already a proof of this exact equality.
example (n : Nat) : n + 0 = n :=
  Nat.add_zero n

-- Likewise, Nat.mul_zero n has the expected equality type.
example (n : Nat) : n * 0 = 0 :=
  Nat.mul_zero n
