-- Chapter 8, exercises 1–4: recursive definitions, induction, and vectors.
-- This chapter asks you to define functions by equations and then prove
-- those equations imply useful laws. A pattern such as `| m, 0 => ...`
-- handles one case; `| m, k + 1 => ...` handles the successor case.
-- Recursive calls must move toward a base case so Lean can see termination.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Induction-and-Recursion/#exercises

namespace Chapter08Nat

-- 1. Revisit arithmetic from E08, this time writing separate defining
-- equations for zero and successor rather than a single `match` body.
-- Choose which argument shrinks on each call, then keep that choice
-- consistent. For exponentiation, the zero exponent returns 1.
def add (m n : Nat) : Nat :=
  sorry

def mul (m n : Nat) : Nat :=
  sorry

def pow (m n : Nat) : Nat :=
  sorry

-- These are suggested laws, not extra axioms. Replace the definitions
-- first, then let Lean reduce the relevant zero branch. If your chosen
-- recursion runs along the other argument, use induction instead.
example (n : Nat) : add n 0 = n := by sorry

-- The result of multiplying by zero should be zero for your definition.
example (n : Nat) : mul n 0 = 0 := by sorry

-- The zero exponent should yield the multiplicative identity, including
-- the input n = 0 under this conventional Nat definition.
example (n : Nat) : pow n 0 = 1 := by sorry

end Chapter08Nat

namespace Chapter08Lists

-- 2. Define reverse with an empty-list equation and a cons equation.
-- A simple version reverses the tail and appends the head as a singleton.
-- To prove double reversal, induction on xs may need a helper lemma
-- describing reverse of an append.
def reverse {α : Type} (xs : List α) : List α :=
  sorry

-- The theorem quantifies over every element type α and every list xs.
-- In the cons case, rewrite with the induction hypothesis for the tail.
example {α : Type} (xs : List α) : reverse (reverse xs) = xs := by sorry

end Chapter08Lists

namespace Chapter08WellFounded

-- 3a. Ordinary recursion on n normally gives you the answer for n-1.
-- Here `step` receives a lookup function `∀ k, k < n → Nat`, so it may
-- ask for the answer at *any* smaller k, with a proof that k < n.
-- Try to build that lookup with strong induction (`Nat.strongRecOn`).
def courseOfValues (step : (n : Nat) → (∀ k : Nat, k < n → Nat) → Nat) : Nat → Nat :=
  sorry

-- 3b. This generalizes 3a beyond Nat and `<`. The relation `r y x`
-- means y is an allowed smaller input than x. `wf` guarantees that
-- repeatedly moving to smaller inputs cannot continue forever.
-- `C x` is the result type at x; `step x recurse` computes that result,
-- where `recurse y proof` is allowed only when `proof : r y x`.
-- This is a research-level exercise in Lean's recursion machinery;
-- read the chapter's well-founded recursion section before starting.
noncomputable def myWellFoundedFix {α : Sort u} (r : α → α → Prop) (wf : WellFounded r)
    (C : α → Sort v) (step : (x : α) → ((y : α) → r y x → C y) → C x) : (x : α) → C x :=
  sorry

end Chapter08WellFounded

namespace Chapter08Vectors

-- 4. `Vector α m` contains exactly m elements, so the output type
-- itself records the claimed length of the append. Recurse on the
-- first vector: empty returns the second; cons adds a head to the
-- recursively appended tail. Lean may require an equality or helper
-- to reconcile how `m + n` reduces in the successor case.
def append {α : Type} {m n : Nat} : Vector α m → Vector α n → Vector α (m + n) :=
  sorry

end Chapter08Vectors
