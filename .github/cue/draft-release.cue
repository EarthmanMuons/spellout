package workflows

draftRelease: {
	name: "draft-release"

	on: push: tags: [ "v[0-9]+.[0-9]+.[0-9]+"]

	permissions: contents: "write"

	env: {
		CARGO_INCREMENTAL: 0
		CARGO_TERM_COLOR:  "always"
		RUST_BACKTRACE:    1
		RUSTFLAGS:         "-D warnings"
	}

	jobs: {
		create_release: {
			name:      "create draft release"
			"runs-on": defaultRunner
			outputs: {
				upload_url: "${{ steps.gh_release.outputs.upload_url }}"
				url:        "${{ steps.gh_release.outputs.url }}"
			}
			steps: [
				_#checkoutCode,
				_#githubRelease & {with: {
					draft:                  true
					generate_release_notes: true
				}},
				{
					name: "Annotate workflow run with draft release URL"
					run: """
						echo "#### :shipit: Opened draft release for: [spellout ${{ github.ref_name }}](${{ steps.gh_release.outputs.url }})" >> "$GITHUB_STEP_SUMMARY"
						"""
				},
			]
		}

		upload_assets: {
			name: "upload release assets"
			needs: ["create_release"]
			strategy: matrix: include: [
				// {
				// 	target: "aarch64-apple-darwin"
				// 	os:     "macos-latest"
				// },
				{
					target: "aarch64-unknown-linux-gnu"
					os:     "ubuntu-latest"
				},
				// {
				// 	target: "x86_64-apple-darwin"
				// 	os:     "macos-latest"
				// },
				{
					target: "x86_64-unknown-linux-gnu"
					os:     "ubuntu-latest"
				},
			]
			"runs-on": "${{ matrix.os }}"
			steps: [
				_#checkoutCode,
				_#installRust,
				_#cacheRust,
				_#setupCrossToolchain & {with: target: "${{ matrix.target }}"},
				{
					name: "Building release assets"
					run:  "cargo xtask dist"
				},
				{
					name: "Uploading release assets"
					if:   "matrix.os != 'windows-latest'"
					run: """
						ls target/dist/
						echo "Uploading spellout-${GITHUB_REF_NAME:1}-${{ matrix.target }}.tar.gz to: ${{ needs.create_release.outputs.upload_url }}"
						"""
				},
			]
		}
	}
}
