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
			_#checkoutCode,
			_#installRust,
			_#cacheRust,
			_#installTool & {with: tool: "cargo-release"},
			{
				name: "Publish any unpublished crates to crates.io"
				env: CARGO_REGISTY_TOKEN: "${{ secrets.CARGO_REGISTRY_TOKEN }}"
				run: """
					cargo release publish -v --execute --no-confirm --allow-branch="HEAD"
					"""
			},
		]
	}
}
