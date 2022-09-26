;gnu clisp  2.49.60
(in-package :cl-user)
(defpackage cash-register
  (:use :cl))
(in-package :cash-register)



(defconstant +penny+ 1)
(defconstant +nickel+ 5)
(defconstant +dime+ 10)
(defconstant +half-dollar+ 50)
(defconstant +dollar+ 100)
(defconstant +five+ 500)
(defconstant +ten+ 1000)
(defconstant +twenty+ 2000)
(defconstant +fifty+ 5000)
(defconstant +one-hundred+ 10000)

(defun denomination-name (value)
	(cond ((eql value +penny+) "PENNY") 
        ((eql value +nickel+) "NICKEL") 
        ((eql value +dime+) "DIME")
        ((eql value +half-dollar+) "HALF DOLLAR" )
        ((eql value +dollar+) "DOLLAR")
        ((eql value +five+) "FIVE")
        ((eql value +ten+) "TEN")
        ((eql value +twenty+) "TWENTY")
        ((eql value +fifty+) "FIFTY")
        ((eql value +one-hundred+) "ONE HUNDRED")))

(defun minor-units (decimal) 
	(floor (* decimal 100)))

(defun calculate-change (price cash)
    (let* (
      (price-minor-units (minor-units price))
      (cash-minor-units (minor-units cash))
      (change (list))
      (remaining (- cash-minor-units price-minor-units))
      (cash-register (list +one-hundred+ +fifty+ +twenty+ +ten+ +five+ +dollar+  +half-dollar+ +dime+ +nickel+ +penny+)))
      (cond ((eql remaining 0) "ZERO")
          ((< remaining 0) "ERROR")
          ((> remaining 0) (loop for denomination in cash-register 
                    			if (<= denomination remaining)
                    			collect (denomination-name denomination) into change
                    			finally (return change))))))

(print (format NIL "~{~A~^,~}" (sort (calculate-change 15.94 16.00) #'string-lessp)))