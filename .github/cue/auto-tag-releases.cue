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
		name:      "tag untagged"
		"runs-on": defaultRunner
		env: {
			GIT_COMMITTER_EMAIL: "noreply@github.com"
			GIT_COMMITTER_NAME:  "GitHub"
		}
		steps: [
			_#generateToken,
			_#checkoutCode & {with: {
				ref:           defaultBranch
				token:         "${{ steps.generate_token.outputs.token }}"
				"fetch-depth": 0
			}},
			_#installRust,
			_#cacheRust,
			_#installTool & {with: tool: "cargo-release"},
			{
				id:   "before"
				name: "Capture tags before"
				run: """
					echo "tags=$(git tag --list)" >> "$GITHUB_OUTPUT"
					"""
			},
			{
				name: "Add any missing tags"
				run:  "cargo release tag -v --execute --no-confirm"
			},
			{
				name: "Push any new tags"
				run:  "cargo release push -v --execute --no-confirm"
			},
			{
				id:   "after"
				name: "Capture tags after"
				run: """
					echo "tags=$(git tag --list)" >> "$GITHUB_OUTPUT"
					"""
			},
			{
				name: "Annotate workflow run with new tags"
				run: """
					echo "#### :sparkles: New tags:" >> "$GITHUB_STEP_SUMMARY";
					comm -3 <(echo "${{ steps.before.outputs.tags }}") <(echo "${{ steps.after.outputs.tags }}") | sed -e 's/^[[:space:]]*//' -e 's/^/- /' >> "$GITHUB_STEP_SUMMARY";
					"""
			},
		]
	}
}
