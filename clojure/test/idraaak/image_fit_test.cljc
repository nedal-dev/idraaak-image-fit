(ns idraaak.image-fit-test
  (:require [idraaak.image-fit :as fit]
            #?(:clj [clojure.test :refer [deftest is testing]]
               :cljs [cljs.test :refer-macros [deftest is testing]])))

(defn- error-type [f]
  (try (f) nil
       (catch #?(:clj clojure.lang.ExceptionInfo :cljs js/Error) e
         (:type (ex-data e)))))

(deftest documented-geometry
  (let [source (fit/size 1200 800)
        target (fit/size 500 375)]
    (is (= {:width 500 :height 333} (fit/contain source target)))
    (is (= {:resized {:width 563 :height 375}
            :crop {:x 31 :y 0 :width 500 :height 375}}
           (fit/cover source target))))
  (is (= {:width 234 :height 375}
         (fit/contain (fit/size 800 1280) (fit/size 500 375))))
  (is (= {:resized {:width 500 :height 800}
          :crop {:x 0 :y 212 :width 500 :height 375}}
         (fit/cover (fit/size 800 1280) (fit/size 500 375)))))

(deftest enlargement-policy
  (let [source (fit/size 80 40) bounds (fit/size 500 375)]
    (is (= source (fit/contain source bounds)))
    (is (= {:width 500 :height 250} (fit/contain source bounds true)))
    (is (= ::fit/upscaling-required
           (error-type #(fit/cover source bounds))))
    (is (= {:resized {:width 750 :height 375}
            :crop {:x 125 :y 0 :width 500 :height 375}}
           (fit/cover source bounds true))))
  (let [square (fit/size 100 100)]
    (is (= square (fit/contain square square)))
    (is (= {:resized square :crop {:x 0 :y 0 :width 100 :height 100}}
           (fit/cover square square)))))

(deftest invalid-inputs
  (doseq [bad [0 -1 0.5 nil "100" 1000001]]
    (is (= ::fit/invalid-dimension (error-type #(fit/size bad 100))))
    (is (= ::fit/invalid-dimension (error-type #(fit/size 100 bad)))))
  (doseq [bad [nil [] {:width 100} {:width 100 :height 0}]]
    (is (= ::fit/invalid-dimension
           (error-type #(fit/contain bad (fit/size 50 50)))))
    (is (= ::fit/invalid-dimension
           (error-type #(fit/cover (fit/size 100 100) bad true)))))
  (doseq [bad [nil 0 1 "false"]]
    (is (= ::fit/invalid-option
           (error-type #(fit/contain (fit/size 100 100) (fit/size 50 50) bad))))
    (is (= ::fit/invalid-option
           (error-type #(fit/cover (fit/size 100 100) (fit/size 50 50) bad)))))
  (is (= ::fit/unrepresentable-size
         (error-type #(fit/contain (fit/size 1000000 1) (fit/size 1 1)))))
  (is (= ::fit/dimension-overflow
         (error-type #(fit/cover (fit/size 1000000 1)
                                (fit/size 1000000 1000000) true)))))

(deftest dimension-boundaries
  (let [large (fit/size 1000000 999999)]
    (is (= large (fit/contain large large)))
    (is (= {:width 999999 :height 999998}
           (fit/contain large (fit/size 999999 999999))))
    (is (= {:resized {:width 1000000 :height 999999}
            :crop {:x 0 :y 0 :width 999999 :height 999999}}
           (fit/cover large (fit/size 999999 999999)))))
  (is (= {:width 1 :height 1}
         (fit/contain (fit/size 1000000 1000000) (fit/size 1 1)))))

(deftest geometry-invariants
  (doseq [sw [1 2 17 1024] sh [1 2 17 1024]
          tw [1 3 100 512] th [1 3 100 512]]
    (let [source (fit/size sw sh) target (fit/size tw th)
          result (try (fit/contain source target true)
                      (catch #?(:clj clojure.lang.ExceptionInfo :cljs js/Error) e e))]
      (if (map? result)
        (let [{rw :width rh :height} result]
          (is (<= 1 rw tw))
          (is (<= 1 rh th))
          (is (or (= rw tw) (= rh th)))
          ;; Rounding error is less than one pixel in the free dimension.
          (is (< #?(:clj (Math/abs (long (- (* rw sh) (* rh sw))))
                    :cljs (js/Math.abs (- (* rw sh) (* rh sw))))
                 (max sw sh))))
        (do
          (is (= ::fit/unrepresentable-size (:type (ex-data result))))
          (is (or (< (* sh tw) sw) (< (* sw th) sh))))))
    (let [{rw :width rh :height} (:resized (fit/cover (fit/size sw sh)
                                                    (fit/size tw th) true))
          {:keys [x y width height]} (:crop (fit/cover (fit/size sw sh)
                                                     (fit/size tw th) true))]
      (is (>= rw tw))
      (is (>= rh th))
      (is (= [width height] [tw th]))
      (is (<= 0 x (- rw tw)))
      (is (<= 0 y (- rh th)))
      (is (<= 0 (- rw tw (* 2 x)) 1))
      (is (<= 0 (- rh th (* 2 y)) 1)))))
