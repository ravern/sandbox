import Std

-- Can you construct a proof that a word reads the same both ways?
-- Adapted from Software Foundations, IndProp: palindromes and palindrome_converse.
-- Source: https://softwarefoundations.cis.upenn.edu/lf-current/IndProp.html
-- The source asks you to invent the relation; its constructors are supplied
-- here so you can concentrate on proving facts about the evidence.

namespace Puzzles15

-- A palindrome is empty, a singleton, or a smaller palindrome surrounded
-- by the same symbol. A proof is a history of using these three rules.
inductive Pal : List α → Prop where
  | empty : Pal []
  | single (x : α) : Pal [x]
  | wrap (x : α) {xs : List α} : Pal xs → Pal (x :: (xs ++ [x]))

-- 1. Build evidence from the inside out. In this example the middle is
-- empty; wrapping it twice produces [1, 2, 2, 1]. Use Pal's constructors.
example : Pal ([1, 2, 2, 1] : List Nat) := by sorry

-- 2. Every list followed by its reversal is a palindrome.
-- Induct on xs, then use Pal.wrap. List.reverse_cons and List.append_assoc
-- help line up the syntax with the constructor's expected list.
theorem mirrored_word (xs : List α) : Pal (xs ++ xs.reverse) := by sorry

-- 3. Induct on the proof h, rather than on xs: the proof's wrap case
-- gives precisely the smaller palindrome you need. This connects the
-- inductive definition to the familiar "equal to its reversal" definition.
theorem reads_backwards {xs : List α} (h : Pal xs) : xs.reverse = xs := by sorry

-- 4. Challenge: prove the converse. Ordinary tail induction is awkward,
-- because removing only the first symbol need not leave a palindrome.
-- Try induction on length, removing both ends of a nonempty list. You
-- may add helper lemmas or explore List.reverseRecOn. This is a step up.
theorem from_reverse_equality (xs : List α) (h : xs.reverse = xs) : Pal xs := by sorry

end Puzzles15
