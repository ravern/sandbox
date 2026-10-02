-- Chapter 5 exercises: revisit earlier puzzles with tactics, then try this one.
-- This is the first exercise file that uses `by`. It opens tactic mode:
-- Lean displays a goal, and each command transforms or finishes it.
-- `intro h` turns an implication into an assumption, `constructor`
-- creates the fields of an And or ↔, and `cases h` examines constructors
-- of an Or or existential hypothesis. `exact term` supplies a final proof.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Tactics/#exercises

-- 1. Choose some statements from E01–E06, copy the `example ... : ...`
-- line into this file, and replace its term with `by` plus tactic commands.
-- Keep your original term proofs for comparison. For an Or goal, `left`
-- selects its left constructor and `right` selects its right constructor.
-- `rw [lemma]` rewrites by an equality; `simp` uses registered rewrite
-- rules automatically. Try explicit steps before compressing a proof.

variable (α : Type) (p q : α → Prop)

example : (∀ x, p x → q x) → (∀ x, p x) → (∀ x, q x) := by
  intro hpq hp x
  exact hpq x (hp x)

example : (∀ x, p x) ↔ ¬(∃ x, ¬p x) := by
  constructor
  · intro hp ⟨x, hnp⟩
    exact hnp (hp x)
  · intro hxnp x
    apply Classical.byContradiction
    intro hnp
    exact hxnp ⟨x, hnp⟩

-- 2. The target is three And components; each component is an Or in
-- which p appears at a different position. First solve it with several
-- lines of `constructor`, `left`/`right`, and `exact hp`.
-- Then investigate `<;>` (apply the next tactic to every new goal)
-- and `first | ...` (try alternatives) to make a one-line proof.
example (p q r : Prop) (hp : p) : (p ∨ q ∨ r) ∧ (q ∨ p ∨ r) ∧ (q ∨ r ∨ p) := by sorry

-- Extra arithmetic exercise from the chapter. Addition groups to the
-- left, so expose the `b + c` subexpression with `Nat.add_assoc`, swap
-- those two terms with `Nat.add_comm`, then restore the grouping.
-- The `rw [...]` brackets contain a list of equalities to apply in order.
example (a b c : Nat) : a + b + c = a + c + b := by sorry
