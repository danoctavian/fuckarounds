import sys
n = int(raw_input())
wallets = map(lambda x: int(x), raw_input().split())
coinsLeft = sum(wallets)
pos = 0

def move(dir):
  sys.stdout.write(dir)

def findClosest(pos, walls):
  if coinsLeft == walls[pos]: return None
  l = pos 
  r = pos
  while l >= 0 or r < len(walls):
    if walls[l] != 0: return l
    if walls[r] != 0: return r
    if l > 0: l -= 1
    if r < len(walls) - 1: r += 1

closest = 0
while coinsLeft > 0:
  if wallets[pos] > 0:  
    move("P")
    wallets[pos] = wallets[pos] - 1
    coinsLeft -= 1
  if coinsLeft == 0: break
  if closest == pos:
    closest = findClosest(pos, wallets) 
    if closest == None or closest == pos:
      closest = filter(lambda x: x >= 0 and x < len(wallets), [pos - 1, pos + 1])[0]
  if closest < pos:
    move("L")
    pos -= 1
  if closest > pos:
    move("R")
    pos += 1

