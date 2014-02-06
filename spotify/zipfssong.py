[n, m] = map(lambda x: int(x), raw_input().split(" "))

songs = []
for i in range(1, n + 1):
  [listens, name] = raw_input().split(" ")
  songs.append((int(listens), i, name))
fstScore = songs[0][0]

top = sorted(map(lambda (listens, i, name): (listens / (fstScore / i), i, name), songs), reverse=True)[:m]
for s in top:
  print s[2]
  
  
