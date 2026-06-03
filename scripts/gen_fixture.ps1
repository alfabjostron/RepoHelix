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
    @{ h='a1000002'; a='ada'; t=1704069000; s='Add config loader';
       f=@('60 2 src/config.rs','8 1 src/lib.rs','5 0 Cargo.toml') },
    @{ h='a1000003'; a='bo';  t=1704070800; s='Introduce HTTP router';
       f=@('90 0 src/router.rs','14 2 src/main.rs','10 0 src/lib.rs') },
    @{ h='a1000004'; a='bo';  t=1704072600; s='Wire router to handlers';
       f=@('45 5 src/router.rs','70 0 src/handlers.rs','6 1 src/main.rs') },

    # ---- Session 2: data layer (day 3) ----
    @{ h='b2000001'; a='cy';  t=1704326400; s='Add storage module';
       f=@('120 0 src/storage.rs','9 0 src/lib.rs','7 2 Cargo.toml') },
    @{ h='b2000002'; a='cy';  t=1704328200; s='Storage: connection pooling';
       f=@('55 8 src/storage.rs','20 0 src/config.rs') },
    @{ h='b2000003'; a='ada'; t=1704330000; s='Handlers use storage';
       f=@('40 6 src/handlers.rs','30 4 src/storage.rs') },
    @{ h='b2000004'; a='ada'; t=1704331800; s='Add user model';
       f=@('65 0 src/models/user.rs','12 0 src/lib.rs') },
    @{ h='b2000005'; a='cy';  t=1704333600; s='User CRUD in handlers';
       f=@('80 10 src/handlers.rs','35 5 src/models/user.rs','15 3 src/storage.rs') },

    # ---- Session 3: features + churn on handlers/router (day 9) ----
    @{ h='c3000001'; a='bo';  t=1704844800; s='Add auth middleware';
       f=@('95 0 src/auth.rs','25 4 src/router.rs','18 6 src/handlers.rs') },
    @{ h='c3000002'; a='bo';  t=1704846600; s='Auth: token parsing';
       f=@('40 12 src/auth.rs','10 2 src/config.rs') },
    @{ h='c3000003'; a='dee'; t=1704848400; s='Add session model';
       f=@('50 0 src/models/session.rs','20 8 src/auth.rs','8 0 src/lib.rs') },
    @{ h='c3000004'; a='dee'; t=1704850200; s='Sessions in handlers';
       f=@('60 15 src/handlers.rs','30 5 src/models/session.rs','12 3 src/router.rs') },
    @{ h='c3000005'; a='ada'; t=1704852000; s='Refactor handlers split';
       f=@('120 90 src/handlers.rs','40 0 src/handlers_user.rs','25 4 src/router.rs') },
    @{ h='c3000006'; a='ada'; t=1704853800; s='Router: nested groups';
       f=@('70 30 src/router.rs','15 5 src/main.rs') },
    @{ h='c3000007'; a='cy';  t=1704855600; s='Storage: migrations';
       f=@('85 0 src/migrate.rs','40 10 src/storage.rs','6 0 Cargo.toml') },

    # ---- Session 4: hardening + docs (day 20) ----
    @{ h='d4000001'; a='dee'; t=1705795200; s='Add error types';
       f=@('70 0 src/errors.rs','30 20 src/handlers.rs','15 8 src/storage.rs','12 6 src/router.rs') },
    @{ h='d4000002'; a='dee'; t=1705797000; s='Propagate errors everywhere';
       f=@('45 30 src/handlers.rs','20 15 src/storage.rs','18 10 src/auth.rs') },
    @{ h='d4000003'; a='bo';  t=1705798800; s='Add metrics endpoint';
       f=@('55 0 src/metrics_ep.rs','20 4 src/router.rs','10 2 src/handlers.rs') },
    @{ h='d4000004'; a='ada'; t=1705800600; s='Config: env overrides';
       f=@('35 20 src/config.rs','8 3 src/main.rs') },
    @{ h='d4000005'; a='ada'; t=1705802400; s='Docs: architecture notes';
       f=@('120 10 README.md','40 0 docs/ARCHITECTURE.md') },
    @{ h='d4000006'; a='cy';  t=1705804200; s='Add integration tests';
       f=@('90 0 tests/integration.rs','15 5 src/lib.rs') },
    @{ h='d4000007'; a='dee'; t=1705806000; s='Fix router edge cases';
       f=@('30 25 src/router.rs','20 12 src/handlers.rs') },
    @{ h='d4000008'; a='bo';  t=1705807800; s='Add logo asset';
       f=@('- - assets/nebula-logo.png','5 0 README.md') }
)

$sb = New-Object System.Text.StringBuilder
foreach ($c in $commits) {
    $auth = $authors[$c.a]
    [void]$sb.Append($SEP)
    [void]$sb.Append($c.h + $FS + $auth[0] + $FS + $auth[1] + $FS + $c.t + $FS + $c.s + "`n")
    foreach ($row in $c.f) {
        # Convert the space-delimited spec into tab-delimited numstat.
        $parts = $row -split ' ', 3
        [void]$sb.Append($parts[0] + "`t" + $parts[1] + "`t" + $parts[2] + "`n")
    }
}

$outDir = Join-Path $PSScriptRoot '..' | Join-Path -ChildPath 'fixtures'
New-Item -ItemType Directory -Force -Path $outDir | Out-Null
$outFile = Join-Path $outDir 'nebula.gitlog'

# Write with UTF-8 (no BOM) so byte content is exactly as intended.
$utf8 = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText($outFile, $sb.ToString(), $utf8)
Write-Host "Wrote $outFile ($($commits.Count) commits)"

// draft note 31
