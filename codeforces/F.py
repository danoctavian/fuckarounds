nodeDepths = [0, 1, 1, 1] + ([-1] * 1001000)
top2 = [1, 2]

ops = int(raw_input())
n = 4
for x in range(0, ops):
  op = int(raw_input())
  op = op - 1      
  nodeDepths[n] = nodeDepths[n + 1] = nodeDepths[op] + 1
  if nodeDepths[n] > nodeDepths[top2[0]]:
    top2[0] = n
  elif nodeDepths[n] > nodeDepths[top2[1]]:
    top2[1] = n
  print nodeDepths[top2[0]] + nodeDepths[top2[1]]
  n += 2

   
