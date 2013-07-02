import Network.HTTP
import Control.Applicative
import Text.JSON
import Control.Monad


main :: IO ()


haskurl = "http://www.haskell.org/"
google = "http://www.google.com/"

jsonSample = "{\"query\" : 1}"

jmap :: (JSValue -> a) -> JSValue -> [a]
jmap f s@(JSArray js) = (f s) : concatMap (jmap f) js
jmap f (JSObject obj) = concatMap ((jmap f) . snd) (fromJSObject obj)
jmap f val = [f val]

fromOk :: Result a -> a
fromOk (Ok x) = x

--extContent :: JsValue -> Maybe String

getContent :: String -> Result String
getContent jsString = do
    json <- decode jsString :: Result JSValue
    x <- return $  jmap (\x -> Nothing) json
    return "wtf"

wikimain = "http://en.wikipedia.org/w/api.php?format=json&action=query&titles=Main%20Page&prop=revisions&rvprop=content"

main = do
  putStrLn "wtf"
  body <- getBody wikimain 
  putStrLn body
  return ()

getBody url = simpleHTTP (getRequest url) >>= getResponseBody

