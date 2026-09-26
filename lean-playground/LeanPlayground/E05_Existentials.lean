-- Chapter 4, exercise 5: all eleven identities from the existential section.
-- They appear before the chapter's exercise list; the list asks you to revisit them.
-- `∃ x, p x` contains a *particular* witness x and a proof of p x.
-- To use `h : ∃ x, p x`, match it: `match h with | ⟨x, hx⟩ => ...`.
-- To build one, write `⟨chosenX, proofOfPChosenX⟩`.
-- To prove an `↔`, write `Iff.intro (fun h => ...) (fun h => ...)`.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Quantifiers-and-Equality/#the-existential-quantifier

variable (α : Type) (p q : α → Prop) (r : Prop)

-- 1. The statement r is independent of x. Unpack the existential;
-- its second component is already a proof of r.
example : (∃ x : α, r) → r :=
  sorry

-- 2. The supplied `a : α` is your witness. Build `⟨a, ...⟩` and fill
-- the remaining r goal with the proof given by the implication.
-- Without a, α could be empty, so there might be no witness to choose.
example (a : α) : r → (∃ x : α, r) :=
  sorry

-- 3. Forward: unpack the witness and its `p x ∧ r`; keep the same x
-- for the left target and move r to the right. Reverse: unpack the
-- left existential and combine its p proof with the separate r proof.
example : (∃ x, p x ∧ r) ↔ (∃ x, p x) ∧ r :=
  sorry

-- 4. Forward: unpack x, then inspect whether `p x` or `q x` holds.
-- Keep x when constructing the appropriate target existential.
-- Reverse: split the outer Or and reuse the witness from that branch.
example : (∃ x, p x ∨ q x) ↔ (∃ x, p x) ∨ (∃ x, q x) :=
  sorry

-- 5–8 compare quantifiers with negation. It helps to expand `¬A`
-- mentally to `A → False`. Some reverse directions need a classical
-- term such as `Classical.em A` or `Classical.byContradiction`:
-- failing to prove `∀ x, p x` does not constructively give a bad x.

-- 5. Forward: an alleged `⟨x, ¬p x⟩` contradicts the universal proof
-- at x. Reverse: to get p x from "not not p x", use classical reasoning.
example : (∀ x, p x) ↔ ¬(∃ x, ¬p x) :=
  sorry

-- 6. Forward: a witness for p x refutes a claim that every x has ¬p x.
-- Reverse: classical reasoning turns "there cannot be no witness"
-- into an actual witness.
example : (∃ x, p x) ↔ ¬(∀ x, ¬p x) :=
  sorry

-- 7. Both directions are constructive. A proof of `¬∃ x, p x`
-- turns any hypothetical `p x` into a contradiction; conversely,
-- a universal `¬p x` refutes the witness inside an existential.
example : (¬∃ x, p x) ↔ (∀ x, ¬p x) :=
  sorry

-- 8. A counterexample `⟨x, ¬p x⟩` plainly refutes `∀ x, p x`.
-- The opposite direction, finding a particular x from a failed
-- universal claim, is the part that needs classical reasoning.
example : (¬∀ x, p x) ↔ (∃ x, ¬p x) :=
  sorry

-- 9. Forward: given `⟨x, p x⟩`, specialize the universal function at x.
-- Reverse: given x and p x, package them as an existential and apply
-- the function on the right. This equivalence is constructive.
example : (∀ x, p x → r) ↔ (∃ x, p x) → r :=
  sorry

-- 10. Forward: an existential provides a particular x and a function
-- `p x → r`; a universal p proof supplies its input. Reverse is harder:
-- match on whether `∀ x, p x` holds using `Classical.em`. The supplied a gives
-- a witness in the first case; a counterexample to p gives one in the other.
example (a : α) : (∃ x, p x → r) ↔ (∀ x, p x) → r :=
  sorry

-- 11. Forward: when r is supplied, apply the existential's function
-- to produce p at its witness. Reverse: match on `Classical.em r`.
-- If r holds, use the witness returned by `r → ∃ x, p x`; otherwise
-- choose a and make `r → p a` from the contradiction.
example (a : α) : (∃ x, r → p x) ↔ (r → ∃ x, p x) :=
  sorry
