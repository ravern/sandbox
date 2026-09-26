import Std

-- Beyond TPIL: does rearranging a data pipeline change its answer?
-- Adapted to Lean from Software Foundations, Poly: map_rev and fold_map.
-- Source: https://softwarefoundations.cis.upenn.edu/lf-current/Poly.html
-- The code is supplied; the puzzles are its laws. This is a gentle restart
-- after E13. Tactics and term proofs are both welcome from here onward.
-- These files are independent: no earlier unfinished proof is imported.

namespace Puzzles14

def map (f : α → β) : List α → List β
  | [] => []
  | x :: xs => f x :: map f xs

def reverse : List α → List α
  | [] => []
  | x :: xs => reverse xs ++ [x]

-- 1. Mapping an identity function should leave every element unchanged.
-- Induct on xs. In the cons case, `simp [map, ih]` can unfold one layer
-- and reuse the answer for the tail. Here ih is your induction hypothesis.
theorem map_id (xs : List α) : map (fun x => x) xs = xs := by sorry

-- 2. Pipeline fusion: two passes can become a single pass. The order is
-- important: the right side applies f first and g second. Induct on xs.
theorem map_fusion (f : α → β) (g : β → γ) (xs : List α) :
    map g (map f xs) = map (fun x => g (f x)) xs := by sorry

-- 3. A helper for the next puzzle: transform a joined list either all at
-- once or a piece at a time. Induct on xs, because append recurses on xs.
theorem map_append (f : α → β) (xs ys : List α) : map f (xs ++ ys) = map f xs ++ map f ys := by
  sorry

-- 4. The source's map_rev exercise. Reversing order and changing values
-- commute. Induct on xs; reverse introduces an append, so use map_append.
-- A direct call to a library map/reverse theorem misses the intended lesson:
-- these are our small definitions, and you have just built their helper law.
theorem map_reverse (f : α → β) (xs : List α) : map f (reverse xs) = reverse (map f xs) := by sorry

-- 5. List.foldr replaces [] with an initial value and (::) with a function.
-- The function below rebuilds the list with transformed heads. Prove that
-- this generic traversal does exactly what the recursive map above does.
def foldMap (f : α → β) (xs : List α) : List β :=
  xs.foldr (fun x acc => f x :: acc) []

theorem foldMap_correct (f : α → β) (xs : List α) : foldMap f xs = map f xs := by sorry

end Puzzles14
