import Std

-- A sorting algorithm needs two promises: ordered output and no lost data.
-- Adapted from Software Foundations, Verified Functional Algorithms, Sort.
-- Source: https://softwarefoundations.cis.upenn.edu/vfa-current/Sort.html
-- The source uses Rocq's Permutation; this Lean version uses List.Perm.
-- Our Sorted predicate compares each head to every later element.

namespace Puzzles19

def insert (x : Nat) : List Nat → List Nat
  | [] => [x]
  | y :: ys => if x ≤ y then x :: y :: ys else y :: insert x ys

def sort : List Nat → List Nat
  | [] => []
  | x :: xs => insert x (sort xs)

def Sorted : List Nat → Prop
  | [] => True
  | x :: xs => (∀ y, y ∈ xs → x ≤ y) ∧ Sorted xs

-- 1. Warmup helper: insertion adds exactly one possible member value.
-- Induct on xs, split the `if`, and simplify membership in a cons list.
-- This statement concerns membership; the next one also tracks duplicates.
theorem mem_insert (x y : Nat) (xs : List Nat) : y ∈ insert x xs ↔ y = x ∨ y ∈ xs := by sorry

-- 2. A permutation may rearrange a list but cannot lose or duplicate
-- elements. Induct on xs. Explore `List.Perm.refl`, `.cons`, `.swap`,
-- and `.trans` to connect the two possible insertion branches.
theorem insert_perm (x : Nat) (xs : List Nat) : (insert x xs).Perm (x :: xs) := by sorry

-- 3. Insertion preserves order when its input was ordered. Induct on xs
-- and split on x ≤ the head. In the false branch the old head stays;
-- prove it is ≤ every new tail member, using mem_insert and h.
theorem insert_sorted (x : Nat) (xs : List Nat) (h : Sorted xs) : Sorted (insert x xs) := by sorry

-- 4. Apply insert_perm after recursively sorting the tail. The induction
-- hypothesis plus List.Perm.cons lets you relate x :: sort xs to x :: xs.
theorem sort_perm (xs : List Nat) : (sort xs).Perm xs := by sorry

-- 5. Induct on xs. The recursive output is ordered by the induction
-- hypothesis; exercise 3 says inserting the head preserves that property.
theorem sort_sorted (xs : List Nat) : Sorted (sort xs) := by sorry

-- 6. Put both promises together. Sortedness alone would let a "sort"
-- return [] for every input; permutation rules out that incorrect program.
theorem sort_correct (xs : List Nat) : Sorted (sort xs) ∧ (sort xs).Perm xs := by sorry

end Puzzles19
