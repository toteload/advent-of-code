# Read the input
Text ← &fras "input/day01.txt"

# Split input into lines
Lines ← ⊜∘ ⊸≠@\n Text

# Split each line by the white space and parse the numbers
Pairs  ← ≡⊜⋕ ¬⊸⦷ "   " Lines
Sorted ← ⍉≡⍆⍉ Pairs

# Answer for part one
A ← /+⌵≡/- Sorted
A

X ← ⊢ ⍉ Pairs
Y ← ⊣ ⍉ Pairs

Similarity ← /+ ˜⌕ Y

# Answer for part two
B ← /+≡(×⟜Similarity) X
B
