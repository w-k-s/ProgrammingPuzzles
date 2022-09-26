(defsystem :cash-register
  :version "0.0.1"
  :author "w-k-s"
  :license ""
  :depends-on ()
  :components ((:module "src"
                :components
                ((:file "cash-register"))))
  :description "")

(defsystem :cash-register/tests
  :version "0.0.1"
  :author "w-k-s"
  :license ""
  :depends-on ()
  :components ((:module "tests"
                :components
                ((:file "cash-register-test"))))
  :description "")
