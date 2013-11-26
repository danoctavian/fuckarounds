import Data.List
import Data.List.Split
import Data.Set (Set)
import qualified Data.Set as Set
import Data.Maybe

type Point = (Float, Float)
data Segment = Segment (Point, Point) Point

segIntersection :: Segment -> Segment -> Maybe Point
segIntersection s1@(Segment (p11, p12) lineA@(s1a, s1b))
                s2@(Segment (p21, p22) lineB@(s2a, s2b))
  = do 
    inters <- lineIntersection lineA lineB
    if belongsTo inters s1 && belongsTo inters s2 && not (elem inters [p11, p12, p21, p22])
    then return inters
    else Nothing

belongsTo :: Point -> Segment -> Bool
belongsTo (x, y) (Segment (p1@(x1, y1), p2@(x2, y2)) _)
  = inInterval x x1 x2 && inInterval y y1 y2
    where
      inInterval x a b = let [low, high] = sort [a, b] in low <= x && x <=high

lineEquation :: Point -> Point -> Point 
lineEquation (x1, y1) (x2, y2)
  = ((y1 - y2) / (x1 - x2), (y2 * x1 - x2 * y1) / (x1 - x2))

lineIntersection :: Point -> Point -> Maybe Point
lineIntersection (a1, b1) (a2, b2)
  | (a1 - a2) /= 0 = Just ((b2 - b1) / (a1 - a2), (a1 * b2 - a2 * b1) / (a1 - a2))
  | otherwise = Nothing

blumBlumShub :: [Int]
blumBlumShub =  map (`mod` 500) (drop 1 sGen)
  where
    sGen = 290797 : (map (\x -> x^2 `mod` 50515093) sGen)

makeSegments :: [Int] -> [Segment]
makeSegments = (map toSeg) . (chunksOf 4)
toSeg :: [Int] -> Segment
toSeg coords = Segment (p1, p2) (lineEquation p1 p2)
               where
                  fc = map fromIntegral coords
                  p1 = (fc!!0, fc!!1)
                  p2 = (fc!!2, fc!!3)

countBSIntersections 
  = countIntersections . makeSegments . (take 20000) $ blumBlumShub

countIntersections ::[Segment] -> Int
countIntersections segs
  =  Set.size . Set.fromList . (map fromJust) . (filter (/= Nothing)) $
     [segIntersection s1 s2 | (s1 : ss) <- tails segs, s2 <- ss]

main :: IO()
main = do
 putStrLn $ show countBSIntersections
