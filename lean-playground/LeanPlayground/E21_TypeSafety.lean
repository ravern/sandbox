import Std

-- Final project: prove a small programming language cannot go wrong.
-- Adapted from Software Foundations, Programming Language Foundations, Types.
-- Source: https://softwarefoundations.cis.upenn.edu/plf-current/Types.html
-- This smaller Lean language uses Nat literals, addition, Booleans, and if.
-- It keeps the source's progress/preservation argument, but is not a
-- line-for-line translation of its language or all of its exercises.

namespace Puzzles21

inductive Ty where
  | nat
  | bool
  deriving DecidableEq

inductive Term where
  | num : Nat → Term
  | bool : Bool → Term
  | add : Term → Term → Term
  | ite : Term → Term → Term → Term
  deriving Repr

-- Values are finished computations. An add or ite must still do work,
-- even if all of its children are already values.
inductive Value : Term → Prop where
  | num (n : Nat) : Value (.num n)
  | bool (b : Bool) : Value (.bool b)

-- HasType t A is evidence that term t obeys the typing rules for A.
-- Addition needs two numbers; if needs a Boolean condition and branches
-- of the same type. The constructors are the complete typing rules.
inductive HasType : Term → Ty → Prop where
  | num (n : Nat) : HasType (.num n) .nat
  | bool (b : Bool) : HasType (.bool b) .bool
  | add {a b : Term} : HasType a .nat → HasType b .nat → HasType (.add a b) .nat
  |
  ite {c t e : Term} {A : Ty} : HasType c .bool → HasType t A → HasType e A → HasType (.ite c t e) A

-- Step performs one unit of evaluation, left to right. It is a relation,
-- so a Step proof records exactly which evaluation rule was used.
inductive Step : Term → Term → Prop where
  | addLeft {a a' b : Term} : Step a a' → Step (.add a b) (.add a' b)
  | addRight (n : Nat) {b b' : Term} : Step b b' → Step (.add (.num n) b) (.add (.num n) b')
  | addNums (n m : Nat) : Step (.add (.num n) (.num m)) (.num (n + m))
  | iteCond {c c' t e : Term} : Step c c' → Step (.ite c t e) (.ite c' t e)
  | iteTrue (t e : Term) : Step (.ite (.bool true) t e) t
  | iteFalse (t e : Term) : Step (.ite (.bool false) t e) e

-- 1. Warmup: a term has at most one type. Induct on its first typing
-- proof and inspect the second. Constructors with incompatible shapes
-- disappear; in an if, compare the corresponding branch typings.
theorem type_unique {t : Term} {A B : Ty} (hA : HasType t A) (hB : HasType t B) : A = B := by sorry

-- 2. Progress: a typed term is finished or can take a step. Induct on
-- the typing proof. For addition, first ask whether its left input can
-- step; only once it is a numeric value do you inspect the right input.
-- A helper may show that a Value with type nat must be a num constructor.
theorem progress {t : Term} {A : Ty} (h : HasType t A) : Value t ∨ ∃ u, Step t u := by sorry

-- 3. Preservation: an evaluation step cannot change a typed term's type.
-- Induct on the Step proof and inspect h. Evaluation inside a child
-- needs the induction hypothesis; selecting an if branch uses its typing.
theorem preservation {t u : Term} {A : Ty} (h : HasType t A) (hs : Step t u) : HasType u A := by
  sorry

-- Steps is the evidence for zero or more steps: either stop now, or
-- take one Step and follow it with the remaining Steps.
inductive Steps : Term → Term → Prop where
  | refl (t : Term) : Steps t t
  | next {t u v : Term} : Step t u → Steps u v → Steps t v

-- 4. Lift preservation from one step to an arbitrary finite execution.
-- Induct on the Steps evidence, carrying the current typing proof.
theorem preservation_many {t u : Term} {A : Ty} (h : HasType t A) (hs : Steps t u) :
    HasType u A := by sorry

-- 5. Combine 2 and 4. If evaluation of a typed term stops, it must have
-- reached a value rather than an invalid operation. This proves safety;
-- it does not by itself prove that evaluation eventually stops.
theorem cannot_get_stuck {t u : Term} {A : Ty} (h : HasType t A) (hs : Steps t u)
    (stopped : ∀ v, ¬Step u v) : Value u := by sorry

-- 6. Extra counterexample: without the typing hypothesis, safety fails.
-- A Boolean plus a number is neither a value nor able to take a step.
-- Inspect each supposed Value/Step constructor to find the contradiction.
example : ¬Value (.add (.bool true) (.num 1)) ∧ (∀ u, ¬Step (.add (.bool true) (.num 1)) u) := by
  sorry

end Puzzles21
