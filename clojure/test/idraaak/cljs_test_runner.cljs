(ns idraaak.cljs-test-runner
  (:require [cljs.test :as t :include-macros true]
            [idraaak.image-fit-test]))

(enable-console-print!)

(defmethod t/report [::t/default :end-run-tests] [result]
  (set! (.-exitCode js/process) (if (t/successful? result) 0 1)))

(t/run-tests 'idraaak.image-fit-test)
