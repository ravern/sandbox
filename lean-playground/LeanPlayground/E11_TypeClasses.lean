-- Chapter 10 has no final exercise list; this exercise appears in the text.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Type-Classes/

section

-- Define default values for List and Sum using local instances.
-- `local` keeps these experiments from changing instance search elsewhere.
local instance {α : Type} : Inhabited (List α) where
  default := sorry

local instance [Inhabited α] {β : Type} : Inhabited (Sum α β) where
  default := sorry

-- Once the instances are complete, uncomment these checks:
-- #eval (default : List Nat)
-- #eval (default : Sum Nat Bool)

end
