# Release Process

The release process for our project is highly automated, ensuring consistent and
reliable releases with a reduced possibility of human error. Though we use
[`cargo-release`][] behind the scenes, no local tooling is required to perform a
release, as we rely on workflows triggered via GitHub Actions.

[`cargo-release`]: https://github.com/crate-ci/cargo-release/

## Shipping a New Version

Follow these steps to ship a new version:

1. **Update the Changelog**: Ensure that the appropriate `CHANGELOG.md` file
   accurately reflects all significant changes for the upcoming release under
   the "Unreleased" section.

2. **Bump the Version**: Run the [bump-version][] workflow from the GitHub
   Actions tab. Here, you can select the package to release and the type of
   release (patch, minor, or major). This action creates a pull request,
   updating all necessary version numbers across the codebase and documentation.

3. **Review and Merge the Pull Request**: Review the changes in the pull request
   from step 2. After verifying, merge the pull request. The
   [auto-tag-release][] workflow then runs on every push to the `main` branch,
   generating a tag for the new version if one does not already exist.

4. **Verify the Publication**:
   - For Libraries: If the new tag corresponds to a library, the
     [publish-crate][] workflow is automatically triggered to build and publish
     the crate to https://crates.io/. When the workflow completes, it will
     display an annotation with a link to the crate's page where you can confirm
     successful publication by examining the displayed metadata.
   - For Binaries: If the new tag corresponds to a binary, the [draft-release][]
     workflow is triggered instead. This workflow generates a new draft GitHub
     release and attaches binary package files to the release. When the workflow
     completes, it will display an annotation with a link to the draft release
     for your review. Make any desired edits to the release notes, optionally
     select the checkbox to create a discussion for the release, and finally
     click the _Publish release_ button to make the release public.

[bump-version]:
  https://github.com/EarthmanMuons/spellout/actions/workflows/bump-version.yml
[auto-tag-release]:
  https://github.com/EarthmanMuons/spellout/actions/workflows/auto-tag-releases.yml
[publish-crate]:
  https://github.com/EarthmanMuons/spellout/actions/workflows/publish-crate.yml
[crate's page]: https://crates.io/crates/spellabet/
[draft-release]:
  https://github.com/EarthmanMuons/spellout/actions/workflows/draft-release.yml
