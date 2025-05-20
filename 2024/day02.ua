Text    ← &fras "input/day02.txt"
Lines   ← ⊜□ ⊸≠@\n Text
Reports ← ⍚(⊜⋕ ¬⊸⌕ " ") Lines

Monotonic ← ≥0 × ⊃(⊢|⊣) ⍆ ⧈(± /-) 2
SmallDiff ← =0 /+ ⧈(+ ⊃(<1|>3) ⌵/-) 2

Safe ← × ⊃(Monotonic|SmallDiff)

A ← /+ ≡(Safe°□) Reports
A

Options   ← ⧅< -1 ⧻ .
Tolerable ← ≠0 /+ ≡Safe Options

B ← /+ ≡(Tolerable°□) Reports
B
