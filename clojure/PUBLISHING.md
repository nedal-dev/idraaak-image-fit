# Publishing to Clojars

The Clojars account `idraaak` owns the automatically assigned group `org.clojars.idraaak`. The configured coordinates are `org.clojars.idraaak/idraaak-image-fit`, version `0.1.0`. The account and group are ready; this version has not been published yet.

## Store the deploy credential

1. Sign in to [Clojars deploy tokens](https://clojars.org/tokens) as `idraaak`.
2. Create a token named `idraaak-image-fit-github`. Choose `org.clojars.idraaak/*` as its scope for the first deployment. Clojars only offers artifact-specific scopes for existing artifacts, so this first publication cannot select the new artifact in the scope dropdown. After publication, use an artifact-specific token for future releases.
3. Choose an expiry of `1 Day`. Leave `Single use?` unchecked so retrying an interrupted first deployment remains possible. Disable the token after the successful release.
4. Copy the token directly to a [GitHub Actions repository secret](https://github.com/nedal-dev/idraaak-image-fit/settings/secrets/actions/new) named `CLOJARS_DEPLOY_TOKEN`. Do not paste its value into chat, Notion, source files, issue comments, or command output.

The workflow supplies the fixed username `idraaak` and reads the token from the secret. Tests and building require no credential.

## Request a tested release

The workflow runs the JVM and ClojureScript tests, builds the production JAR and POM, and checks the group, version, license, Homepage and exact Git revision before any deployment.

Publishing is enabled only for a push to `clojure-image-fit` whose head commit message contains `[publish-clojars]`. Ordinary pushes, pull requests and manual test runs do not publish. After storing the secret, update this publishing status to record the release request, then commit that update with the marker.

The deployment command is:

```text
lein with-profile -dev deploy clojars
```

A missing or rejected token stops the deployment; it does not count as publication. If a requested release fails because the token is missing, correct the secret and rerun the failed GitHub Actions job.

The Homepage field comes from `:url "https://idraaak.com/"` in `project.clj`. SCM metadata points to this repository and the exact tested Git revision, including the independent release branch.

## Verify the result

After successful deployment:

- Open the public [Clojars artifact page](https://clojars.org/org.clojars.idraaak/idraaak-image-fit) and confirm version `0.1.0` and the Homepage destination.
- Inspect the rendered anchor's `href` and `rel`, the page's robots directives, and crawl access. A successful upload does not prove indexing.
- Check that [cljdoc](https://cljdoc.org/d/org.clojars.idraaak/idraaak-image-fit/0.1.0) builds this artifact and displays the Clojure README rather than the root Rust README.
- Update the README status, record the exact public URLs and verification results, disable the Clojars token, and remove the temporary GitHub secret.

Stable versions are immutable. Correct a released version with a new version rather than attempting to replace `0.1.0`.
