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

	jobs: prepareNextRelease: {
		name:      "prepare next release"
		"runs-on": defaultRunner
		steps: [
			_#generateToken,
			_#checkoutCode & {with: ref: defaultBranch},
			_#installRust,
			_#cacheRust,
			_#installTool & {with: tool: "cargo-release"},
			{
				name: "Update cargo dependencies for package ${{ inputs.package }}"
				run: """
					cargo update --package "${{ inputs.package }}"
					"""
			},
			{
				name: "Bump ${{ inputs.level }} version for package ${{ inputs.package }}"
				run: """
					cargo release version -v --execute --no-confirm --package "${{ inputs.package }}" "${{ inputs.level }}"
					"""
			},
			{
				name: "Perform pre-release replacements"
				run: """
					cargo release replace -v --execute --no-confirm --package "${{ inputs.package }}"
					"""
			},
			{
				name: "Fix up Markdown formatting"
				run:  "npx --yes prettier@\(_prettierVersion) --color --prose-wrap always --write -- **/*.md"
			},
			{
				name: "Get the new release version"
				run: """
					version=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name == "${{ inputs.package }}") | .version')
					echo "release_version=${version}" >> "$GITHUB_ENV"
					"""
			},
			_#createPullRequest & {
				with: {
					token:            "${{ steps.generate_token.outputs.token }}"
					"branch-suffix":  "timestamp"
					"commit-message": "Prepare ${{ inputs.package }} v${{ env.release_version }} ${{ inputs.level }} release"
					title:            "Prepare ${{ inputs.package }} v${{ env.release_version }} ${{ inputs.level }} release"
					body:             """
						This PR was automatically created by the [bump-version](https://github.com/${{ github.repository }}/actions/runs/${{ github.run_id }}) workflow, which ran the following commands:


						```
						$ cargo update --package ${{ inputs.package }}
						$ cargo release version --execute --package ${{ inputs.package }} ${{ inputs.level }}
						$ cargo release replace --execute --package ${{ inputs.package }}
						$ prettier --prose-wrap always --write -- **/*.md
						```

						**Please review the submitted changes.**
						
						Once this PR is merged into the _\(defaultBranch)_ branch, an automated process will tag the rebased commit.
						"""
				}},
			{
				name: "Annotate workflow run with PR URL"
				run: """
					echo "#### :shipit: Opened pull request for ${{ inputs.package }} v${{ env.release_version }}: ${{ steps.cpr.outputs.pull-request-url }}" >> "$GITHUB_STEP_SUMMARY"
					"""
			},
		]
	}
}
