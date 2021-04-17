import Control.Monad.Trans.Writer.Lazy
import Control.Monad.Reader
import Control.Monad.State.Lazy

data StEnv a = StEnv {
                 next :: a -> a,
                 stop :: a -> Bool
               }

st_writer :: WriterT [a] (ReaderT (StEnv a) (State a)) ()
st_writer = do env <- ask
               s <- get
               tell [s]
               if stop env s
                 then return ()
                 else put (next env s) >> st_writer

collatz x
 | mod x 2 == 0 = div x 2
 | otherwise = 3 * x + 1

exec_st_writer :: (StEnv a) -> a -> [a]
exec_st_writer env a = evalState (runReaderT (execWriterT st_writer) env) a

main = putStrLn . show $ exec_st_writer (StEnv collatz (\x -> x == 1)) (21::Integer)
