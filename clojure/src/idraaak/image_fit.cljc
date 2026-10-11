(ns idraaak.image-fit
  "Integer contain and centered-cover geometry for Clojure and ClojureScript.
  This namespace never reads, modifies, or uploads image files.")

(def max-dimension
  "Largest accepted input or output dimension, in pixels.
  This bound keeps cross-products exact in JavaScript as well as on the JVM."
  1000000)

(defn- fail! [type message data]
  (throw (ex-info message (assoc data :type type))))

(defn size
  "Return {:width width :height height}.
  Both dimensions must be integers from 1 through max-dimension."
  [width height]
  (doseq [[axis value] [[:width width] [:height height]]]
    (when-not (and (integer? value) (<= 1 value max-dimension))
      (fail! ::invalid-dimension
             "Dimensions must be integers from 1 through 1000000."
             {:axis axis :value value})))
  {:width width :height height})

(defn- checked-size [value]
  (when-not (map? value)
    (fail! ::invalid-dimension "A size must be a map with width and height."
           {:value value}))
  (size (:width value) (:height value)))

(defn- checked-option [value]
  (when-not (or (true? value) (false? value))
    (fail! ::invalid-option "allow-upscale? must be true or false."
           {:value value})))

(defn- output-size [width height]
  (cond
    (or (zero? width) (zero? height))
    (fail! ::unrepresentable-size
           "The proportional contain result would have a zero-pixel dimension."
           {:width width :height height})

    (or (> width max-dimension) (> height max-dimension))
    (fail! ::dimension-overflow
           "A calculated dimension exceeds 1000000 pixels."
           {:width width :height height})

    :else (size width height)))

(defn- ceil-div [numerator denominator]
  (+ (quot numerator denominator)
     (if (zero? (rem numerator denominator)) 0 1)))

(defn contain
  "Fit the whole source inside bounds. The non-limiting dimension rounds down.
  With allow-upscale? false (the default), an already-fitting source is unchanged.
  Throws ExceptionInfo (Error in JS) with a namespaced :type on invalid input
  or a result that would round to zero pixels."
  ([source bounds] (contain source bounds false))
  ([source bounds allow-upscale?]
   (let [{sw :width sh :height :as source} (checked-size source)
         {bw :width bh :height} (checked-size bounds)]
     (checked-option allow-upscale?)
     (if (and (not allow-upscale?) (<= sw bw) (<= sh bh))
       source
       (if (>= (* sw bh) (* sh bw))
         (output-size bw (quot (* sh bw) sw))
         (output-size (quot (* sw bh) sh) bh))))))

(defn cover
  "Plan a resize followed by a centered crop that fills target.
  Returns {:resized {:width ... :height ...}
           :crop {:x ... :y ... :width ... :height ...}}.
  Crop coordinates refer to the RESIZED image. The non-limiting dimension
  rounds up; an odd extra removed pixel lies on the right or bottom.
  Throws ::upscaling-required if filling target requires enlargement while
  allow-upscale? is false (the default)."
  ([source target] (cover source target false))
  ([source target allow-upscale?]
   (let [{sw :width sh :height} (checked-size source)
         {tw :width th :height} (checked-size target)]
     (checked-option allow-upscale?)
     (when (and (not allow-upscale?) (or (> tw sw) (> th sh)))
       (fail! ::upscaling-required
              "Covering this target requires enlargement."
              {:source (size sw sh) :target (size tw th)}))
     (let [{rw :width rh :height :as resized}
           (if (>= (* sw th) (* sh tw))
             (output-size (ceil-div (* sw th) sh) th)
             (output-size tw (ceil-div (* sh tw) sw)))]
       {:resized resized
        :crop {:x (quot (- rw tw) 2)
               :y (quot (- rh th) 2)
               :width tw
               :height th}}))))
