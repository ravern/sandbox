-- Chapter 3, exercises 18–25: classical propositional logic.
-- In E01 every branch followed from evidence you already had. Here you may
-- need to decide an arbitrary proposition even when no proof is supplied.
-- Stay in term style: `Classical.em p` has type `p ∨ ¬p`. Match on that
-- result to get a branch with `hp : p` and one with `hnp : ¬p`.
-- The final exercise is constructive: solve it without `classical`.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Propositions-and-Proofs/#exercises

variable (p q r : Prop)

-- 18. You must choose whether to return a `p → q` or a `p → r` function.
-- Match on `Classical.em p`. If p is false, either function is vacuous;
-- if p is true, apply the given function and inspect its Or result.
example : (p → q ∨ r) → ((p → q) ∨ (p → r)) :=
  -- fun h => Or.inl (fun hp =>
  --             match h hp with
  --             | Or.inl hq => hq
  --             | Or.inr hr => ???)
  fun h =>
  match Classical.em p with
  | Or.inl hp =>
    match h hp with
    | Or.inl hq => Or.inl (fun _ => hq)
    | Or.inr hr => Or.inr (fun _ => hr)
  | Or.inr hnp => Or.inl (fun hp => False.elim (hnp hp))

-- 19. To produce `¬p ∨ ¬q`, match on `Classical.em p`.
-- In the ¬p branch you are done; in the p branch, any proof of q would
-- build the forbidden `p ∧ q`, so you can prove ¬q.
example : ¬(p ∧ q) → ¬p ∨ ¬q :=
  sorry

-- 20. The hypothesis says that every proposed function `p → q` fails.
-- If p were false, such a function would be easy to make. After showing p,
-- assume q and use it to make a constant `p → q`, contradicting the hypothesis.
example : ¬(p → q) → p ∧ ¬q :=
  sorry

-- 21. Match on `Classical.em p`. If p holds, the implication produces q;
-- otherwise the `¬p` side of the target Or is ready.
example : (p → q) → (¬p ∨ q) :=
  sorry

-- 22. You know the contrapositive and want the forward implication.
-- Write `fun hp => ...`, then match on `Classical.em q`. The ¬q branch
-- would give ¬p and contradict hp.
example : (¬q → ¬p) → (p → q) :=
  sorry

-- 23. This is excluded middle itself. Lean provides exactly this
-- proposition as the term `Classical.em p`.
example : p ∨ ¬p :=
  sorry

-- 24. Peirce's law: take a function `(p → q) → p`.
-- Match on `Classical.em p`. The p branch is immediate; in the ¬p branch, construct
-- `p → q` from the contradiction and feed it to the function.
example : (((p → q) → p) → p) :=
  (fun h =>
    match Classical.em p with
    | Or.inl hp => hp
    | Or.inr hnp => h (fun hp => False.elim (hnp hp)))

-- 25. `p ↔ ¬p` supplies both `p → ¬p` and `¬p → p`.
-- To prove its negation, assume it and construct `¬p` first: if p held,
-- the first direction would contradict it. Then use the second direction.
example : ¬(p ↔ ¬p) :=
  sorry
