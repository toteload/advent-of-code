Actions ← {"toggle" "turn on" "turn off"}
P       ← $"(_|_|_) (\\d+),(\\d+) through (\\d+),(\\d+)" °{⊙⊙∘} Actions
Coords  ← ∩⍉ ⊃(+1-|∘) ⊃(↙2|↘2)
Parse   ← ⊙(Coords ⋕) ⊗Actions °⊂ ⍉ ≡↘1 regex P

Run! ← /+ ♭ ∧⍜(⊙(↙⊙↘)) ^0 ⊙⊙⊙(↯ 1000_1000 0)

A ← Run!⨬(¬|↥1|↧0)
B ← Run!⨬(+2|+1|↥0-1)

˙⍤ =1000000 A Parse "turn on 0,0 through 999,999"
˙⍤ =1000 A Parse "toggle 0,0 through 999,0"

⊃(A|B) Parse &fras "input/day06.txt"
