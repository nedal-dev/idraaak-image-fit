# Idraaak Image Fit for Clojure and ClojureScript

Calculate integer resize dimensions and centered crop rectangles before passing them to an image-processing library. This package calculates geometry only. It does not open, modify, compress, save, or upload images.

Created for [Idraaak](https://idraaak.com/), a technology publication. MIT licensed. Developed with AI assistance, with a shared automated test suite for the JVM and JavaScript.

## Status

Version 0.1.0 is being prepared for Clojars. It is not yet published; the dependency coordinates below become available after deployment. The Rust library at the repository root is a separate package and remains available.

## Dependency coordinates after publication

Leiningen:

```clojure
[io.github.nedal-dev/idraaak-image-fit "0.1.0"]
```

Clojure CLI:

```clojure
{:deps {io.github.nedal-dev/idraaak-image-fit {:mvn/version "0.1.0"}}}
```

For ClojureScript, include ClojureScript itself in your application as usual. The package contains one portable `.cljc` namespace and adds no image-processing dependencies.

## Keep the entire image visible

```clojure
(require '[idraaak.image-fit :as fit])

(fit/contain (fit/size 1200 800) (fit/size 500 375))
;; => {:width 500, :height 333}
```

Contain fits inside the bounds without cropping. The non-limiting dimension rounds down, so the result never exceeds the box. It does not add padding.

## Fill a thumbnail and crop the excess

```clojure
(fit/cover (fit/size 1200 800) (fit/size 500 375))
;; => {:resized {:width 563, :height 375},
;;     :crop {:x 31, :y 0, :width 500, :height 375}}
```

Resize to 563 by 375 pixels first, then crop 500 by 375 starting at x = 31, y = 0. These coordinates refer to the **resized** image. An odd extra removed pixel goes on the right or bottom.

## Enlargement

Both functions disable enlargement by default. Pass `true` as the third argument to allow it.

```clojure
(fit/contain (fit/size 80 40) (fit/size 500 375))
;; => {:width 80, :height 40}

(fit/contain (fit/size 80 40) (fit/size 500 375) true)
;; => {:width 500, :height 250}
```

Cover throws an error when enlargement is necessary but disabled. It does not silently return a result that leaves the target partly empty.

## Bounds and errors

Inputs and calculated output dimensions must be positive integers up to **1,000,000 pixels per axis**. This explicit bound keeps multiplication exact in both the JVM and JavaScript implementations. It differs from the Rust package's numeric range. Geometry calculations do not guarantee that an image library or machine can allocate a buffer of that size.

Failures are `ex-info` exceptions. Use `ex-data` to read their namespaced `:type`:

| Type | Meaning |
| --- | --- |
| `:idraaak.image-fit/invalid-dimension` | Missing, non-integer, zero, negative, or out-of-range input |
| `:idraaak.image-fit/invalid-option` | Enlargement option is not a boolean |
| `:idraaak.image-fit/unrepresentable-size` | Contain would round one dimension to zero pixels |
| `:idraaak.image-fit/upscaling-required` | Cover needs forbidden enlargement |
| `:idraaak.image-fit/dimension-overflow` | A calculated output dimension exceeds the supported bound |

Pixel rounding can slightly change the exact mathematical aspect ratio. The package does not interpret EXIF orientation: use your image library's oriented dimensions.

## Develop and build

From this repository's `clojure/` directory, with Java 17+, Node.js and Leiningen:

```text
lein run -m idraaak.jvm-test-runner
lein with-profile +cljs run -m cljs.main -O simple -t node -o target/cljs-tests.js -c idraaak.cljs-test-runner
node target/cljs-tests.js
lein with-profile -dev jar
lein with-profile -dev pom
```

The tests cover the documented landscape and portrait examples, enlargement rules, invalid inputs, subpixel output, overflow, and geometry invariants across 256 source/target combinations on each runtime.

See [publishing instructions](PUBLISHING.md) for Clojars account verification and deployment. Credentials are never required for tests or building.
