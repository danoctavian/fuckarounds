caseCount = int(raw_input())


maxFlavors = 0
over = 0

class Customer:
  def __init__(self):
    self.flavors = {}
  def addFlavor(self, flavor, malted):
    self.flavors[flavor] = malted
  def __repr__(self):
    return str(self.flavors)

def buildCustomer(strC):
  global over
  global maxFlavors
  customer = Customer()
  pieces = strC.split(" ")
  for i in range(1, len(pieces) - 1, 2):
    customer.addFlavor(pieces[i], pieces[i + 1])

  if len(customer.flavors) > 30:
    over += 1
  if len(customer.flavors) > maxFlavors:
    maxFlavors = len(customer.flavors)
  return customer

for i in range(caseCount):
#  maxFlavors = 0
  flavorsN = int(raw_input())
  customersN = int(raw_input())
  customers = []
  for j in range(customersN):
    customers.append(buildCustomer(raw_input()))  
  flavorCounts = []
  for i in range(0, maxFlavors + 1):
    flavorCounts.append([])  
  for c in customers:
    flavorCounts[len(c.flavors)].append(c)
    

#  flavors = [0] * flavorsN
#  for customers in flavorCounts:
#    for c in customers:
#      for flavor in customers.flavor:
#        

  
#  print customers
#  print flavorCounts
#  print "Case #" + str(i+ 1) + ": "

print maxFlavors
print over
