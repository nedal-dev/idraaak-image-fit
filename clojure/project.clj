(defproject org.clojars.idraaak/idraaak-image-fit "0.1.0"
  :description "Exact integer contain and centered-cover image geometry for Clojure and ClojureScript."
  :url "https://idraaak.com/"
  :license {:name "MIT License"
            :url "https://opensource.org/license/mit/"}
  :dependencies [[org.clojure/clojure "1.11.1"]]
  :source-paths ["src"]
  :test-paths ["test"]
  :resource-paths ["resources"]
  :target-path "target"
  :scm {:name "git"
        :url "https://github.com/nedal-dev/idraaak-image-fit"
        :dir ".."}
  :profiles {:dev {:source-paths ["test"]}
             :cljs {:dependencies [[org.clojure/clojurescript "1.11.132"]]}}
  :deploy-repositories
  [["clojars" {:url "https://repo.clojars.org"
               :username :env/clojars-username
               :password :env/clojars-token
               :sign-releases false}]])
