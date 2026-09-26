-- Chapter 4, exercise 4: write statements as Lean propositions.
-- There is no proof to find here. Replace each `sorry` with a definition.
-- Source: https://lean-lang.org/theorem_proving_in_lean4/Quantifiers-and-Equality/#exercises

namespace Chapter04Specifications

-- 1. n is even when some k satisfies n = 2 * k.
def even (n : Nat) : Prop := sorry

-- 2. A prime is greater than 1 and has no nontrivial divisor.
def prime (n : Nat) : Prop := sorry

-- 3. For every bound, there is a larger prime.
def infinitely_many_primes : Prop := sorry

-- 4. A Fermat prime is a prime of the form 2^(2^k) + 1.
def Fermat_prime (n : Nat) : Prop := sorry

-- 5. State that Fermat primes exceed every bound.
def infinitely_many_Fermat_primes : Prop := sorry

-- 6. Every even integer greater than 2 is a sum of two primes.
def goldbach_conjecture : Prop := sorry

-- 7. Every odd integer greater than 5 is a sum of three primes.
def Goldbach's_weak_conjecture : Prop := sorry

-- 8. For n > 2, positive a, b, c never satisfy a^n + b^n = c^n.
def Fermat's_last_theorem : Prop := sorry

end Chapter04Specifications
