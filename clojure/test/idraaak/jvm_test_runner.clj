(ns idraaak.jvm-test-runner
  (:require [clojure.test :as t]
            [idraaak.image-fit-test]))

(defn -main [& _]
  (let [{:keys [fail error]} (t/run-tests 'idraaak.image-fit-test)]
    (System/exit (if (zero? (+ fail error)) 0 1))))
