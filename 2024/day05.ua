Text ← &fras "input/05.txt"

P ← ⊜□ ⊸(¬ ⦷ "\n\n") Text
Q ← ⍚(⊜□ ⊸≠@\n) P

Rules ← ≡(⊜⋕ ⊸≠@| °□) °□ ⊢ Q
Pages ← ⍚(⊜⋕ ⊸≠@,) °□ ⊣ Q

Incorrect ← ↧1/+≡(∊Rules⇌) ◇⧅<2

M ← ¬ ≡Incorrect Pages

PickMid ← ⊡ ⌊÷2⧻ .

A ← /+ ≡(PickMid °□) ▽ M Pages
&p A

# Part two
# --------

# Per list of pages
# 1. Find all the relevant order constraints
# 2. Given the constraints find a topological order
#   a. Construct an adjacency matrix
#   b. Square it ceil(log2(N)) times using max plus matrix multiplication

# A list of all relevant page groups for this part
Pages ← ▽ ≡Incorrect . Pages

# Get order constraints for a group of pages
Constraints ← ▽ ⊸≡(∊Rules) ⊂ ≡⇌ . ⧅<2

Adjs ← ⊃(
| ∘
| ˜(↯⧻) 1
) ≡⊗ ⊃(Constraints|¤)

Diag ← ⊃(
| ≡(⊂.)⇡
| ˜↯ 0
) ⧻

Idxs ← ˜∩⊂⊃(Adjs|Diag)

MaxPlus ← ⍜⍉⊞(/↥+)

Check ← (
  ↧1/+ ∊ ◴♭ Constraints .
  ⍤ "Unconstrained value found"
)

OrderedMid ← (
  .
  ⬚¯∞ ⌝⊡ Idxs
  ⍥(MaxPlus.) ⊸(⌈ₙ2 ⧻)
  ⊏⍖ ≡/↥
  PickMid
)

B ← /+ ≡◇OrderedMid Pages
&p B
