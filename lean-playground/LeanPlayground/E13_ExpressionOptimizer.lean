-- Chapter 8, exercise 5: build and verify a constant-folding optimizer.
-- This is a small verified compiler pass. First define the meaning of
-- expressions (`eval`), then transform their syntax (`fuse`), then prove
-- that evaluating before and after the transformation gives the same Nat.
-- Finish each part in order: later proofs rely on earlier definitions.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Induction-and-Recursion/#exercises

namespace Chapter08Expressions

inductive Expr where
  | const : Nat → Expr
  | var : Nat → Expr
  | plus : Expr → Expr → Expr
  | times : Expr → Expr → Expr
  deriving Repr

open Expr

-- This represents `(v 0 * 7) + (2 * v 1)`; `var` nodes are looked up
-- through a separate assignment function rather than holding values.
def sampleExpr : Expr :=
  plus (times (var 0) (const 7)) (times (const 2) (var 1))

-- Part A. `eval v e` is the numerical meaning of expression e when
-- variable n has value `v n`. The variable case is already handled;
-- a constant should evaluate to its stored number. In the plus/times
-- cases, recursively evaluate both child expressions
-- with the *same* assignment v, then combine the resulting numbers.
def eval (v : Nat → Nat) : Expr → Nat
  | const n => sorry
  | var n => v n
  | plus e₁ e₂ => sorry
  | times e₁ e₂ => sorry

def sampleVal : Nat → Nat
  | 0 => 5
  | 1 => 6
  | _ => 0

-- Here v 0 = 5 and v 1 = 6, so sampleExpr means 5*7 + 2*6 = 47.
-- After completing eval, uncomment this check:
-- #eval eval sampleVal sampleExpr

-- Part B. `simpConst` simplifies only the *current* node. For example,
-- `plus (const 5) (const 7)` becomes `const 12`; a plus whose children
-- are not both constants stays unchanged. It does not search deeper.
def simpConst : Expr → Expr
  | plus (const n₁) (const n₂) => const (n₁ + n₂)
  | times (const n₁) (const n₂) => const (n₁ * n₂)
  | e => e

-- `fuse` must search the whole tree. In a plus or times case, first
-- call fuse recursively on both children, rebuild the same node, then
-- pass it through simpConst. Constants and variables stay unchanged.
def fuse : Expr → Expr :=
  sorry

-- Part C. Prove that a single simpConst step cannot change the value.
-- Examine e by constructor. The interesting cases are plus/times with
-- two constant children; arithmetic computation should close those.
-- Other cases leave e unchanged, so their values agree immediately.
theorem simpConst_eq (v : Nat → Nat) : ∀ e : Expr, eval v (simpConst e) = eval v e := by sorry

-- Part D. Prove correctness of the full recursive transformation.
-- Induct on e. In a plus/times case, the induction hypotheses say the
-- optimized children keep their values; `simpConst_eq` handles the
-- final one-node simplification after the children are rebuilt.
theorem fuse_eq (v : Nat → Nat) : ∀ e : Expr, eval v (fuse e) = eval v e := by sorry

end Chapter08Expressions
