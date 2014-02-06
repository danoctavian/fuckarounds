cases = int(raw_input())

def makeLetterMap(text):
  text = filter(lambda x: x!= ' ', list(text))
  mp = {}
  for c in text:
    if not c in mp:  
      mp[c] = 1
    else:
      mp[c] = mp[c] + 1
  return mp

for i in range(cases):
  header = raw_input()
  text = raw_input()
  headerMp = makeLetterMap(header)
  textMp = makeLetterMap(text)
  s = "possible"
  for c in textMp:
    count = textMp[c]
    if (not c in headerMp) or headerMp[c] < count:
      s = "impossible"
      break
  print "Case #" + str(i + 1) + ": " + s 



