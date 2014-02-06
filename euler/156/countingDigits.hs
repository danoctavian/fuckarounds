import Data.List

if' cond a b =  if cond then a else b

digits :: Integer -> [Integer]
digits n = reverse $ unfoldr (\n -> if' (n > 0)  (Just (n `mod` 10, n `div` 10)) Nothing) n

digitCount :: Integer -> Integer -> Integer
digitCount n d
 = sum $ map (\dig -> if' (dig >= d) (combinations - dig - 1)  0) digs
  where
    digs = digits n
    combinations = sum digs + (fromIntegral $ length digs)

