# Generates fixtures/nebula.gitlog — a deterministic synthetic Git history in
# the exact format repohelix expects (git log --numstat with custom separators).
#
# This is a build helper, not part of the shipped tool. Run it from the repo
# root:  pwsh -File scripts/gen_fixture.ps1
#
# The generated history models a small fictional web service ("nebula") so the
# demos show realistic churn, co-change coupling, hotspots and clusters.

$ErrorActionPreference = 'Stop'
