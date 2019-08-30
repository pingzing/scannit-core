# Install Rust
$cargoPath = "";
if ($env:AGENT_OS -eq "Windows_NT") {
    curl -sSf -o rustup-init.exe https://win.rustup.rs;
    rustup-init.exe -y --default-toolchain stable;
    $cargoPath = "~/.cargo/bin";    
}
else {
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable    
    $cargoPath = "~/.cargo/bin";
}

$regex = "version = `"(\d.\d).\d";

# Rewrite version for main project
$rewrittenMainToml = (Get-Content ./Cargo.toml) -replace $regex, "`$1.$env:BUILD_BUILDNUMBER";
Set-Content /.Cargo.toml $rewrittenMainToml;
Write-Host "Rewrote version for main Cargo.toml.";

# Rewrite version for FFI project
$rewrittenFfiToml = (Get-Content ./scannit-core-ffi/Cargo.toml) -replace $regex, "`$1.$env:BUILD_BUILDNUMBER";
Set-Content ./scannit-core-ffi/Cargo.toml $rewrittenFfiToml;
Write-Host "Rewrote version for FFI Cargo.toml.";

# CARGO_API_KEY is a secret env var, and should be replaced by Azure DevOps.
# Publish main project
& $cargoPath/cargo cargo login $(CARGO_API_KEY);
& $cargoPath/cargo publish;

# Publish FFI project
Set-Location ./scannit-core-ffi;
& $cargoPath/cargo publish;
