Parse  ← ⊏ ⊗ "^v><" ⊙[0_1 0_¯1 1_0 ¯1_0]
Walk   ← \+ ⊂ 0_0
Visits ← ⧻ ◴

A ← Visits Walk Parse

˙⍤ =2 A ">"
˙⍤ =4 A "^>v<"
˙⍤ =2 A "^v^v^v^v^v"

Split ← ⊕∘ ◿ 2 °⊏

B ← Visits /⊂ ≡Walk Split Parse

˙⍤ =3 B "^v"
˙⍤ =3 B "^>v<"
˙⍤ =11 B "^v^v^v^v^v"

⊃A B &fras "input/day03.txt"
