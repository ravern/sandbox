-- Chapter 4, exercise 4: write statements as Lean propositions.
-- There is no proof to find here. Replace each `sorry` with a *statement*.
-- A `def ... : Prop` describes a claim; it does not assert the claim is true.
-- Use `∃` for "there is", `∀` for "every", `∧` to combine conditions,
-- and `→` for "if ... then ...". `^` is exponentiation and `≠` means
-- unequal. Later definitions may refer to earlier ones such as `prime`.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Quantifiers-and-Equality/#exercises

namespace Chapter04Specifications

-- 1. "n is even" means there exists a natural number k with n = 2*k.
-- The witness k is allowed to depend on n. Try starting with `∃ k : Nat,`.
def even (n : Nat) : Prop :=
  sorry

-- 2. Exclude 0 and 1 explicitly, then describe allowed divisors.
-- `d ∣ n` means d divides n. Any divisor of a prime must be 1 or n.
def prime (n : Nat) : Prop :=
  sorry

-- 3. "Infinitely many" can be stated without a special infinity type:
-- for each bound b, find a prime n strictly greater than b.
-- The quantifier order is `∀ b, ∃ n, ...`; reversing it changes the claim.
def infinitely_many_primes : Prop :=
  sorry

-- 4. First require `prime n`; then say that *some* k makes n equal
-- to `2 ^ (2 ^ k) + 1`. This definition may reuse the one above.
def Fermat_prime (n : Nat) : Prop :=
  sorry

-- 5. Repeat the pattern of exercise 3, replacing `prime` with
-- `Fermat_prime`. You are defining a conjecture, not proving it.
def infinitely_many_Fermat_primes : Prop :=
  sorry

-- 6. Start with `∀ n : Nat, ...`. The precondition is that n is even
-- and n > 2; the conclusion gives two witnesses a and b that are prime
-- and add to n. Use `→` to separate the precondition from the conclusion.
def goldbach_conjecture : Prop :=
  sorry

-- 7. On Nat, "odd" can be expressed as `¬ even n`. As above, require
-- the input condition first, then introduce three prime witnesses
-- whose sum is n.
def Goldbach's_weak_conjecture : Prop :=
  sorry

-- 8. Quantify over n, a, b, c. The hypotheses require n > 2 and
-- a, b, c > 0; the conclusion can use `a ^ n + b ^ n ≠ c ^ n`.
-- Positivity matters because Nat also contains 0.
def Fermat's_last_theorem : Prop :=
  sorry

end Chapter04Specifications
