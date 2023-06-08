package workflows

autoTagReleases: {
	name: "auto-tag-releases"

	on: {
		push: branches: [defaultBranch]

		// Allow manually running this workflow.
		workflow_dispatch: null
	}

	env: {
		CARGO_INCREMENTAL: 0
		CARGO_TERM_COLOR:  "always"
		RUST_BACKTRACE:    1
		RUSTFLAGS:         "-D warnings"
	}

	jobs: tagUntagged: {
		name:      "tag untagged package releases"
		"runs-on": defaultRunner
		steps: [
			_#generateToken,
			_#checkoutCode & {with: ref: defaultBranch},
			_#installRust,
			_#cacheRust,
			_#installTool & {with: tool: "cargo-release"},
			{
				name: "Add any missing tags"
				run:  "cargo release tag -v --execute --no-confirm"
			},
			{
				name: "Push any new tags"
				env: GITHUB_TOKEN: "${{ steps.generate_token.outputs.token }}"
				run: "cargo release push -v --execute --no-confirm"
			},
		]
	}
}
