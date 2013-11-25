-- probabilistic labelling

--we make the assumption there are only 2 labels, therefore we store only the probability that it is an edge
import Data.Array.IArray
import Data.Maybe
import Data.Array (listArray)
import System.Environment
import Control.Monad

data Pixel = Pixel Float [Int]
  deriving Show
data Label = E | NE
type Comp = Label -> Label -> Float

type Pixels = Array Int Pixel

doSteps :: Pixels -> Comp -> Int -> Pixels
doSteps pixs comp n = foldl (\pixs step -> step pixs) pixs (take n $ repeat (stepProbLabelling comp))

stepProbLabelling :: Comp -> Pixels -> Pixels
stepProbLabelling  comp pixels
  = fmap (\pix@(Pixel _ neighbors) ->  Pixel (edgeProbability pixels comp pix) neighbors) pixels

edgeProbability :: Pixels -> Comp -> Pixel -> Float
edgeProbability pixs comp pix
  = suppTimesProb E / (suppTimesProb E + suppTimesProb NE)
    where
      suppTimesProb :: Label -> Float
      suppTimesProb label = labelProb pix label * totalContextSupport pixs comp pix label

totalContextSupport :: Pixels -> Comp -> Pixel -> Label -> Float 
totalContextSupport pixs comp pix@(Pixel p neighbors) label
  = foldl (+) 0 (map (\neigh-> contextSupp pix label (pixs ! neigh) comp) neighbors)

contextSupp :: Pixel -> Label -> Pixel -> Comp -> Float
contextSupp pix label otherPix comp = partialSupp E + partialSupp NE
  where
    partialSupp :: Label -> Float
    partialSupp label2 = (comp label label2) * (labelProb otherPix label2)
      
labelProb :: Pixel -> Label -> Float
labelProb (Pixel p _) E = p
labelProb (Pixel p _) NE = 1 - p

neighbors :: Int-> Int -> Int ->  [Int]
neighbors i w h
  = foldl (++) [] [getPixel (x + dx) (y + dy) w h | dx <- [-1..1],
                  dy <- [-1..1], not (dx == 0 && dy == 0)] 
    where
     (x, y) = (i `mod` w, i `div` w)

getPixel :: Int -> Int -> Int -> Int -> [Int]
getPixel x y w h = if' (x >= 0 && y >=0 && x < w && y < h) [y * w + x] []

getPixelProbArray :: String ->  Pixels
getPixelProbArray str
 = Data.Array.listArray (0, w * h - 1) $ map (\(i, p) -> Pixel p (neighbors i w h)) (zip [0..w * h - 1] probs)
  where
    nums :: [Float]
    nums = map read $ words str
    [w, h] = map round (take 2 nums)
    probs = drop 2 nums
                  
if' :: Bool -> a -> a -> a
if' True left _ = left
if' False _ right = right

compA :: Label -> Label -> Float
compA _ _ = 1

compB :: Label -> Label -> Float
compB E E = 2
compB _ _ = 1



main :: IO ()
main = do
   args <- getArgs
   content <- readFile (args !! 0)
   arr <- return $ doSteps (getPixelProbArray content) (if' ((args !! 1) == "A") compA compB) 2
   putStrLn $ show arr
