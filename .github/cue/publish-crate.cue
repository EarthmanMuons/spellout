package workflows

publishCrate: {
	name: "publish-crate"

	on: push: tags: [ "*-v[0-9]+.[0-9]+.[0-9]+"]

	env: {
		CARGO_INCREMENTAL: 0
		CARGO_TERM_COLOR:  "always"
		RUST_BACKTRACE:    1
		RUSTFLAGS:         "-D warnings"
	}

	jobs: publishUnpublished: {
		name:      "publish unpublished"
		"runs-on": defaultRunner
		steps: [
			_#checkoutCode & {with: "fetch-depth": 0},
			_#installRust,
			_#cacheRust,
			_#installTool & {with: tool: "cargo-release"},
			{
				name: "Publish crate to crates.io"
				env: CARGO_REGISTRY_TOKEN: "${{ secrets.CARGO_REGISTRY_TOKEN }}"
				run: """
					cargo release publish -v --execute --no-confirm --allow-branch="HEAD"
					"""
			},
			{
				name: "Annotate workflow run with published crate URL"
				run: """
					echo "### :shipit: Published crate for ${GITHUB_REF_NAME}:" >>"$GITHUB_STEP_SUMMARY"
					echo "- https://crates.io/crates/${GITHUB_REF_NAME%-v*}/${GITHUB_REF_NAME#*-v}/" >>"$GITHUB_STEP_SUMMARY"
					"""
			},
		]
	}
}
