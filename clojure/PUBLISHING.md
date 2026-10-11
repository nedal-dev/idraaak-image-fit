# Publishing to Clojars

This project has not been published to Clojars yet. Building or uploading source to GitHub does not create a Clojars listing.

1. Sign in to [Clojars](https://clojars.org/login) with the project's authorized account. If registering by email, use the approved backlink email, idraaak.com@gmail.com. Verify the email if prompted.
2. The configured group is `io.github.nedal-dev`. Confirm that the account owns this verified group before deployment. GitHub sign-in can verify groups associated with the GitHub account. If a different verified group must be used, update `project.clj`, the README coordinates and the cljdoc configuration together before building.
3. Create a deploy token limited to `io.github.nedal-dev/idraaak-image-fit`. Do not paste the token into GitHub source, a chat message, Notion, or a command that displays it.
4. Check out the exact tested revision, enter `clojure/`, rerun the tests if source changed, and build the JAR and POM.
5. Provide `CLOJARS_USERNAME` and `CLOJARS_TOKEN` through your local environment or a private secrets facility. Deploy with the following command.

```text
lein deploy clojars
```

The Homepage field comes from `:url "https://idraaak.com/"` in `project.clj`. SCM metadata points to this repository and the release's Git revision.

After successful deployment:

- Open the public Clojars page and confirm the version and Homepage destination.
- Inspect the actual rendered anchor's `href` and `rel`, the page's robots directives, and crawl access. Do not infer indexing from a successful upload.
- Check that cljdoc builds this artifact and displays the Clojure README rather than the root Rust README.
- Update the README status, then record the exact public URLs and verification results.

Stable versions are immutable. Correct a released version with a new version rather than attempting to replace 0.1.0.
