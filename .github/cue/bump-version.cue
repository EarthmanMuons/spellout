package workflows

bumpVersion: {
	name: "bump-version"

	on: {
		// Allow manually running this workflow.
		workflow_dispatch: {
			inputs: {
				package: {
					description: "Cargo package"
					required:    true
					default:     "spellout"
					type:        "choice"
					options: [
						"spellabet",
						"spellout",
					]
				}
				level: {
					description: "Bump level"
					required:    true
					type:        "choice"
					options: [
						"patch",
						"minor",
						"major",
					]
				}
			}
		}
	}

	env: {
		CARGO_INCREMENTAL: 0
		CARGO_TERM_COLOR:  "always"
		RUST_BACKTRACE:    1
		RUSTFLAGS:         "-D warnings"
	}

	jobs: prepareRelease: {
		name: "prepare release"
		permissions: {
			contents:        "write"
			"pull-requests": "write"
		}
		"runs-on": defaultRunner
		steps: [
			_#checkoutCode & {with: ref: defaultBranch},
			_#installRust,
			_#cacheRust,
			{
				name: "Update cargo dependencies for package {{ inputs.package }}"
				run: """
					cargo update --package "{{ inputs.package }}"
					"""
			},
			{
				name: "Install cargo-release"
				run:  "cargo install --locked cargo-release"
			},
			{
				name: "Bump {{ inputs.package }} crate version"
				run: """
					cargo release version --execute --no-confirm --package "{{ inputs.package }}" "{{ inputs.level }}"
					"""
			},
			{
				name: "Perform pre-release replacements"
				run: """
					cargo release replace --execute --no-confirm --package "{{ inputs.package }}"
					"""
			},
			// the changelog spacing will need a fixup
			_#prettier & {
				with: prettier_options: "--color --prose-wrap always --write **/*.md"
			},
			_#createPullRequest & {with: {
				"branch-suffix":  "short-commit-hash"
				"commit-message": "[{{ inputs.package }}] Prepare next {{ inputs.level }} release"
				title:            "[{{ inputs.package }}] Prepare next {{ inputs.level }} release"
				body: """
					This PR was auto-generated and ran the following commands:

					```
					$ cargo update --package "{{ inputs.package }}"
					$ cargo install --locked cargo-release
					$ cargo release version --execute --no-confirm --package "{{ inputs.package }}" "{{ inputs.level }}"
					$ cargo release replace --execute --no-confirm --package "{{ inputs.package }}"
					$ prettier --prose-wrap always --write **/*.md
					```

					Please review the submitted changes. After merging, the rebased commit will automatically be tagged and a draft release will be opened.
					"""
			}},
			{
				name: "Annotate workflow run with PR URL"
				run:  "echo \"### Opened Pull Request: {{ steps.cpr.outputs.pull-request-url }}\" >> \"$GITHUB_STEP_SUMMARY\""
			},
		]
	}
}
