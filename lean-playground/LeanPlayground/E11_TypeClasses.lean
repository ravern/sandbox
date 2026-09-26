-- Chapter 10 has no final exercise list; this exercise appears in the text.
-- A typeclass is a structure Lean can search for automatically.
-- `Inhabited α` asks for a default α value. An `instance` supplies
-- a rule for constructing that structure; later `default` uses the
-- rule Lean finds from its type. This file uses local instances so
-- your experiments do not affect the other exercise modules.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Type-Classes/

section

-- An empty list works for any element type α, even if α is empty.
-- Fill the first default field with a `List α` value.
local instance {α : Type} : Inhabited (List α) where
  default := sorry

-- `Sum α β` is either `Sum.inl a` or `Sum.inr b`.
-- This instance assumes α is inhabited, so it can choose the left
-- constructor without requiring any β value.
local instance [Inhabited α] {β : Type} : Inhabited (Sum α β) where
  default := sorry

-- Once the instances are complete, uncomment these checks:
-- #eval (default : List Nat)
-- #eval (default : Sum Nat Bool)

end
