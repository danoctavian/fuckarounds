#brute force solution - complete crap
import sys

def combinations(n):
  if n == 1:
    yield [True]
    yield [False]
  else:
    for c in combinations(n - 1):
      yield c + [True]
      yield c + [False]

cases = int(raw_input())

def wins(animals, c, d, a):
  (A, i) = a
  i -= 1
  if A == "D":
    i += c
  return animals[i]

for i in range(cases):
  [c, d, v] = map(int, raw_input().split(" "))
  voters = []
  for j in range(v):
    anims = raw_input().split(" ")
    voters.append(map(lambda s: (s[0], int(s[1:])), anims)) 
    best = 0
  for comb in combinations(c + d):
    satisfied = 0
    for voter in voters:
      (good, bad) = voter 
      if wins(comb, c, d, good) and not wins(comb, c, d, bad):
        satisfied += 1
    if satisfied > best: best = satisfied
  print best

    
