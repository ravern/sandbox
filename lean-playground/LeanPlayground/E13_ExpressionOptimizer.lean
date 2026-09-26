-- Chapter 8, exercise 5: build and verify a constant-folding optimizer.
-- This is the last and most involved exercise in the book's exercise lists.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Induction-and-Recursion/#exercises

namespace Chapter08Expressions

inductive Expr where
  | const : Nat → Expr
  | var : Nat → Expr
  | plus : Expr → Expr → Expr
  | times : Expr → Expr → Expr
  deriving Repr

open Expr

def sampleExpr : Expr :=
  plus (times (var 0) (const 7)) (times (const 2) (var 1))

-- Part A: evaluate expressions under an assignment of variable values.
def eval (v : Nat → Nat) : Expr → Nat
  | const n => sorry
  | var n => v n
  | plus e₁ e₂ => sorry
  | times e₁ e₂ => sorry

def sampleVal : Nat → Nat
  | 0 => 5
  | 1 => 6
  | _ => 0

-- After completing eval, this should print 47:
-- #eval eval sampleVal sampleExpr

-- Part B: simplify one node when both children are constants.
def simpConst : Expr → Expr
  | plus (const n₁) (const n₂) => const (n₁ + n₂)
  | times (const n₁) (const n₂) => const (n₁ * n₂)
  | e => e

-- Recurse into children first, then use simpConst on the rebuilt node.
def fuse : Expr → Expr := sorry

-- Part C: prove the one-step simplifier preserves every interpretation.
theorem simpConst_eq (v : Nat → Nat) :
    ∀ e : Expr, eval v (simpConst e) = eval v e := by
  sorry

-- Part D: prove the full optimizer preserves every interpretation.
-- Induct on e and use simpConst_eq in the plus and times cases.
theorem fuse_eq (v : Nat → Nat) :
    ∀ e : Expr, eval v (fuse e) = eval v e := by
  sorry

end Chapter08Expressions
