# Program Upgrade System - Local CI Runner (PowerShell)
# Similar pattern to GDX component CI runners
# Run from PowerShell in the program-upgrade-system directory

param(
    [switch]$Verbose = $false
)

$ErrorActionPreference = "Stop"

Write-Host "🚀 Running Program Upgrade System CI pipeline (via act)" -ForegroundColor Blue
Write-Host ""

# Check if act is installed
if (-not (Get-Command act -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Error: nektos-act is not installed" -ForegroundColor Red
    Write-Host ""
    Write-Host "Install it with:"
    Write-Host "  choco install act-cli"
    Write-Host ""
    Write-Host "Or download from: https://github.com/nektos/act/releases"
    exit 1
}

# Check if Docker is running
try {
    docker info 2>$null | Out-Null
} catch {
    Write-Host "❌ Error: Docker is not running" -ForegroundColor Red
    Write-Host "Please start Docker and try again"
    exit 1
}

Write-Host "Working directory: $(Get-Location)" -ForegroundColor Green
Write-Host ""

Write-Host "▶️  Starting CI pipeline..." -ForegroundColor Green
Write-Host ""
Write-Host "Note: The solana-test-validator will run in the background" -ForegroundColor Yellow
Write-Host "      and persist across workflow steps, just like on GitHub Actions."
Write-Host ""

# Create cache directory
$CacheDir = "$env:USERPROFILE\.cache\program-upgrade-ci"
if (-not (Test-Path $CacheDir)) {
    New-Item -ItemType Directory -Path $CacheDir -Force | Out-Null
}

# Build act arguments
$actArgs = @(
    "--workflows", ".github\workflows\ci.yml",
    "--job", "build-and-test",
    "--container-architecture", "linux/amd64",
    "--container-options", "--network=host --dns=8.8.8.8 --dns=1.1.1.1",
    "--bind",
    "--pull=false",
    "--env", "CARGO_NET_GIT_FETCH_WITH_CLI=true"
)

if ($Verbose) {
    $actArgs += "--verbose"
}

Write-Host "Running: act $($actArgs -join ' ')" -ForegroundColor Cyan
Write-Host ""

# Run act
& act @actArgs

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ CI pipeline completed successfully!" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "❌ CI pipeline failed with exit code: $LASTEXITCODE" -ForegroundColor Red
    exit $LASTEXITCODE
}
