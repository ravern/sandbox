import Std

-- Is one event log obtainable from another just by deleting events?
-- Adapted from Software Foundations, IndProp: subseq. The concrete example
-- and length bound are additional stepping stones for this Lean version.
-- Source: https://softwarefoundations.cis.upenn.edu/lf-current/IndProp.html

namespace Puzzles16

-- Subseq xs ys permits deletion from ys, but never changing event order.
-- `keep` matches the same head; `skip` discards a head from the larger list.
-- We supply the relation so the exercises focus on proofs about it.
inductive Subseq : List α → List α → Prop where
  | nil (ys : List α) : Subseq [] ys
  | keep (x : α) {xs ys : List α} : Subseq xs ys → Subseq (x :: xs) (x :: ys)
  | skip (x : α) {xs ys : List α} : Subseq xs ys → Subseq xs (x :: ys)

-- 1. Keep 1, skip 2, keep 3, then finish with the empty-list constructor.
-- This also demonstrates that a subsequence need not be contiguous.
example : Subseq ([1, 3] : List Nat) [1, 2, 3] := by sorry

-- 2. A log is a subsequence of itself: induct on xs and keep every event.
theorem reflexive (xs : List α) : Subseq xs xs := by sorry

-- 3. Adding events at the end of the larger log cannot invalidate an
-- existing witness. Induct on h; each constructor has a corresponding
-- way to rebuild the result. Only the base case absorbs the new suffix.
theorem append_right {xs ys : List α} (h : Subseq xs ys) (zs : List α) : Subseq xs (ys ++ zs) := by
  sorry

-- 4. Deleting events cannot make a longer list. Induct on h; keep adds
-- one to both lengths, whereas skip adds one only to the right length.
-- `omega` can finish the small arithmetic obligations after simplification.
theorem length_bound {xs ys : List α} (h : Subseq xs ys) : xs.length ≤ ys.length := by sorry

-- 5. Challenge: deleting from a deletion is still a deletion.
-- Induct on hyz while keeping xs general (`generalizing xs`). In its
-- keep case, inspect hxy to discover whether it keeps or skips that head.
theorem transitive {xs ys zs : List α} (hxy : Subseq xs ys) (hyz : Subseq ys zs) :
    Subseq xs zs := by sorry

end Puzzles16
