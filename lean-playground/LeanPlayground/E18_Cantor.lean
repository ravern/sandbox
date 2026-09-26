import Std

-- Can any catalogue list every possible collection of its own entries?
-- Adapted from Mathematics in Lean, Sets and Functions, culminating in Cantor.
-- Source: https://leanprover-community.github.io/mathematics_in_lean/C04_Sets_and_Functions.html
-- This version writes sets as predicates α → Prop, and supplies the two
-- function properties directly, so no Mathlib installation is needed.

namespace Puzzles18

def Injective (f : α → β) : Prop :=
  ∀ x y, f x = f y → x = y

def Surjective (f : α → β) : Prop :=
  ∀ y, ∃ x, f x = y

-- 1. If g can undo f, f cannot collapse two distinct inputs.
-- Given f x = f y, apply g to both sides (`congrArg g`) and use undo.
-- This is a warmup adapted from the source's inverse-function discussion.
theorem undo_implies_injective (f : α → β) (g : β → α) (undo : ∀ x, g (f x) = x) : Injective f := by
  sorry

-- 2. Two functions that lose no information still lose none in sequence.
-- Unfold Injective; undo the equality with hg first, then hf.
theorem injective_composition (f : α → β) (g : β → γ) (hf : Injective f) (hg : Injective g) :
    Injective (fun x => g (f x)) := by sorry

-- 3. If each stage can reach every output, so can the whole pipeline.
-- Start with a desired γ, obtain its β witness from hg, then obtain
-- the corresponding α witness from hf. Build the final existential.
theorem surjective_composition (f : α → β) (g : β → γ) (hf : Surjective f) (hg : Surjective g) :
    Surjective (fun x => g (f x)) := by sorry

-- 4. Cantor's diagonal argument. Think of f i as catalogue entry i,
-- describing a collection of α values. Define `S := fun i => ¬ f i i`:
-- the collection of entries that do not contain their own index.
-- If f lists every collection, some j satisfies f j = S. At index j,
-- membership would be equivalent to its own negation (compare E02/E04).
-- Try `congrArg (fun s => s j)` on the equality. Classical logic is allowed,
-- but the contradiction can also be derived constructively.
theorem cantor (f : α → (α → Prop)) : ¬Surjective f := by sorry

end Puzzles18
