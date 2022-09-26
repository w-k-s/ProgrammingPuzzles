(in-package :cl-user)
(defpackage cash-register-tests
  (:use :cl :fiveam))
(in-package :cash-register-tests)

(def-suite all-tests
    :description "The master suite of all quasiRPG tests.")

(in-suite all-tests)

(test dummy-tests
  "Just a placeholder."
  (is (listp (list 1 2)))
  (is (= 5 (+ 2 3))))