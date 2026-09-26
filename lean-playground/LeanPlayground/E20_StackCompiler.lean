import Std

-- Compile arithmetic to a stack machine, then prove the compiler correct.
-- Adapted from Software Foundations, Imp: stack_compiler, execute_app,
-- and stack_compiler_correct. We supply the compiler and interpreter;
-- underflow is an explicit Option failure in this version.
-- Source: https://softwarefoundations.cis.upenn.edu/lf-current/Imp.html

namespace Puzzles20

inductive Expr where
  | lit : Nat → Expr
  | var : Nat → Expr
  | add : Expr → Expr → Expr
  | sub : Expr → Expr → Expr
  | mul : Expr → Expr → Expr
  deriving Repr

def eval (env : Nat → Nat) : Expr → Nat
  | .lit n => n
  | .var x => env x
  | .add a b => eval env a + eval env b
  | .sub a b => eval env a - eval env b
  | .mul a b => eval env a * eval env b

inductive Instr where
  | push : Nat → Instr
  | load : Nat → Instr
  | add
  | sub
  | mul
  deriving Repr

-- The head of the list is the top of the stack. After pushing a then b,
-- the stack starts b :: a :: rest. Subtraction must therefore use a - b.
-- On Nat, subtraction is truncated at zero.
def step (env : Nat → Nat) : Instr → List Nat → Option (List Nat)
  | .push n, s => some (n :: s)
  | .load x, s => some (env x :: s)
  | .add, b :: a :: s => some ((a + b) :: s)
  | .sub, b :: a :: s => some ((a - b) :: s)
  | .mul, b :: a :: s => some ((a * b) :: s)
  | _, _ => none

-- Option.bind runs the next computation only after a successful result.
-- A single underflow therefore stops the whole program with none.
def run (env : Nat → Nat) : List Instr → List Nat → Option (List Nat)
  | [], s => some s
  | i :: code, s => (step env i s).bind (run env code)

def compile : Expr → List Instr
  | .lit n => [.push n]
  | .var x => [.load x]
  | .add a b => compile a ++ compile b ++ [.add]
  | .sub a b => compile a ++ compile b ++ [.sub]
  | .mul a b => compile a ++ compile b ++ [.mul]

-- 1. Sanity check: an addition instruction on an empty stack fails.
-- The machine still has a meaning for this invalid program: none.
example : run (fun _ => 0) [.add] [] = none := by sorry

-- 2. Executing concatenated code is the same as executing both pieces
-- in sequence. Induct on p, generalizing s so the hypothesis applies to
-- a changed stack. Inspect step's Option result in the inductive case.
theorem run_append (env : Nat → Nat) (p q : List Instr) (s : List Nat) :
    run env (p ++ q) s = (run env p s).bind (run env q) := by sorry

-- 3. The key invariant: compiled code pushes exactly one correct value
-- above *any* existing stack. Proving only the empty-stack case first
-- makes induction too weak: subexpressions run above earlier results.
-- Induct on e, generalizing s, and use run_append for composed code.
theorem compile_on_any_stack (env : Nat → Nat) (e : Expr) (s : List Nat) :
    run env (compile e) s = some (eval env e :: s) := by sorry

-- 4. Specialize the invariant to []. The `some` also proves that
-- compiled expressions never underflow, for every variable assignment.
theorem compiler_correct (env : Nat → Nat) (e : Expr) :
    run env (compile e) [] = some [eval env e] := by sorry

-- 5. Extra bug hunt: a compiler author reverses the two pushes for
-- subtraction. Give a concrete a,b for which the machine disagrees with
-- the source expression a-b. This is an added counterexample exercise.
example : ∃ a b : Nat, run (fun _ => 0) [.push b, .push a, .sub] [] ≠ some [a - b] := by sorry

end Puzzles20
