# LeanPlayground

Open the `LeanPlayground` folder in VS Code, then work through the numbered
`E00`–`E21` files in `LeanPlayground/`. E00–E13 follow exercises from
[Theorem Proving in Lean 4](https://lean-lang.org/theorem_proving_in_lean4/).
E13 is its last end-of-chapter exercise, **not the end of the book**: later
chapters cover structures, typeclasses, conversion tactics, and axioms.
E14–E21 continue with selected exercises adapted from other sources.

Replace each `sorry` with a definition or proof. The Lean InfoView shows the
current goal when your cursor is in a proof. Run `lake build` from this folder
to check every file; warnings about `sorry` are expected until you finish.
E00–E06 follow the book's proof-term approach: write a value directly after
`:=`, using `fun`, constructors, and pattern matching. E07 introduces `by`
and tactic proofs; later files may use either style.

The files cover every item in the book's chapter 3, 4, 5, 7, and 8 exercise
lists, including the eleven existential identities that chapter 4 asks you
to revisit. They also include concrete exercises embedded in chapters 3, 7,
and 10. For open-ended exercises, the definitions and laws in the files are
starting points; the comments preserve the broader task.

The exercise statements and examples are adapted from the
[official book source](https://github.com/leanprover/theorem_proving_in_lean4)
(Apache 2.0), revision `4e28129`.

## Formatting

VS Code formats Lean files on save using [lean-fmt](https://github.com/jcreinhold/lean-fmt)
and the recommended Simple LSP Client extension. After cloning, run
`lake build lean-fmt` once, then reload VS Code. You can also use **Format
Document** (Shift+Option+F on macOS), or `lake exe lean-fmt format` in the
terminal. The formatter is pinned to a revision tested with this project's
Lean version; rebuild it after updating the toolchain.

## Beyond the book

These eight files add 40 proof goals. The definitions are supplied so you
can focus on proving properties. Each file is independent of earlier
unfinished exercises and uses only Lean and its bundled Std library.
Tactics are welcome. E14 starts gently again; later files include harder
final challenges, so you can move on and return to a difficult last goal.

| File | What you prove | Source |
| --- | --- | --- |
| [E14](LeanPlayground/E14_ListTransformations.lean) | Pipeline fusion, map/reverse commuting, fold-based map | [Software Foundations: Poly](https://softwarefoundations.cis.upenn.edu/lf-current/Poly.html) |
| [E15](LeanPlayground/E15_Palindromes.lean) | Constructed palindromes are exactly words equal to their reversal | [Software Foundations: IndProp](https://softwarefoundations.cis.upenn.edu/lf-current/IndProp.html) |
| [E16](LeanPlayground/E16_Subsequences.lean) | Deletions preserve order, bound length, and compose | [Software Foundations: IndProp](https://softwarefoundations.cis.upenn.edu/lf-current/IndProp.html) |
| [E17](LeanPlayground/E17_BinaryNumbers.lean) | Binary round trips, a counterexample, and normalization | [Software Foundations: Induction (2024)](https://www.cs.princeton.edu/courses/archive/spring24/cos510/sf/lf/Induction.html) |
| [E18](LeanPlayground/E18_Cantor.lean) | Function properties and Cantor's diagonal theorem | [Mathematics in Lean: Sets and Functions](https://leanprover-community.github.io/mathematics_in_lean/C04_Sets_and_Functions.html) |
| [E19](LeanPlayground/E19_VerifiedSort.lean) | Insertion sort orders values without losing duplicates | [Software Foundations: Sort](https://softwarefoundations.cis.upenn.edu/vfa-current/Sort.html) |
| [E20](LeanPlayground/E20_StackCompiler.lean) | A stack compiler computes the right answer and never underflows | [Software Foundations: Imp](https://softwarefoundations.cis.upenn.edu/lf-current/Imp.html) |
| [E21](LeanPlayground/E21_TypeSafety.lean) | Progress, preservation, and absence of stuck typed programs | [Software Foundations: Types](https://softwarefoundations.cis.upenn.edu/plf-current/Types.html) |

Software Foundations is written for Rocq. These files are Lean adaptations,
with some supplied implementations and added warmups/counterexamples; they
are not the complete exercise collections from those sources. E18 uses
predicates directly instead of Mathlib's set API. Each file explains its
changes and gives hints without filling in the proofs.
