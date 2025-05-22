Text ← &fras "test/04.txt"
Grid ← ⊜∘ ⊸≠@\n Text
Grid

F ← ⌕"XMAS"
C ← /+♭
R ← ≡⇌
V ← ⍉

C⍜∘ F Grid
C⍜R F Grid

C⍜V F Grid
C⍜(R V) F Grid

≡↻⇡10 Grid
