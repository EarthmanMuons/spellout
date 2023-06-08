package workflows

draftRelease: {
	name: "draft-release"

	on: push: tags: [ "v[0-9]+.[0-9]+.[0-9]+"]

	permissions: contents: "write"

	jobs: {
		draft_release: {
			name:      "create draft release"
			"runs-on": defaultRunner
			outputs: {
				upload_url: "${{ steps.gh_release.outputs.upload_url }}"
				url:        "${{ steps.gh_release.outputs.url }}"
			}
			steps: [
				_#checkoutCode,
				_#githubRelease & {with: {
					draft:                    true
					generate_release_notes:   true
					discussion_category_name: "announcements"
				}},
				{
					name: "Annotate workflow run with draft release URL"
					run: """
						echo "#### :shipit: Opened draft release for v${{ github.ref_name }}: ${{ steps.gh_release.outputs.url }}" >> "$GITHUB_STEP_SUMMARY"
						"""
				},
			]
		}

		upload_artifacts: {
			name: "upload release artifacts"
			needs: ["draft_release"]
			"runs-on": defaultRunner
			steps: [
				{
					name: "Building release artifacts"
					run: """
						echo "Uploading to ${{ needs.draft_release.outputs.upload_url }}"
						"""
				},
			]
		}
	}
}
