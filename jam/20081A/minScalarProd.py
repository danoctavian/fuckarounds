caseCount = int(raw_input())
def readVec(strVec):
  return map(lambda x: int(x), strVec.split(" "))

for i in range(0, caseCount):
  size = int(raw_input())
  vec1 = readVec(raw_input())
  vec2 = readVec(raw_input())
  print "Case #" + str(i + 1) + ": " + str(sum(map(lambda (x, y):x * y, zip(sorted(vec1), sorted(vec2, reverse=True)))))
