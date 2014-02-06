import heapq
import itertools as it

def maxScore(heap, deck):
  deck.reverse()
  turns = 1
  score = 0
  while turns > 0 and len(heap) > 0:
    card = heapq.heappop(heap)  
    print card
    (t, c, s) = heappify(card)
    print "score inc" + str(s)
    turns += t
    score += s
    while c > 0 and len(deck) > 0:
      c -= 1
      heapq.heappush(heap, heappify(deck.pop()))
            
  return score

cases = int(raw_input())

def heappify(card):
  (t, c, s) = card
  return (-t, -c, -s)

def toCard(str):
  [c, s, t] = map(lambda x: int(x), str.split(' '))
  return (t, c, s)

for i in range(1, cases + 1):
  heap = []  
  N = int(raw_input())
  for i in range(N): 
    heapq.heappush(heap, heappify(toCard(raw_input())))

  M = int(raw_input())
  deck = []
  for i in range(M): 
    deck.append(toCard(raw_input()))
  answer = str(maxScore(heap, deck))
  print "Case #" + str(i) + ": " +  answer

 
