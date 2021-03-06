# Install Rust
if ($env:AGENT_OS -ne "Windows_NT") {
    (Invoke-WebRequest https://sh.rustup.rs).Content | & sh -s -- -y --default-toolchain stable    
}

$regex = "version = `"\d.\d.\d";
$ffiDependencyRegex = "scannit-core = {path = `"../`"";

# Rewrite version for main project
$rewrittenMainToml = (Get-Content ./Cargo.toml) -replace $regex, "version = `"$env:BUILD_BUILDNUMBER";
Set-Content ./Cargo.toml $rewrittenMainToml;
Write-Host "Rewrote version for main Cargo.toml.";

# Rewrite version for FFI project
$rewrittenFfiToml = (Get-Content ./scannit-core-ffi/Cargo.toml) -replace $regex, "version = `"$env:BUILD_BUILDNUMBER";

# Rewrite the version of scannit-core that scannit-core-ffi depends on, because locally it uses path = "../", but
# the crates.io version needs a version number to pin to.
$rewrittenFfiToml = $rewrittenFfiToml -replace $ffiDependencyRegex, "scannit-core = {path = `"../`", version = `"1.0.*`""
Set-Content ./scannit-core-ffi/Cargo.toml $rewrittenFfiToml;
Write-Host "Rewrote version for FFI Cargo.toml.";

# CARGO_API_KEY is a secret env var, and should be replaced by Azure DevOps.
# Publish main project
& cargo login $env:CargoApiKey;
& cargo publish --allow-dirty;

# TODO: This is dumb. Without the delay, attempting to publish the FFI project will always grab the previous
# version, because crates.io is still publishing the newly-uploaded version.
# This should just be a separate release stage that depends on this one.
$sleepSeconds = 30;
Write-Host "Sleeping $sleepSeconds seconds to allow crates.io time to publish scannit-core...";
Start-Sleep 30;

# Publish FFI project
Set-Location ./scannit-core-ffi;
& cargo publish --allow-dirty;
