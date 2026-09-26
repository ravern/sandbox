-- Chapter 7, exercises 1–2: define operations and prove their laws.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Inductive-Types/#exercises

namespace Chapter07Nat

-- 1. Fill in recursive definitions of multiplication, predecessor,
-- truncated subtraction, and exponentiation on Nat.
-- Choose equations such as pred 0 = 0 and sub n m = 0 when m ≥ n.
-- You can start with `match n with | 0 => ... | k + 1 => ...`.
def mul (m n : Nat) : Nat := sorry
def pred (n : Nat) : Nat := sorry
def sub (n m : Nat) : Nat := sorry
def pow (n k : Nat) : Nat := sorry

-- Prove basic properties after replacing the definition placeholders.
-- These are suggested laws; the book leaves the choice of laws open.
example (n : Nat) : mul n 0 = 0 := by
  sorry

example : pred 0 = 0 := by
  sorry

example (n : Nat) : sub n 0 = n := by
  sorry

example (n : Nat) : pow n 0 = 1 := by
  sorry

end Chapter07Nat

namespace Chapter07Lists

-- 2. Define length and reverse recursively. The three target laws
-- are the exact ones suggested in the chapter exercise.
def length {α : Type} (xs : List α) : Nat := sorry
def reverse {α : Type} (xs : List α) : List α := sorry

example {α : Type} (xs ys : List α) :
    length (xs ++ ys) = length xs + length ys := by
  sorry

example {α : Type} (xs : List α) :
    length (reverse xs) = length xs := by
  sorry

example {α : Type} (xs : List α) : reverse (reverse xs) = xs := by
  sorry

end Chapter07Lists
