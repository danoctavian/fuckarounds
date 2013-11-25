
type Point = (Float, Float)
data Segment = Segment (Point, Point) Point

getSegIntersection :: Segment -> Segment -> Maybe Point
getSegIntersection (Segment ((s1x1, s1y1), (s1x2, s1y2)) (s1a, s1b))
                (Segment ((s2x1, s2y1), (s2x2, s2y2)) (s2a, s2b))
  = undefined

getLineIntersection :: Point -> Point -> Maybe Point
getLineIntersection (a1, b1) (a2, b2)
  | (a1 - a2) /= 0 = Just ((b2 - b1) / (a1 - a2), :)
  | otherwise = Nothing

