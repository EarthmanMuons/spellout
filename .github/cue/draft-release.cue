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
						echo "### :shipit: Opened draft release for: [spellout ${{ github.ref_name }}](${{ steps.gh_release.outputs.url }})" >> "$GITHUB_STEP_SUMMARY"
						"""
				},
			]
		}

		upload_assets: {
			name: "upload release assets"
			needs: ["create_release"]
			defaults: run: shell: "bash"
			strategy: {
				"fail-fast": false
				matrix: include: [
					{target: "aarch64-apple-darwin", os:       "macos-latest", build_tool:   "cargo"},
					{target: "aarch64-pc-windows-msvc", os:    "windows-latest", build_tool: "cargo"},
					{target: "aarch64-unknown-linux-musl", os: "ubuntu-latest", build_tool:  "cross"},
					{target: "x86_64-apple-darwin", os:        "macos-latest", build_tool:   "cargo"},
					{target: "x86_64-pc-windows-msvc", os:     "windows-latest", build_tool: "cargo"},
					{target: "x86_64-unknown-linux-musl", os:  "ubuntu-latest", build_tool:  "cargo"},
				]}
			"runs-on": "${{ matrix.os }}"
			env: GH_TOKEN: "${{ secrets.GITHUB_TOKEN }}"
			steps: [
				_#checkoutCode,
				{
					name: "Install musl tools"
					if:   "matrix.os == 'ubuntu-latest'"
					run:  "sudo apt-get install -y musl-tools"
				},
				_#installRust & {with: targets: "${{ matrix.target }}"},
				_#cacheRust,
				_#installTool & {
					with: tool: "cross"
					if: "matrix.os == 'ubuntu-latest'"
				},
				{
					name: "Building release assets"
					run: """
						if [[ "${{ matrix.build_tool }}" == "cross" ]]; then
							cargo xtask dist --target "${{ matrix.target }}" --cross
						else
							cargo xtask dist --target "${{ matrix.target }}"
						fi
						"""
				},
				{
					name: "Uploading release assets"
					run: """
						if [[ "${{ matrix.os }}" == "windows-latest" ]]; then
						  extension="zip"
						else
						  extension="tar.gz" 
						fi
						filename="spellout-${GITHUB_REF_NAME:1}-${{ matrix.target }}.${extension}"

						echo "Uploading ${filename} to: ${{ needs.create_release.outputs.upload_url }}"
						gh release upload "$GITHUB_REF_NAME" "target/dist/${filename}"

						echo ":arrow_up: Uploaded release asset ${filename}" >>"$GITHUB_STEP_SUMMARY"
						"""
				},
			]
		}
	}
}
