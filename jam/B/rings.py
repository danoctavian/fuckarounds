cases = int(raw_input())


def circles(r, t):
  count = 0
  r = r + 1
  while True:
    circleA = 2 * r - 1
    if circleA > t:
      return count
    t -= circleA 
    count += 1
    r += 2
  

for i in range(cases):
  [r, t] = raw_input().split(" ")  
  print "Case #" + str(i + 1) + ": " + str(circles(int(r), int(t)))

