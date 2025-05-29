Text ← &fras "test/05.txt"

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
#   b. Square it N times using max plus matrix multiplication

# A list of all relevant page groups for this part
Pages ← ▽ ≡Incorrect . Pages

# Get order constraints for a group of pages
Orders ← ▽ ≡(∊Rules) . ≡⇌◇⧅<2

P ← ≡⊗ ⊃(Orders|¤)

Adjs ← ⊃(
| ∘
| ˜(↯⧻) 1
) P

Diag ← ⊃(
| ≡(⊂.)⇡ # Create diagonal indices
| ˜↯ 0   # List of zeroes to fill with
) ⧻

Idxs ← ˜∩⊂⊃(Adjs|Diag)

MaxPlus ← ⍜⍉⊞(/↥+)

# This is still wrong. I misread a part...
# After taking **only the incorrectly-ordered updates** and ordering them correctly ...
# I also possibly shuffle other elements...

Ordered ← (
  ⬚¯∞ ⌝⊡ Idxs   # Create the matrix
  ⍥(MaxPlus.)10 # The square amount is hard coded to be high enough. not pretty 
  ≡/↥           # Find the topological index for each page
  ⊏⍖            # Sort the pages by topological index
  PickMid
) .

B ← /+ ≡(◇Ordered) Pages
&p B
