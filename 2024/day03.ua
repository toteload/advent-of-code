Text ← &fras "input/03.txt"
Muls ← regex "mul\\((\\d{1,3}),(\\d{1,3})\\)" Text

A ← /+ ≡(/×) ⋕ ⍜⍉(↘1) Muls
A

Ops ← ⍜⍉ (↘1) ⬚"" regex "mul\\((\\d{1,3}),(\\d{1,3})\\)|(do)\\(\\)|(don't)\\(\\)" Text

# Take an array and return a mask.
# The mask is 1 for all positions where the length of the element is not 0.
Presence ← ≡(≠0 ⧻°□)

Dos   ← Presence ⍜⍉ (⊡2) Ops
Donts ← Presence ⍜⍉ (⊡3) Ops

Off ← ¯Donts
On  ← ⍜⊡◌ 0 Dos 1

Clamp ← ↥⊙↧ 0 1

Active ← \(Clamp+) + Off On

M ← × Active ¬+ Dos Donts

B ← /+ ≡(/×) ⋕ ▽ M ⍜⍉ (↙ 2) Ops
B
