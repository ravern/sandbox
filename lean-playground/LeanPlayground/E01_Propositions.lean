-- Chapter 3, exercises 1–17: constructive propositional logic.
-- A proof is a value of the proposition's type. Before chapter 5,
-- write that value directly after `:=`; keep `by` tactics for E07.
-- Read the outermost symbol first: `p → q` needs `fun hp => ...`,
-- `p ∧ q` needs `And.intro` or `⟨..., ...⟩`, and `p ∨ q` needs
-- `Or.inl` or `Or.inr`. An `↔` needs `Iff.intro` with two functions.
-- For a hypothesis `h : p ∧ q`, use `h.1 : p` and `h.2 : q`.
-- For `h : p ∨ q`, use `match h with | Or.inl hp => ... | Or.inr hq => ...`
-- or `Or.elim h (fun hp => ...) (fun hq => ...)`.
-- Work top to bottom; later exercises combine these moves.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Propositions-and-Proofs/#exercises

variable (p q r : Prop)

-- 1. "p and q" contains both proofs, so their order can be swapped.
-- Start with `Iff.intro (fun h => ...) (fun h => ...)`. Each `h` is an
-- And proof; build the reversed And from `h.2` and `h.1`.
example : p ∧ q ↔ q ∧ p :=
  Iff.intro (fun ⟨p, q⟩ => And.intro q p) (fun h => And.intro h.2 h.1)

-- 2. "p or q" supplies one proof, but you do not know which one.
-- Each direction starts `fun h => ...`. Inspect `h` with `match`;
-- use `Or.inr` for a p proof or `Or.inl` for a q proof in the new Or.
example : p ∨ q ↔ q ∨ p :=
  Iff.intro
    (fun h =>
      match h with
      | Or.inl p => Or.inr p
      | Or.inr q => Or.inl q)
    (fun h =>
      match h with
      | Or.inl q => Or.inr q
      | Or.inr p => Or.inl p)

-- 3. Parentheses change how the And proofs are nested, not what they contain.
-- In the forward direction, `h.1` itself has two pieces; `h.2` is the third.
-- Reassemble those three pieces in the nesting shown on the right.
example : (p ∧ q) ∧ r ↔ p ∧ (q ∧ r) :=
  Iff.intro (fun ⟨⟨p, q⟩, r⟩ => And.intro p (And.intro q r))
    (fun ⟨p, ⟨q, r⟩⟩ => And.intro (And.intro p q) r)

-- 4. A nested Or is a choice with three possible destinations.
-- Match the outer Or, then match the inner Or where needed.
-- In each branch, nest `Or.inl` and `Or.inr` to reach the desired shape.
example : (p ∨ q) ∨ r ↔ p ∨ (q ∨ r) :=
  Iff.intro
    (fun h =>
      match h with
      | Or.inl g =>
        (match g with
        | Or.inl p => Or.inl p
        | Or.inr q => Or.inr (Or.inl q))
      | Or.inr r => Or.inr (Or.inr r))
    (fun h =>
      match h with
      | Or.inl p => Or.inl (Or.inl p)
      | Or.inr g =>
        (match g with
        | Or.inl q => Or.inl (Or.inr q)
        | Or.inr r => Or.inr r))

-- 5. Forward: take the p proof out of the And, then inspect whether
-- the other part proves q or r. Backward: inspect which And you received
-- and reconstruct `p ∧ (q ∨ r)` with the appropriate Or branch.
example : p ∧ (q ∨ r) ↔ (p ∧ q) ∨ (p ∧ r) :=
  sorry

-- 6. Forward: when p holds, it proves both target Ors; when q ∧ r holds,
-- use q for the first and r for the second. Backward: inspect the two
-- Or proofs. If either supplies p, use it; otherwise you have q and r.
example : p ∨ (q ∧ r) ↔ (p ∨ q) ∧ (p ∨ r) :=
  sorry

-- 7. `p → q → r` means: given p, then q, produce r.
-- In one direction, obtain p and q from an And. In the other direction,
-- use `fun hp hq => ...` and package those inputs into an And.
example : (p → (q → r)) ↔ (p ∧ q → r) :=
  Iff.intro (fun h => (fun ⟨hp, hq⟩ => h hp hq)) (fun h => (fun hp hq => h (And.intro hp hq)))

-- 8. A function that consumes `p ∨ q` must handle either input.
-- Forward: make a function for p and another for q by wrapping each
-- input in an Or. Backward: split an incoming Or and call the right function.
example : ((p ∨ q) → r) ↔ (p → r) ∧ (q → r) :=
  sorry

-- From here, `¬p` is shorthand for `p → False`. To prove a negation,
-- use `fun hp => ...` and produce False from an existing contradiction.

-- 9. If neither p nor q can hold, their Or cannot hold. Conversely,
-- a proof that `p ∨ q` is impossible rules out p and q separately.
-- Remember that `↔` still requires two directions.
example : ¬(p ∨ q) ↔ ¬p ∧ ¬q :=
  Iff.intro (fun h => And.intro (fun hp => h (Or.inl hp)) (fun hq => h (Or.inr hq)))
    (fun ⟨hp', hq'⟩ =>
      (fun h =>
        match h with
        | Or.inl p => hp' p
        | Or.inr q => hq' q))

-- 10. Match the input `¬p ∨ ¬q`. In the first branch, a supposed
-- `p ∧ q` supplies p and contradicts ¬p; the second branch is symmetric.
example : ¬p ∨ ¬q → ¬(p ∧ q) :=
  sorry

-- 11. Suppose `p ∧ ¬p` exists. Extract both halves and apply the
-- negation proof to the positive proof to obtain False.
example : ¬(p ∧ ¬p) :=
  (fun ⟨hp, hp'⟩ => hp' hp)

-- 12. Assume a function `p → q` exists. The input And gives you p,
-- so the function produces q; the other half of the And forbids q.
example : p ∧ ¬q → ¬(p → q) :=
  sorry

-- 13. To prove `p → q`, write a function taking p. The given `¬p` turns that into
-- False, and `False.elim` can finish a goal of any proposition q.
example : ¬p → (p → q) :=
  (fun hp' => (fun hp => False.elim (hp' hp)))

-- 14. Match the outer Or. A proof of q directly answers the function's
-- output; a proof of ¬p rules out any p input as in the previous puzzle.
example : (¬p ∨ q) → (p → q) :=
  sorry

-- 15. Forward: match `p ∨ False`; the False case can prove p via
-- `False.elim`. Backward: a proof of p goes into `Or.inl`.
example : p ∨ False ↔ p :=
  Iff.intro
    (fun h =>
      match h with
      | Or.inl hp => hp
      | Or.inr h' => False.elim h')
    (fun hp => Or.inl hp)

-- 16. Forward: the right half of `p ∧ False` is already False.
-- Backward: from False, use `False.elim` to build the required And.
example : p ∧ False ↔ False :=
  sorry

-- 17. Assume `p → q`, then `¬q`, then p. Apply the first function to p
-- and the second to its result. No case split on whether p is true is needed.
example : (p → q) → (¬q → ¬p) :=
  (fun hpq => (fun hq' => (fun hp => hq' (hpq hp))))
