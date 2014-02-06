import math
cases = int(raw_input())


def neededPaint(r, C):
  return 2 * r * C + C + 2 * (C - 1) * C
  

def quadrSolution(r, t):
  return (-2 *(2 * r - 1) + math.sqrt(pow((2 * r - 1), 2) + 8 * t)) / 4
def circles(r, t):
  C = 1
  needed = neededPaint(r, C)
  return math.floor(quadrSolution(r, t))
  

for i in range(cases):
  [r, t] = raw_input().split(" ")  
  print "Case #" + str(i + 1) + ": " + str(int(circles(int(r), int(t))))

