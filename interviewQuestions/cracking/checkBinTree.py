import random as rand

class Node:
  def __init__(self, value):
    self.label = value
    self.left = None
    self.right = None

def getWrongRange(labelRange, minLabel, maxLabel):
  (mn, mx) = labelRange
  possibleRanges = []
  if minLabel < mn:
    possibleRanges.append((minLabel, mn))
  if maxLabel > mx:
    possibleRanges.append((mx, maxLabel))
  return rand.choice(possibleRanges)
    

def generateBinTree(size, unsortedP, labelRange):
  (minLabel, maxLabel) = labelRange
  
  def generateRandBinTree(size, labelRange):
    if size == 0:
      return (True, None)
    isSorted = True
    if rand.random() <= unsortedP:
      labelRange = getWrongRange(labelRange, minLabel, maxLabel)
      isSorted = False
    (mn, mx) = labelRange
    label = rand.uniform(mn, mx)
    root = Node(label)
    size = size - 1
    
    (leftIsSorted, left) = generateRandBinTree(size / 2, (mn, label))
    (rightIsSorted, right) = generateRandBinTree(size / 2, (label, mx))
    root.left = left
    root.right = right
    return (isSorted and leftIsSorted and rightIsSorted, root)

  return generateRandBinTree(size, labelRange)
     
#(isSorted, randTree) = generateBinTree(10, 0.1, (0, 1000))
def isBST(root):
  lastLabel = -1   
  def InOrder(root):
    if root == None: return True
    if not InOrder(root.left):
      return false
    if not (root.label >= lastLabel):
      return false
    label = root.label
    return InOrder(root.right)
  return  InOrder(root)


for x in range(0, 10):
  (isSorted, randTree) = generateBinTree(10, 0.1, (0, 1000))
  assert isSorted == isBST(randTree)  
