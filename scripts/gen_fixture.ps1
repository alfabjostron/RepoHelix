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
