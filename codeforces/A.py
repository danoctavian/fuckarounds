nums = raw_input()

[a, b] = map(lambda x: int(x), nums.split())
h = 0
while a >= b:
  h = h + b
  a = a - b
  a += 1  
h = h + a
print h
  
