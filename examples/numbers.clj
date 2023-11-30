
(([zero one succ] (succ (succ zero)))
  ([f x] x)
  ([f x] (f x))
  ([n f x] (f (n f x))))