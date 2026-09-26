-- Chapter 7, exercises 3–4: build interpreters for your own syntax trees.
-- An `inductive` declaration lists every possible shape of a value.
-- To consume one, pattern-match on its constructor. Recursive cases
-- receive smaller child trees, which can be passed to the same function.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Inductive-Types/#exercises

namespace Chapter07Terms

-- 3. `const 7` represents the number 7; `var 0` represents a named
-- variable; `plus a b` and `times a b` combine smaller expressions.
-- The assignment `values : Nat → Nat` gives each variable its value:
-- `values n` is what `var n` should evaluate to.
inductive Term where
  | const : Nat → Term
  | var : Nat → Term
  | plus : Term → Term → Term
  | times : Term → Term → Term
  deriving Repr

def eval (values : Nat → Nat) : Term → Nat
  | .const n => n
  | .var n => values n
  -- For plus, evaluate *both* child terms under the same assignment
  -- before adding their resulting numbers.
  | .plus a b => sorry
  -- Times has the same recursive shape, using multiplication.
  | .times a b => sorry

-- Once eval is complete, try `#eval` on a concrete expression. For
-- example, a variable assigned 5 plus a constant 7 should evaluate to 12.
end Chapter07Terms

namespace Chapter07Formulas

-- 4. A Formula is syntax, not yet a truth value. A `var n` node names
-- a Boolean variable. `truth`, `falsity`, `and`, `or`, and `not` build
-- larger formula trees. This is one possible grammar; you may extend it.
inductive Formula where
  | var : Nat → Formula
  | truth : Formula
  | falsity : Formula
  | and : Formula → Formula → Formula
  | or : Formula → Formula → Formula
  | not : Formula → Formula
  deriving Repr

-- `eval` interprets the syntax. Ask `values n` for a variable's Bool;
-- for `and a b`, recursively evaluate a and b, then combine those Bools.
-- Handle all six constructors with a match.
def eval (values : Nat → Bool) : Formula → Bool :=
  sorry

-- Choose one complexity measure and apply it consistently. A simple
-- choice counts each constructor as one node; a binary node then has
-- complexity `1 + complexity left + complexity right`.
def complexity : Formula → Nat :=
  sorry

-- `substitute` must keep unrelated variables and constants unchanged.
-- At `var n`, compare n with `name`; at a compound formula, recurse
-- through its children and rebuild the same constructor.
def substitute (name : Nat) (replacement : Formula) : Formula → Formula :=
  sorry

end Chapter07Formulas
