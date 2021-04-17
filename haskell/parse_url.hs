import Text.ParserCombinators.Parsec
import Control.Applicative((<*), (*>), (<*>), (<$>))

digits = many1 digit
(+*) l e = l ++ [e]
listM c = fmap (\x -> [x]) c

data HttpURL = HttpURL {
             h_host :: String,
             h_port :: Int,
             h_path :: String,
             h_search :: String
           } deriving Show

httpurl = do string "http://"
             h <- host
             port <- option "80" $ char ':' *> digits
             (hpath, hsearch) <- option ("", "") post
             eof
             return $ HttpURL h ((read port)::Int) hpath hsearch
  where
    post = char '/' *> argpair
    argpair = (,) <$> hpath <*> option "" search
    hpath = fmap concat $ (:) <$> hsegment <*> many slaseg
    slaseg = (:) <$> char '/' <*> hsegment
    search = char '?' *> hsegment

host = fmap concat $ try hostnumber <|> hostname
  where
    hostnumber = (:) <$> digits <*> count 3 dotdigits
    dotdigits = (:) <$> char '.' <*> digits
    hostname = (+*) <$> many (try domaindot) <*> label letter
    domaindot = (+*) <$> label alphaNum <*> char '.'
    label head = (:) <$> head <*> option "" repeat
    repeat = try ( (:) <$> c <*> repeat ) <|> listM alphaNum
    c = alphaNum <|> char '-'

hsegment = fmap concat (many segelem)
  where
    segelem = try escape <|> listM unreserved
    unreserved = letter <|> digit <|> oneOf "$-_.+!*'(),;:@&="
    escape = list3 <$> char '%' <*> hexDigit <*> hexDigit
    list3 a b c = a:b:c:[]

run input = case parse httpurl "" input of
              Left err -> putStrLn (show err)
              Right val -> putStrLn (show val)

main = run "http://hogehoge"
