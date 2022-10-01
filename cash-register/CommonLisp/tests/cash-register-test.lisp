(in-package :cl-user)
(defpackage cash-register-tests
  (:use :cl :fiveam :cash-register))
(in-package :cash-register-tests)

(def-suite all-tests
    :description "The master suite of all Cash Register tests.")

(in-suite all-tests)

(test calculate-change
  "Change calculated correct"
  (is (string= "ERROR"        (cash-register::calculate-change 17.00 16.00)))
  (is (string= "ZERO"         (cash-register::calculate-change 16.00 16.00)))
  (is (string= "FIVE"         (cash-register::calculate-change 15 20)))
  (is (string= "NICKEL,PENNY" (cash-register::calculate-change 15.94 16.00)))
  (is (string= "FIFTY,FIVE,TWENTY,TWENTY,TWO,TWO"        
                              (cash-register::calculate-change 1.00 100.00))))