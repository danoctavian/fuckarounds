specialSum n = sumUpTo n + sumUpToMults n d1 + sumUpToMults n d2 - sumUpToMults n (d1 * d2)
  where
    sumUpTo n = n * (n + 1) `div` 2 
    sumUpToMults n d = d * sumUpTo (n `div` d) -- sum up all multiples of d from 1 to n
    d1 = 3
    d2 = 5
