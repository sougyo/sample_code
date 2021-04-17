import Control.Monad.Trans.Writer.Lazy
import Control.Monad.Reader

mywriter :: WriterT String (Reader String) Int
mywriter = do a <- ask
              tell $ "Hello, " ++ a 
              return $ length a

myreader :: ReaderT String (Writer String) Int
myreader = do a <- ask
              lift $ tell $ "Hello, " ++ a
              return $ length a

main = do
  let rw_result = runReader (runWriterT mywriter) "hoge" :: (Int, String)
  let wr_result = runWriter (runReaderT myreader "hoge") :: (Int, String)
  putStrLn $ show rw_result
  putStrLn $ show wr_result

