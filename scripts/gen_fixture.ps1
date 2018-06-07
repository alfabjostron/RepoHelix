# Generates fixtures/nebula.gitlog — a deterministic synthetic Git history in
# the exact format repohelix expects (git log --numstat with custom separators).
#
# This is a build helper, not part of the shipped tool. Run it from the repo
# root:  pwsh -File scripts/gen_fixture.ps1
#
# The generated history models a small fictional web service ("nebula") so the
# demos show realistic churn, co-change coupling, hotspots and clusters.

$ErrorActionPreference = 'Stop'

$SEP = [char]0x1e + 'REPOHELIX_COMMIT' + [char]0x1e
$FS  = [char]0x1f

# Author identities (synthetic; any resemblance to real people is coincidental).
$authors = @{
    ada   = @('Ada Reyes',   'ada@nebula.example')
    bo    = @('Bo Tanaka',   'bo@nebula.example')
    cy    = @('Cy Okafor',   'cy@nebula.example')
    dee   = @('Dee Marsh',   'dee@nebula.example')
}

# Each commit: hash, author key, unix time, subject, and file numstat rows
# as "added removed path". Times are chosen to form 4 development sessions
# (clusters) separated by multi-day gaps.
$commits = @(
    # ---- Session 1: bootstrap (day 0) ----
    @{ h='a1000001'; a='ada'; t=1704067200; s='Bootstrap project layout';
       f=@('40 0 src/main.rs','25 0 src/lib.rs','12 0 Cargo.toml','30 0 README.md') },
