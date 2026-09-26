-- Concrete exercises embedded in chapter 7, outside the final exercise list.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Inductive-Types/

namespace Chapter07Bool

-- The introduction rules for this type are its two constructors.
-- To eliminate a Boolean, handle both constructor cases.
inductive MyBool where
  | false : MyBool
  | true : MyBool

-- Define Boolean and, or, and not with pattern matching.
def and : MyBool → MyBool → MyBool := sorry
def or : MyBool → MyBool → MyBool := sorry
def not : MyBool → MyBool := sorry

-- Suggested identity: double negation returns the original Boolean.
example (b : MyBool) : not (not b) = b := by
  sorry

-- Suggested identity: and is commutative.
example (a b : MyBool) : and a b = and b a := by
  sorry

end Chapter07Bool

namespace Chapter07PartialFunctions

-- Compose partial functions, represented by functions returning Option.
-- If f a is none, the composition must return none.
def compose (f : α → Option β) (g : β → Option γ) : α → Option γ := sorry

-- Check that an everywhere-defined identity function changes nothing.
example (f : α → Option β) :
    compose (fun a => some a) f = f := by
  sorry

end Chapter07PartialFunctions

namespace Chapter07CustomList

-- Prove right identity and associativity for this custom append.
inductive List (α : Type) where
  | nil : List α
  | cons : α → List α → List α

def append (as bs : List α) : List α :=
  match as with
  | .nil => bs
  | .cons a rest => .cons a (append rest bs)

theorem append_nil (as : List α) : append as .nil = as := by
  sorry

theorem append_assoc (as bs cs : List α) :
    append (append as bs) cs = append as (append bs cs) := by
  sorry

end Chapter07CustomList

namespace Chapter07Equality

-- The chapter proves symmetry by matching on equality. Complete the
-- other two standard properties the same way.
theorem trans {α : Type} {a b c : α} (h₁ : a = b) (h₂ : b = c) : a = c := by
  sorry

theorem congr {α β : Type} {a b : α} (f : α → β) (h : a = b) : f a = f b := by
  sorry

end Chapter07Equality

namespace Chapter07Inhabited

-- Show these types have a default value. Try constructing the values
-- directly before reaching for typeclass search.
example : Inhabited Bool := by
  sorry

example : Inhabited Nat := by
  sorry

example [Inhabited α] [Inhabited β] : Inhabited (α × β) := by
  sorry

example [Inhabited β] : Inhabited (α → β) := by
  sorry

end Chapter07Inhabited
