import Data.Functor ((<$>))
import Control.Applicative ((<|>))

infixl 2 :@:

data Term = Idx Int
          | Term :@: Term
          | Lmb Term
          deriving (Eq, Read, Show)


shift :: Int -> Term -> Term
shift val term = go 0 term
  where
    go :: Int -> Term -> Term
    go depth (Idx i)     = if i < depth then Idx i else Idx $ i + val
    go depth (t1 :@: t2) = (go depth t1) :@: (go depth t2)
    go depth (Lmb t)     = Lmb $ go (succ depth) t


substDB :: Int -> Term -> Term -> Term
substDB j s t = go 0 t
  where
    go :: Int -> Term -> Term
    go depth (Idx i)      = if (i - depth) == j then (shift depth s) else Idx i
    go depth (t1 :@: t2)  = go depth t1 :@: go depth t2
    go depth (Lmb t')     = Lmb $ go (succ depth) t'


betaRuleDB :: Term -> Term
betaRuleDB (t@(Lmb _) :@: s) = let (Lmb r) = substDB (-1) s t in shift (-1) r


oneStepDBN :: Term -> Maybe Term
oneStepDBN t@((Lmb _) :@:  _) = Just $ betaRuleDB t
oneStepDBN   (t1      :@: t2) = (:@: t2) <$> oneStepDBN t1 <|> (t1 :@:) <$> oneStepDBN t2
oneStepDBN   (Lmb t) = Lmb <$> oneStepDBN t
oneStepDBN   (Idx _) = Nothing


oneStepDBA :: Term -> Maybe Term
oneStepDBA   t@(f@(Lmb _) :@:  a) = (f :@: ) <$> oneStepDBN a <|> (Just $ betaRuleDB t)
oneStepDBA   (t1      :@: t2) = (:@: t2) <$> oneStepDBN t1 <|> (t1 :@:) <$> oneStepDBN t2
oneStepDBA   (Lmb t) = Lmb <$> oneStepDBN t
oneStepDBA   (Idx _) = Nothing

cIDB = Lmb (Idx 0)
comegaDB = Lmb (Idx 0 :@: Idx 0)
cOmegaDB = comegaDB :@: comegaDB
test = cIDB :@: cOmegaDB