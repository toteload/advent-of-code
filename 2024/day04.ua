Text ← &fras "input/04.txt"
Grid ← ⊜∘ ⊸≠@\n Text

M    ← ≡(⬚2 ˜↻ +1⊚) ⟜⇡
Diag ← ⊜□ ⍉+⊃(M|×2⇡) ⊙≡↻ ⊙⇡.⊢△.

D ← /+♭ ≡₀(/+◇⌕"XMAS") [∩⟜≡⇌ ∩Diag ⟜≡⇌ Grid]
E ← /+♭ ⌕"XMAS" [∩⟜≡⇌ ⟜⍉ Grid]

A ← + D E
A

Corners ← ⊡ [0_0 0_2 2_2 2_0]
F       ← ≡₁(/+∊ ⍥(⊸↻1)4 "MMSS") ⧈Corners 3_3 Grid
G       ← ≡₀(=@A) ⧈(⊡ 1_1) 3_3 Grid

B ← /+♭ × F G
B
