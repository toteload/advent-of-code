Lines ← ⊜∘ ⊸≠ @\n

Vowels    ← ≥3 /+ ∊ "aeiou"
Twice     ← /↥ ⧈=
Forbidden ← ¬ /↥ ⧈(∊ ["ab" "cd" "pq" "xy"]) 2

ANice ← ↧↧ ⊃(Vowels|Twice|Forbidden)
A     ← /+ ≡ANice

˙⍤ =1 ANice "ugknbfddgicrmopn"
˙⍤ =1 ANice "aaa"
˙⍤ =0 ANice "jchzalrnumimnmhp"
˙⍤ =0 ANice "haegwjzuvuyypxyu"
˙⍤ =0 ANice "dvszwmarrgswjxmb"

Double ← /↥ ♭ × ⊃(˙⊞≍|˙⊞(≥ 2 -) ⇡ ⧻) ⧈∘ 2
Eye    ← /↥ ⧈(=⊙◌)

BNice ← ↧ ⊃(Double|Eye)
B     ← /+ ≡BNice

˙⍤ =1 BNice "qjhvhtzxzqqjkmpb"
˙⍤ =1 BNice "xxyxx"
˙⍤ =0 BNice "uurcxstgmygtbstg"
˙⍤ =0 BNice "ieodomkazucvgmuy"

⊃(A|B) Lines &fras "input/day05.txt"
