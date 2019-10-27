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

# Rewrite version for main project
$rewrittenMainToml = (Get-Content ./Cargo.toml) -replace $regex, "version = `"$env:BUILD_BUILDNUMBER";
Set-Content /.Cargo.toml $rewrittenMainToml;
Write-Host "Rewrote version for main Cargo.toml.";
Write-Host "New content is: $($rewrittenMainToml)";

# Rewrite version for FFI project
$rewrittenFfiToml = (Get-Content ./scannit-core-ffi/Cargo.toml) -replace $regex, "version = `"$env:BUILD_BUILDNUMBER";
Set-Content ./scannit-core-ffi/Cargo.toml $rewrittenFfiToml;
Write-Host "Rewrote version for FFI Cargo.toml.";
Write-Host "New content is: $($rewrittenFfiToml)";

# CARGO_API_KEY is a secret env var, and should be replaced by Azure DevOps.
# Publish main project
& $cargoPath/cargo login $env:CargoApiKey;
& $cargoPath/cargo publish;

# Publish FFI project
Set-Location ./scannit-core-ffi;
& $cargoPath/cargo publish --allow-dirty;
