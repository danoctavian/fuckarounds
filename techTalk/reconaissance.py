cases = int(raw_input())

for j in range(cases):
  n = int(raw_input())
  heightsStr = raw_input()
  heights = map(lambda x: int(x), heightsStr.split(" "))
  heights = zip(heights, range(1, n + 1))
  bestDiff = 999999999999999
  s1 = -1
  s2 = -1
  for i in range(n - 1):
    (h1, p1) = heights[i]
    (h2, p2) = heights[i + 1]
    if abs(h2 -h1) < bestDiff:
      bestDiff = abs(h2 - h1)
      s1 = p1
      s2 = p2
      if bestDiff == 83: print s1, s2
  (h1, p1) =heights[0]
  (h2, p2) =heights[n - 1]
  if abs(h1 - h2) < bestDiff:
    s1 = p2
    s2 = p1

  print "Case #" + str(j + 1) + ": " + str(s1) + " " + str(s2)


