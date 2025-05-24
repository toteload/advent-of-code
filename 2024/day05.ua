Text ← &fras "input/05.txt"

P ← ⊜□ ⊸(¬ ⦷ "\n\n") Text
Q ← ⍚(⊜□ ⊸≠@\n) P

Rules ← ≡(⊜⋕ ⊸≠@| °□) °□ ⊢ Q
Pages ← ⍚(⊜⋕ ⊸≠@,) °□ ⊣ Q

Incorrect ← ↧1/+≡(∊Rules⇌) ◇⧅<2

M ← ¬ ≡Incorrect Pages
A ← /+ ≡(⊡ ⌊÷2⧻ . °□) ▽ M Pages
A
