$ErrorActionPreference = "Stop"

function Run-Step {
    param (
        [string]$Title,
        [scriptblock]$Command
    )
    Write-Host "`n==========================================" -ForegroundColor Cyan
    Write-Host ">> $Title" -ForegroundColor Cyan
    Write-Host "==========================================" -ForegroundColor Cyan

    & $Command

    if ($LASTEXITCODE -ne 0) {
        Write-Host "`n[FAILED] Error occurred during '$Title'. Stopping script." -ForegroundColor Red
        exit $LASTEXITCODE
    }
}

Run-Step "1. Formatter Check (cargo fmt)" {
    cargo fmt --all
}

Run-Step "2. Workspace Compile Check (cargo check)" {
    cargo check --workspace --all-targets
}

Run-Step "3. Clippy Lint Check (rpe-core)" {
    cargo clippy -p rpe-core --all-targets -- -D warnings
}

Run-Step "4. Core Unit & Integration Tests (rpe-core)" {
    cargo test -p rpe-core
}

Write-Host "`n[SUCCESS] All checks passed!" -ForegroundColor Green
