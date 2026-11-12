Text ← &fras "input/day02.txt"
Dims ← ⊜(⊜⋕ ⊸≠ @x) ⊸≠ @\n Text

Paper  ← + ⊃/↧(×2 /+) ≡/× ⧅> 2
Ribbon ← + ⊃(/+ ×2 ↙2 ⍆)/×

⊃(/+ ≡Paper)(/+ ≡Ribbon) Dims
