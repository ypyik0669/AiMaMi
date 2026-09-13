$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot
$previousHome = $env:CODEX_HOME
$previousWebView = $env:WEBVIEW2_USER_DATA_FOLDER
$executable = Join-Path $repoRoot "src-tauri\target\debug\AiMaMi.exe"
$runningTest = Get-Process -Name AiMaMi -ErrorAction SilentlyContinue |
    Where-Object { $_.Path -eq $executable }
if ($runningTest) {
    throw "Exit the previous debug/test AiMaMi from its tray before rebuilding. Leave the installed AiMaMi running."
}

Push-Location $repoRoot
try {
    & corepack pnpm tauri build --debug --no-bundle --config src-tauri/tauri.smoke.conf.json -- --locked
    if ($LASTEXITCODE -ne 0) { throw "Isolated test build failed." }

    $testRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("aimami-smoke-" + [guid]::NewGuid().ToString("N"))
    $env:CODEX_HOME = Join-Path $testRoot "codex-home"
    $env:WEBVIEW2_USER_DATA_FOLDER = Join-Path $testRoot "webview"
    New-Item -ItemType Directory -Path $env:CODEX_HOME -Force | Out-Null

    $process = Start-Process -FilePath $executable -WorkingDirectory $repoRoot -WindowStyle Hidden -PassThru `
        -RedirectStandardOutput (Join-Path $testRoot "stdout.log") `
        -RedirectStandardError (Join-Path $testRoot "stderr.log")
    Start-Sleep -Seconds 5
    $process.Refresh()
    if ($process.HasExited) {
        Get-Content -LiteralPath (Join-Path $testRoot "stderr.log")
        throw "Test process exited. Check the log, or close an existing isolated test from its tray menu."
    }
    Write-Output "Isolated test PID: $($process.Id)"
    Write-Output "Test data and logs: $testRoot"
    Write-Output "Existing AiMaMi and Codex data were not replaced. Exit only the isolated test from its tray menu."
}
finally {
    $env:CODEX_HOME = $previousHome
    $env:WEBVIEW2_USER_DATA_FOLDER = $previousWebView
    Pop-Location
}
