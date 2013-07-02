
data Attribute a = A {aName :: String, vals :: [a]}
data Datum a = D {dName :: String, attrs :: [(Attribute a, a)]}
