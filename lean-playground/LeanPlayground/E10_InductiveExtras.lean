-- Concrete exercises embedded in chapter 7, outside the final exercise list.
-- These are separate small studies of how constructors define data and
-- how pattern matching lets you inspect every possible constructor.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Inductive-Types/

namespace Chapter07Bool

-- `MyBool.false` and `MyBool.true` are the only ways to make a MyBool.
-- So a function taking MyBool can be defined by one branch for each.
-- This is the "introduction/elimination" idea from the chapter:
-- constructors introduce values; matching eliminates them into a result.
inductive MyBool where
  | false : MyBool
  | true : MyBool

-- Define truth-table behavior with pattern matching. For example,
-- `and false b` must be false, while `and true b` must be b.
-- `or` chooses true if either argument is true; `not` flips the value.
def and : MyBool → MyBool → MyBool :=
  sorry

def or : MyBool → MyBool → MyBool :=
  sorry

def not : MyBool → MyBool :=
  sorry

-- Suggested identity: double negation returns the original Boolean.
-- Prove it by splitting b into false and true; each case should reduce
-- after the definitions of `not` are filled in.
example (b : MyBool) : not (not b) = b := by sorry

-- Suggested identity: and is commutative. Check all four input pairs
-- by case-splitting a and b; each branch should follow by computation.
example (a b : MyBool) : and a b = and b a := by sorry

end Chapter07Bool

namespace Chapter07PartialFunctions

-- A partial function returns `Option β`: `some b` means success and
-- `none` means no result. For `(compose f g) a`, first compute `f a`.
-- Return none if it is none; if it is `some b`, continue with `g b`.
-- This is how failure propagates through a chain of computations.
def compose (f : α → Option β) (g : β → Option γ) : α → Option γ :=
  sorry

-- `fun a => some a` always succeeds without changing a. Show that
-- composing it before f gives the same function. You may need `funext a`
-- to turn equality of functions into equality of their outputs at each a.
example (f : α → Option β) : compose (fun a => some a) f = f := by sorry

end Chapter07PartialFunctions

namespace Chapter07CustomList

-- This is a fresh List type, separate from Lean's built-in List.
-- Its two constructors are `.nil` and `.cons head tail`. The recursive
-- append follows the first list, keeping each head and appending its tail.
inductive List (α : Type) where
  | nil : List α
  | cons : α → List α → List α

def append (as bs : List α) : List α :=
  match as with
  | .nil => bs
  | .cons a rest => .cons a (append rest bs)

-- Induct on as. The nil case reduces directly; the cons case uses
-- the induction hypothesis for its tail.
theorem append_nil (as : List α) : append as .nil = as := by sorry

-- Associativity uses the same induction: expand append on the first
-- list, then rewrite the recursive remainder with the hypothesis.
theorem append_assoc (as bs cs : List α) : append (append as bs) cs = append as (append bs cs) := by
  sorry

end Chapter07CustomList

namespace Chapter07Equality

-- A proof `h : a = b` lets Lean treat a and b as the same value.
-- Match on h; the only constructor of equality is reflexivity (`rfl`).
-- Complete transitivity and congruence using that one case.
theorem trans {α : Type} {a b c : α} (h₁ : a = b) (h₂ : b = c) : a = c := by sorry

-- Congruence says equal inputs give equal outputs under any function f.
-- Matching on h reduces the goal to `f a = f a`.
theorem congr {α β : Type} {a b : α} (f : α → β) (h : a = b) : f a = f b := by sorry

end Chapter07Equality

namespace Chapter07Inhabited

-- `Inhabited α` is a structure with a `default : α` field.
-- Constructing it means choosing one concrete α value; it does not
-- mean proving that every α value has a property. Try `⟨value⟩`.
example : Inhabited Bool := by sorry

-- Zero is a natural candidate for Nat's default.
example : Inhabited Nat := by sorry

-- If α and β each have a default, pair those two defaults.
example [Inhabited α] [Inhabited β] : Inhabited (α × β) := by sorry

-- A function can ignore its α input and always return β's default.
example [Inhabited β] : Inhabited (α → β) := by sorry

end Chapter07Inhabited
