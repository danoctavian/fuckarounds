cases = int(raw_input())

for i in range(cases):
  n = int(raw_input())
  s = ""
  if n % 2 == 0:
    s = "even"
  else:
    s = "odd"
  print "Case #" + str(i + 1) + ": " + s
