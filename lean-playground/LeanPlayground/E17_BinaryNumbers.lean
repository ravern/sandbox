import Std

-- A serialization puzzle: when does a round trip preserve the exact bits?
-- Adapted from the binary-number exercises in Software Foundations,
-- Induction. Definitions are supplied in Lean, including normalization.
-- Source (2024 edition): https://www.cs.princeton.edu/courses/archive/spring24/cos510/sf/lf/Induction.html

namespace Puzzles17

-- Bits are stored least significant first: bit0 b doubles b's value;
-- bit1 b doubles it and adds one. zero terminates the representation.
inductive Bin where
  | zero
  | bit0 : Bin → Bin
  | bit1 : Bin → Bin
  deriving Repr, DecidableEq

def toNat : Bin → Nat
  | .zero => 0
  | .bit0 b => 2 * toNat b
  | .bit1 b => 2 * toNat b + 1

def increment : Bin → Bin
  | .zero => .bit1 .zero
  | .bit0 b => .bit1 b
  | .bit1 b => .bit0 (increment b)

def ofNat : Nat → Bin
  | 0 => .zero
  | n + 1 => increment (ofNat n)

-- 1. Prove that carrying a bit increases the represented number by one.
-- Induct on b. The bit1 branch uses the induction hypothesis for the
-- carry; unfold one layer, rewrite it, and solve the arithmetic.
theorem increment_value (b : Bin) : toNat (increment b) = toNat b + 1 := by sorry

-- 2. Encoding and decoding a Nat returns the original number.
-- Induct on n. The successor case should use increment_value.
theorem nat_roundtrip (n : Nat) : toNat (ofNat n) = n := by sorry

-- 3. Bug hunt: the reverse round trip is NOT always the original Bin.
-- Find a concrete witness with a redundant zero digit. This is a proof
-- of a counterexample, not an intentionally impossible theorem.
example : ∃ b : Bin, ofNat (toNat b) ≠ b := by sorry

-- Normalization removes redundant high-order zero digits. Doubling zero
-- should keep its unique representation; doubling a nonzero value adds a bit.
def canonicalDouble : Bin → Bin
  | .zero => .zero
  | b => .bit0 b

def normalize : Bin → Bin
  | .zero => .zero
  | .bit0 b => canonicalDouble (normalize b)
  | .bit1 b => .bit1 (normalize b)

-- 4. Removing those redundant digits cannot change the numerical value.
-- A helper about toNat (canonicalDouble b) makes the bit0 case simpler.
theorem normalize_value (b : Bin) : toNat (normalize b) = toNat b := by sorry

-- 5. Challenge: repair the failed round-trip statement with normalize.
-- Induct on b. Expect a helper relating ofNat (n+n) to canonicalDouble
-- (ofNat n); prove that helper by induction on n before returning here.
theorem bin_roundtrip (b : Bin) : ofNat (toNat b) = normalize b := by sorry

end Puzzles17
