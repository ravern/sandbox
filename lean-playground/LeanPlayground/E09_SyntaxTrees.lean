-- Chapter 7, exercises 3–4: build interpreters for your own syntax trees.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Inductive-Types/#exercises

namespace Chapter07Terms

-- 3. Extend the four constructors if you want more arithmetic operators.
-- A variable `var n` gets its value from the assignment `values n`.
inductive Term where
  | const : Nat → Term
  | var : Nat → Term
  | plus : Term → Term → Term
  | times : Term → Term → Term
  deriving Repr

def eval (values : Nat → Nat) : Term → Nat
  | .const n => n
  | .var n => values n
  | .plus a b => sorry
  | .times a b => sorry

-- Once eval is complete, try #eval with a concrete assignment and term.
end Chapter07Terms

namespace Chapter07Formulas

-- 4. Define a syntax for propositional formulas. One possible grammar
-- is given here; change it if you want other connectives.
inductive Formula where
  | var : Nat → Formula
  | truth : Formula
  | falsity : Formula
  | and : Formula → Formula → Formula
  | or : Formula → Formula → Formula
  | not : Formula → Formula
  deriving Repr

-- Interpret variables using `values`; recurse through the tree.
def eval (values : Nat → Bool) : Formula → Bool := sorry

-- Count nodes, or choose another explicit complexity measure.
def complexity : Formula → Nat := sorry

-- Replace each occurrence of variable `name` with `replacement`.
def substitute (name : Nat) (replacement : Formula) : Formula → Formula := sorry

end Chapter07Formulas
