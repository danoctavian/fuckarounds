import sys
users = int(raw_input())
desired = map(lambda x: int(x), raw_input().split())

desired = sorted(zip(desired, range(0, len(desired))))

#print desired
final = [0] * len(desired)
rating = desired[0][0]
for des in desired:
  if rating < des[0]:
    rating = des[0]
  final[des[1]] = rating
  rating += 1

for fin in final:
  sys.stdout.write(str(fin) + " ")
