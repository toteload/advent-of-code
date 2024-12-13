from helpers import *

txt = day_input(2)

rounds = [line.strip().split(' ') for line in txt.split('\n') if line != '']

def outcome_score(p0, p1):
    # This indexing approaching is waaaaayyy easier to understand than some if/else branches.
    res = [3,6,0]
    idx = ((ord(p1) - ord('X')) - (ord(p0) - ord('A'))) % 3
    return res[idx]

def shape_score_xyz(x):
    return 1 + ord(x) - ord('X')

print(sum([shape_score_xyz(p1) + outcome_score(p0, p1) for [p0, p1] in rounds]))

def shape_score_abc(x):
    return 1 + ord(x) - ord('A')

def shape_select(p0, outcome):
    res = ['C', 'A', 'B']
    idx = ((ord(p0) - ord('A')) + (ord(outcome) - ord('X'))) % 3
    return res[idx]

def outcome_score(x):
    return [0, 3, 6][ord(x) - ord('X')]

print(sum([shape_score_abc(shape_select(p0, outcome)) + outcome_score(outcome) for [p0, outcome] in rounds]))

