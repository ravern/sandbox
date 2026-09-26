# LeanPlayground

Open the `LeanPlayground` folder in VS Code, then work through the numbered
`E00`–`E13` files in `LeanPlayground/`. The numbers give a learning order;
each file's first comment names its source chapter in
[Theorem Proving in Lean 4](https://lean-lang.org/theorem_proving_in_lean4/).
`E00_Warmups.lean` starts with the techniques you have already seen, and
`E13_ExpressionOptimizer.lean` is the final challenge. Chapters without a
numbered file have no final exercise list in the book.

Replace each `sorry` with a definition or proof. The Lean InfoView shows the
current goal when your cursor is in a proof. Run `lake build` from this folder
to check every file; warnings about `sorry` are expected until you finish.

The files cover every item in the book's chapter 3, 4, 5, 7, and 8 exercise
lists, including the eleven existential identities that chapter 4 asks you
to revisit. They also include concrete exercises embedded in chapters 3, 7,
and 10. For open-ended exercises, the definitions and laws in the files are
starting points; the comments preserve the broader task.

The exercise statements and examples are adapted from the
[official book source](https://github.com/leanprover/theorem_proving_in_lean4)
(Apache 2.0), revision `4e28129`.
