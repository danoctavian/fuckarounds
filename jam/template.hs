import Text.ParserCombinators.Parsec

-- implement the following
data Case = Case Int
  deriving Show

data Answer = A Int
  deriving Show

solveCase :: Case -> Answer
solveCase = undefined

parseCases :: String -> [Case]
parseCases = undefined 

-- skeleton code
main = do
  caseCount <- readInt
  outputs <- fmap makeOutput getContents
  mapM putStrLn outputs
  return ()

makeOutput :: String -> [String]
makeOutput = (map  strAnswer) . (zip [1..]) . (map solveCase) . parseCases 

strAnswer (i, a) = "Case #" ++ show i ++ ": " ++ show a

readInt :: IO (Int)
readInt = readLn
