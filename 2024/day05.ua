Text ← &fras "test/05.txt"

P ← ⊜□ ⊸(¬ ⦷ "\n\n") Text
Q ← ⍚(⊜□ ⊸≠@\n) P

Rules ← ≡(⊜⋕ ⊸≠@| °□) °□ ⊢ Q
Pages ← ⍚(⊜⋕ ⊸≠@,) °□ ⊣ Q

Incorrect ← ↧1/+≡(∊Rules⇌) ◇⧅<2

M ← ¬ ≡Incorrect Pages

# Answer part one
A ← /+ ≡(⊡ ⌊÷2⧻ . °□) ▽ M Pages

# Get order constraints for a list of pages
O ← ▽ ≡(∊Rules) . ≡⇌◇⧅<2

# A list of pages
A ← °□⊡2 ▽ ≡Incorrect . Pages

# Find constraints for a list of pages
P ← ≡⊗ ⊃(O|¤)

L ← ⬚¯∞ ⌝⊡ P A ↯¤⧻P A 1

Z ← (
  ⬚¯∞ ⌝⊡ ⊃(
    ≡(⊂.)⇡  # Create diagonal indices
  | ˜(↯¤) 0 # List of zeroes to fill with
  )
)
Z 5
X ← ⬚¯∞↥ Z 5 L
X

Identity ← ⊞=.⇡
I        ← Identity 5
B        ← ↯5_5¯∞

# ⍜▽(⋅1)
