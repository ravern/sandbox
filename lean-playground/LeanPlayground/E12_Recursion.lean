-- Chapter 8, exercises 1–4: recursive definitions, induction, and vectors.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Induction-and-Recursion/#exercises

namespace Chapter08Nat

-- 1. Define these with equations or pattern matching. This revisits
-- chapter 7's arithmetic operations, now focusing on the equation compiler.
def add (m n : Nat) : Nat := sorry
def mul (m n : Nat) : Nat := sorry
def pow (m n : Nat) : Nat := sorry

-- Suggested basic properties to derive from your defining equations.
example (n : Nat) : add n 0 = n := by
  sorry

example (n : Nat) : mul n 0 = 0 := by
  sorry

example (n : Nat) : pow n 0 = 1 := by
  sorry

end Chapter08Nat

namespace Chapter08Lists

-- 2. Define list operations with equations and use induction for laws.
def reverse {α : Type} (xs : List α) : List α := sorry

example {α : Type} (xs : List α) : reverse (reverse xs) = xs := by
  sorry

end Chapter08Lists

namespace Chapter08WellFounded

-- 3a. Course-of-values recursion may use results at *any* smaller k.
-- A solution can recurse via Nat.strongRecOn or a well-founded relation.
def courseOfValues
    (step : (n : Nat) → (∀ k : Nat, k < n → Nat) → Nat) : Nat → Nat := sorry

-- 3b. Recreate the interface of WellFounded.fix. This is one of the
-- hardest tasks: justify every recursive call using `r y x` and `wf`.
noncomputable def myWellFoundedFix
    {α : Sort u} (r : α → α → Prop) (wf : WellFounded r)
    (C : α → Sort v)
    (step : (x : α) → ((y : α) → r y x → C y) → C x) :
    (x : α) → C x := sorry

end Chapter08WellFounded

namespace Chapter08Vectors

-- 4. Append length-indexed vectors. The resulting length must be m + n.
-- Try induction on the first vector; a helper may be needed for casts.
def append {α : Type} {m n : Nat} :
    Vector α m → Vector α n → Vector α (m + n) := sorry

end Chapter08Vectors
