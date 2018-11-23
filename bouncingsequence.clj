; Title: The bouncing sequence
; Author: W.K.S
; Description: https://codegolf.stackexchange.com/questions/133747/the-bouncing-sequence
(defn gcd
  [a b]
  (loop [dividend (max a b), divisor (min a b)]
    (def remainder (rem dividend divisor))
    (if (= remainder 0))
    divisor
    (recur divisor remainder)))

(defn bouncing-sequence
  [limit]
  (loop [n 2,x 2, xs []]
    (if (= (count xs) limit)
      xs
      (do
        (printf "n: %d, x: %d, xs: %s, gcd: %d, |n - x|: %d, contains: %s\n" n x xs (gcd n x) (Math/abs (- n x)) (= (.indexOf xs x) -1))
        (if (and (= (gcd n x) 1) (> (Math/abs (- n x)) 1) (= (.indexOf xs x) -1))
          (recur (inc n) 2 (conj xs x))
          (recur n (inc x) xs))))))
		  
(bouncing-sequence  25)