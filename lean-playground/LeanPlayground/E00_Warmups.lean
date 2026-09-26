-- Start here. Replace each `sorry` with a proof, then move to E01.
-- Put your cursor after `by` to see the goal in VS Code's Lean InfoView.
-- These warmups are adapted from the early examples in:
-- https://lean-lang.org/theorem_proving_in_lean4/Propositions-and-Proofs/

-- Worked example: the goal tells Lean that ⟨..., ...⟩ means And.intro.
example (P Q : Prop) (h : P ∧ Q) : Q ∧ P := by
  exact ⟨h.2, h.1⟩

-- 1. `intro` turns an implication goal into an assumption.
example (P Q : Prop) (h : P) : Q → P := by
  sorry

-- 2. An And proof contains both pieces; try h.1 or h.2.
example (P Q : Prop) (h : P ∧ Q) : P := by
  sorry

-- Embedded chapter 2 exercise: applying a function reduces by substitution.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Dependent-Type-Theory/
example (f : Nat → Nat) (n : Nat) : (fun x => f x) n = f n := by
  sorry

-- Worked example: `rw` uses an equality to change the goal.
example (n : Nat) : n + 0 = n := by
  rw [Nat.add_zero]

-- Worked example: `exact` supplies a proof of precisely the goal.
example (n : Nat) : n * 0 = 0 := by
  exact Nat.mul_zero n
