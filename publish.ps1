# Install Rust
$cargoPath = "";
if ($env:AGENT_OS -eq "Windows_NT") {
    Invoke-WebRequest https://win.rustup.rs -OutFile ./rustup-init.exe;
    & ./rustup-init.exe -y --default-toolchain stable;
    $cargoPath = "~/.cargo/bin";    
}
else {
    (Invoke-WebRequest https://sh.rustup.rs).Content | & sh -s -- -y --default-toolchain stable    
    $cargoPath = "~/.cargo/bin";
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
& $cargoPath/cargo login $env:CargoApiKey;
& $cargoPath/cargo publish --allow-dirty;

# Publish FFI project
# TODO: Either split this out into a separate release task, or add a long deloy, so
# Crates.io has a chance to catch up. Otherwise scannit-core-ffi on crates.io will
# always be a version behind latest.
Set-Location ./scannit-core-ffi;
& $cargoPath/cargo publish --allow-dirty;
