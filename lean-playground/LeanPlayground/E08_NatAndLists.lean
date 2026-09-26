-- Chapter 7, exercises 1–2: define operations and prove their laws.
-- These are programming exercises followed by proof exercises. A recursive
-- definition needs a base case and a case that calls itself on a smaller
-- input. For a theorem about every Nat, `induction n with | zero => ...
-- | succ k ih => ...` gives a base case and a step with induction
-- hypothesis `ih`. Definitions live in namespaces to avoid clashing with
-- Lean's built-in operations of the same names.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Inductive-Types/#exercises

namespace Chapter07Nat

-- 1. Define arithmetic by reducing one input at a time. For example,
-- multiplication by 0 is 0; multiplication by k+1 adds one more copy
-- of m to the result for k. Exponentiation is similar, but multiplies
-- by n at each successor step and starts at 1.
-- `pred 0` should be 0 and `pred (k+1)` should be k. Subtraction is
-- truncated: subtracting past zero stays at zero rather than negative.
-- Use pattern matching such as `match n with | 0 => ... | k + 1 => ...`.
def mul (m n : Nat) : Nat :=
  sorry

def pred (n : Nat) : Nat :=
  sorry

def sub (n m : Nat) : Nat :=
  sorry

def pow (n k : Nat) : Nat :=
  sorry

-- These are suggested laws; the book leaves the choice of laws open.
-- Finish the definitions before attempting them. Some base cases reduce
-- by computation (`rfl`); others may need `induction` and a recursive law.
-- `mul n 0` uses the zero branch of *your* mul, not Nat's built-in `*`.
example (n : Nat) : mul n 0 = 0 := by sorry

-- This checks the boundary case you chose for pred.
example : pred 0 = 0 := by sorry

-- Subtracting zero should leave n unchanged, including n = 0.
example (n : Nat) : sub n 0 = n := by sorry

-- Any number to the zeroth power is 1 for this Nat definition.
example (n : Nat) : pow n 0 = 1 := by sorry

end Chapter07Nat

namespace Chapter07Lists

-- 2. A List is either `[]` or `x :: xs`. Define length by counting one
-- for the head and recurring on the tail. Define reverse by reversing
-- the tail and placing the old head at its end.
-- The three target laws below are the ones suggested in the chapter.
def length {α : Type} (xs : List α) : Nat :=
  sorry

def reverse {α : Type} (xs : List α) : List α :=
  sorry

-- `xs ++ ys` joins lists. Induct on xs: the empty case is immediate;
-- the cons case uses the induction hypothesis for its tail.
example {α : Type} (xs ys : List α) : length (xs ++ ys) = length xs + length ys := by sorry

-- Reversing changes order, not the number of elements. An induction
-- on xs may need the previous append-length law in the cons case.
example {α : Type} (xs : List α) : length (reverse xs) = length xs := by sorry

-- Reversing twice restores the original order. The induction step
-- often needs a helper fact about reversing an appended singleton.
-- State and prove that helper as a separate theorem if Lean gets stuck.
example {α : Type} (xs : List α) : reverse (reverse xs) = xs := by sorry

end Chapter07Lists
